//! User role promotion command.

use async_trait::async_trait;
use suprnova::rbac::{create_permission, create_role, give_permission_to_role};
use suprnova::{Command, FrameworkError, HasRoles, TypedCommand};

use crate::models::user::User;

const ADMIN_ROLE: &str = "admin";
const MODERATOR_ROLE: &str = "moderator";
const AUTHOR_ROLE: &str = "author";
const CONTRIBUTOR_ROLE: &str = "contributor";
const MEMBER_ROLE: &str = "member";

const DEFAULT_ROLES: [&str; 5] = [
    ADMIN_ROLE,
    MODERATOR_ROLE,
    AUTHOR_ROLE,
    CONTRIBUTOR_ROLE,
    MEMBER_ROLE,
];

const V2_PERMISSIONS: [&str; 16] = [
    "articles.create",
    "articles.submit",
    "articles.review",
    "articles.publish",
    "questions.create",
    "answers.accept_own",
    "comments.create",
    "resources.submit",
    "resources.review",
    "resources.publish",
    "moderation.review",
    "moderation.decide",
    "users.manage",
    "roles.manage",
    "taxonomy.manage",
    "settings.manage",
];

// Existing first-party article admin CRUD predates V2 community submissions.
const INTERNAL_ARTICLE_PERMISSIONS: [&str; 1] = ["articles.update"];

const MODERATOR_PERMISSIONS: [&str; 4] = [
    "articles.review",
    "resources.review",
    "moderation.review",
    "moderation.decide",
];

const AUTHOR_PERMISSIONS: [&str; 5] = [
    "articles.create",
    "articles.update",
    "articles.submit",
    "articles.review",
    "articles.publish",
];

const CONTRIBUTOR_PERMISSIONS: [&str; 4] = [
    "articles.submit",
    "questions.create",
    "comments.create",
    "resources.submit",
];

const MEMBER_PERMISSIONS: [&str; 4] = [
    "questions.create",
    "answers.accept_own",
    "comments.create",
    "resources.submit",
];

#[derive(clap::Parser, Command, Debug)]
#[console(
    name = "users:promote",
    description = "Assign a role to a user by email"
)]
pub struct UsersPromoteCommand {
    /// Email address of the user to promote.
    #[arg(long)]
    pub email: String,
    /// Role to assign: admin, moderator, author, contributor, or member.
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

/// Create Pulsar's default roles and V2 permissions.
///
/// Safe to run more than once; the framework RBAC helpers skip existing rows.
pub async fn seed_default_roles() -> Result<(), FrameworkError> {
    for role in DEFAULT_ROLES {
        create_role(role).await?;
    }

    for permission in V2_PERMISSIONS {
        create_permission(permission).await?;
        give_permission_to_role(ADMIN_ROLE, permission).await?;
    }

    for permission in INTERNAL_ARTICLE_PERMISSIONS {
        create_permission(permission).await?;
        give_permission_to_role(ADMIN_ROLE, permission).await?;
    }

    grant_permissions(MODERATOR_ROLE, &MODERATOR_PERMISSIONS).await?;
    grant_permissions(AUTHOR_ROLE, &AUTHOR_PERMISSIONS).await?;
    grant_permissions(CONTRIBUTOR_ROLE, &CONTRIBUTOR_PERMISSIONS).await?;
    grant_permissions(MEMBER_ROLE, &MEMBER_PERMISSIONS).await?;

    Ok(())
}

async fn grant_permissions(role: &str, permissions: &[&str]) -> Result<(), FrameworkError> {
    for &permission in permissions {
        give_permission_to_role(role, permission).await?;
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
    if DEFAULT_ROLES.contains(&normalized.as_str()) {
        Ok(normalized)
    } else {
        Err(FrameworkError::bad_request(format!(
            "Unknown role `{role}`. Expected admin, moderator, author, contributor, or member."
        )))
    }
}
