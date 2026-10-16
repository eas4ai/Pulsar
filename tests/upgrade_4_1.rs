//! Consumer-side checks for the persisted-state migrations required by Suprnova 4.x.

use pulsar::migrations::Migrator;
use sea_orm::{ConnectionTrait, Database, DbBackend, Statement};
use sea_orm_migration::MigratorTrait;

#[tokio::test]
async fn legacy_sessions_are_moved_to_the_laravel_layout_with_their_rows() {
    let database = Database::connect("sqlite::memory:")
        .await
        .expect("connect migration database");

    Migrator::up(&database, Some(2))
        .await
        .expect("create the pre-4.0 users and sessions tables");
    database
        .execute_unprepared(
            "INSERT INTO sessions (id, user_id, payload, csrf_token, last_activity) \
             VALUES ('legacy-session', NULL, '{\"cart\":\"saved\"}', 'legacy-csrf', \
             '2026-10-01 12:00:00')",
        )
        .await
        .expect("insert a pre-4.0 session");

    Migrator::up(&database, None)
        .await
        .expect("run the complete migration chain");

    let columns = database
        .query_all_raw(Statement::from_string(
            DbBackend::Sqlite,
            "PRAGMA table_info('sessions')",
        ))
        .await
        .expect("inspect upgraded sessions table")
        .into_iter()
        .map(|row| row.try_get::<String>("", "name").expect("column name"))
        .collect::<Vec<_>>();
    assert_eq!(
        columns,
        [
            "id",
            "user_id",
            "ip_address",
            "user_agent",
            "payload",
            "last_activity",
        ]
    );

    let session = database
        .query_one_raw(Statement::from_string(
            DbBackend::Sqlite,
            "SELECT id, payload FROM sessions WHERE id = 'legacy-session'",
        ))
        .await
        .expect("read upgraded session")
        .expect("legacy session is preserved");
    // The Laravel payload is base64 JSON containing both the legacy CSRF
    // token and application data; pin it so a lossy conversion cannot pass.
    assert_eq!(
        (
            session.try_get::<String>("", "id").expect("session id"),
            session
                .try_get::<String>("", "payload")
                .expect("session payload"),
        ),
        (
            "legacy-session".to_owned(),
            "eyJfc3Vwcm5vdmEiOjEsIl90b2tlbiI6ImxlZ2FjeS1jc3JmIiwiY2FydCI6InNhdmVkIn0=".to_owned(),
        )
    );
}

#[tokio::test]
async fn legacy_roles_are_moved_to_the_spatie_layout_with_their_details() {
    let database = Database::connect("sqlite::memory:")
        .await
        .expect("connect migration database");
    database
        .execute_unprepared(
            "CREATE TABLE roles (\
                id INTEGER PRIMARY KEY AUTOINCREMENT NOT NULL, \
                name TEXT NOT NULL, \
                display_name TEXT NULL, \
                guard_name TEXT NOT NULL DEFAULT 'web', \
                created_at TEXT NOT NULL DEFAULT CURRENT_TIMESTAMP, \
                updated_at TEXT NOT NULL DEFAULT CURRENT_TIMESTAMP\
            )",
        )
        .await
        .expect("create the pre-4.0 roles table");
    database
        .execute_unprepared(
            "INSERT INTO roles (id, name, display_name, guard_name) \
             VALUES (7, 'author', 'Article author', 'web')",
        )
        .await
        .expect("insert a pre-4.0 role");

    Migrator::up(&database, None)
        .await
        .expect("run the complete migration chain");

    let role = database
        .query_one_raw(Statement::from_string(
            DbBackend::Sqlite,
            "SELECT id, name, guard_name FROM roles WHERE id = 7",
        ))
        .await
        .expect("read upgraded role")
        .expect("legacy role is preserved");
    assert_eq!(role.try_get::<String>("", "name").unwrap(), "author");
    assert_eq!(role.try_get::<String>("", "guard_name").unwrap(), "web");

    let details = database
        .query_one_raw(Statement::from_string(
            DbBackend::Sqlite,
            "SELECT display_name FROM suprnova_role_details WHERE role_id = 7",
        ))
        .await
        .expect("read migrated role details")
        .expect("legacy role details are preserved");
    assert_eq!(
        details.try_get::<String>("", "display_name").unwrap(),
        "Article author"
    );
}
