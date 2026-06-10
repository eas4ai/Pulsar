//! User role promotion command.

use async_trait::async_trait;
use suprnova::rbac::{create_permission, create_role, give_permission_to_role};
use suprnova::{Command, FrameworkError, HasRoles, TypedCommand};

use crate::models::user::User;

const ADMIN_ROLE: &str = "admin";
const AUTHOR_ROLE: &str = "author";
const MEMBER_ROLE: &str = "member";

const ARTICLE_PERMISSIONS: [&str; 4] = [
    "articles.create",
    "articles.update",
    "articles.publish",
    "articles.delete",
];

const AUTHOR_PERMISSIONS: [&str; 3] = ["articles.create", "articles.update", "articles.publish"];

#[derive(clap::Parser, Command, Debug)]
#[console(
    name = "users:promote",
    description = "Assign a role to a user by email"
)]
pub struct UsersPromoteCommand {
    /// Email address of the user to promote.
    #[arg(long)]
    pub email: String,
    /// Role to assign: admin, author, or member.
    #[arg(long)]
    pub role: String,
}

#[async_trait]
impl TypedCommand for UsersPromoteCommand {
    async fn run(self) -> Result<(), FrameworkError> {
        let role = normalize_role(&self.role)?;
        let user = promote_user(&self.email, &role).await?;
        println!("Promoted {} to {}.", user.email, role);
        Ok(())
    }
}

/// Create Pulsar's default roles and article permissions.
///
/// Safe to run more than once; the framework RBAC helpers skip existing rows.
pub async fn seed_default_roles() -> Result<(), FrameworkError> {
    create_role(ADMIN_ROLE).await?;
    create_role(AUTHOR_ROLE).await?;
    create_role(MEMBER_ROLE).await?;

    for permission in ARTICLE_PERMISSIONS {
        create_permission(permission).await?;
        give_permission_to_role(ADMIN_ROLE, permission).await?;
    }

    for permission in AUTHOR_PERMISSIONS {
        give_permission_to_role(AUTHOR_ROLE, permission).await?;
    }

    Ok(())
}

/// Promote an existing user to one of Pulsar's default roles.
///
/// The role table is seeded before assignment so local development databases
/// can use the command immediately after migrations.
pub async fn promote_user(email: &str, role: &str) -> Result<User, FrameworkError> {
    let role = normalize_role(role)?;
    seed_default_roles().await?;

    let user = User::find_by_email(email)
        .await?
        .ok_or_else(|| FrameworkError::not_found(format!("User `{email}`")))?;

    user.assign_role(&role).await?;
    Ok(user)
}

fn normalize_role(role: &str) -> Result<String, FrameworkError> {
    let normalized = role.trim().to_ascii_lowercase();
    match normalized.as_str() {
        ADMIN_ROLE | AUTHOR_ROLE | MEMBER_ROLE => Ok(normalized),
        _ => Err(FrameworkError::bad_request(format!(
            "Unknown role `{role}`. Expected admin, author, or member."
        ))),
    }
}
