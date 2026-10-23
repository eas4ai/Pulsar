pub use sea_orm_migration::prelude::*;
use suprnova::rbac::migrations::{CreateRbacTables, RbacToSpatieLayout};
use suprnova::session::migrations::{CreateSessionsTable, SessionUserKey};

mod m20240101_000001_create_users_table;
mod m20240101_000002_create_sessions_table;
mod m20240101_000003_create_remember_tokens_table;
mod m20240101_000004_add_email_verified_at_to_users;
mod m20240101_000005_create_auth_flow_tokens;
mod m20260110_000006_create_articles;
mod m20260610_000007_create_v2_foundations;

pub struct Migrator;

#[async_trait::async_trait]
impl MigratorTrait for Migrator {
    fn migrations() -> Vec<Box<dyn MigrationTrait>> {
        let mut migrations: Vec<Box<dyn MigrationTrait>> = vec![
            Box::new(m20240101_000001_create_users_table::Migration),
            Box::new(m20240101_000002_create_sessions_table::Migration),
            Box::new(m20240101_000003_create_remember_tokens_table::Migration),
            Box::new(m20240101_000004_add_email_verified_at_to_users::Migration),
            Box::new(m20240101_000005_create_auth_flow_tokens::Migration),
            Box::new(CreateRbacTables),
            // Suprnova 4 reads the Laravel-compatible Spatie layout. This
            // migration preserves the rows created by older Pulsar releases.
            Box::new(RbacToSpatieLayout),
            Box::new(m20260110_000006_create_articles::Migration),
            Box::new(m20260610_000007_create_v2_foundations::Migration),
            // Pulsar's original session migration predates Suprnova 4. Move
            // that table into the Laravel layout after all application
            // migrations, preserving active session rows.
            Box::new(CreateSessionsTable::new(SessionUserKey::Integer)),
            Box::new(suprnova::auth::migrations::UniqueEmailIgnoringCase::on(
                "app_users",
                "email",
            )),
            Box::new(suprnova::render_cache::migration::Migration),
        ];
        migrations.extend(suprnova::agents::migrations());
        migrations
    }
}
