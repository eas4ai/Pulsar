//! Upgrade an existing Pulsar database without losing account records.

use pulsar::migrations::Migrator;
use sea_orm::{ConnectionTrait, Database, DbBackend, Statement};
use sea_orm_migration::MigratorTrait;

#[tokio::test]
async fn upgraded_email_index_preserves_accounts_and_refuses_case_duplicates() {
    let db = Database::connect("sqlite::memory:").await.unwrap();
    // The ten migrations shipped by Pulsar 2.1.1.
    Migrator::up(&db, Some(10)).await.unwrap();
    db.execute_unprepared("INSERT INTO app_users (email) VALUES ('Existing@Example.Test')")
        .await
        .unwrap();
    Migrator::up(&db, None).await.unwrap();
    let row = db
        .query_one_raw(Statement::from_string(
            DbBackend::Sqlite,
            "SELECT email FROM app_users".to_owned(),
        ))
        .await
        .unwrap()
        .unwrap();
    assert_eq!(
        row.try_get::<String>("", "email").unwrap(),
        "Existing@Example.Test"
    );
    assert!(
        db.execute_unprepared("INSERT INTO app_users (email) VALUES ('existing@example.test')")
            .await
            .is_err()
    );
    // Re-running migrations is safe after the first deployment.
    Migrator::up(&db, None).await.unwrap();
}

#[tokio::test]
async fn email_index_refuses_ambiguous_existing_accounts_without_deleting_them() {
    let db = Database::connect("sqlite::memory:").await.unwrap();
    Migrator::up(&db, Some(10)).await.unwrap();
    db.execute_unprepared(
        "INSERT INTO app_users (email) VALUES ('Existing@Example.Test'), ('existing@example.test')",
    )
    .await
    .unwrap();
    let error = Migrator::up(&db, None)
        .await
        .expect_err("case duplicates need an operator decision");
    assert!(error.to_string().contains("email"), "{error}");
    let row = db
        .query_one_raw(Statement::from_string(
            DbBackend::Sqlite,
            "SELECT COUNT(*) AS count FROM app_users".to_owned(),
        ))
        .await
        .unwrap()
        .unwrap();
    assert_eq!(row.try_get::<i64>("", "count").unwrap(), 2);
}
