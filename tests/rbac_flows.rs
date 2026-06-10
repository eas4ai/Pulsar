//! RBAC flow tests for Pulsar user promotion.

mod common;

use pulsar::commands::users_promote::{promote_user, seed_default_roles};
use pulsar::models::user::User;
use suprnova::HasRoles;

#[tokio::test]
async fn rbac_author_role_has_v2_article_authoring_permissions() {
    let _harness = common::setup().await;
    let user = User::create("Octavia Butler", "octavia@pulsar.test", "secretpass")
        .await
        .expect("create user");

    let promoted = promote_user("octavia@pulsar.test", "author")
        .await
        .expect("promote user");

    assert_eq!(promoted.id, user.id);
    assert!(promoted.has_role("author").await.unwrap());
    assert!(promoted.has_permission_to("articles.create").await.unwrap());
    assert!(promoted.has_permission_to("articles.update").await.unwrap());
    assert!(promoted.has_permission_to("articles.submit").await.unwrap());
    assert!(promoted.has_permission_to("articles.review").await.unwrap());
    assert!(
        promoted
            .has_permission_to("articles.publish")
            .await
            .unwrap()
    );
    assert!(
        !promoted
            .has_permission_to("resources.review")
            .await
            .unwrap()
    );
    assert!(!promoted.has_permission_to("users.manage").await.unwrap());
}

#[tokio::test]
async fn rbac_moderator_role_has_v2_review_and_moderation_permissions() {
    let _harness = common::setup().await;
    User::create("Nora Jemisin", "nora@pulsar.test", "secretpass")
        .await
        .expect("create user");

    let promoted = promote_user("nora@pulsar.test", "moderator")
        .await
        .expect("promote user");

    assert!(promoted.has_role("moderator").await.unwrap());
    assert!(promoted.has_permission_to("articles.review").await.unwrap());
    assert!(
        promoted
            .has_permission_to("resources.review")
            .await
            .unwrap()
    );
    assert!(
        promoted
            .has_permission_to("moderation.review")
            .await
            .unwrap()
    );
    assert!(
        promoted
            .has_permission_to("moderation.decide")
            .await
            .unwrap()
    );
    assert!(
        !promoted
            .has_permission_to("articles.publish")
            .await
            .unwrap()
    );
    assert!(!promoted.has_permission_to("articles.update").await.unwrap());
    assert!(!promoted.has_permission_to("users.manage").await.unwrap());
}

#[tokio::test]
async fn rbac_contributor_role_has_v2_submission_permissions() {
    let _harness = common::setup().await;
    User::create("Ada Palmer", "ada@pulsar.test", "secretpass")
        .await
        .expect("create user");

    let promoted = promote_user("ada@pulsar.test", "contributor")
        .await
        .expect("promote user");

    assert!(promoted.has_role("contributor").await.unwrap());
    assert!(promoted.has_permission_to("articles.submit").await.unwrap());
    assert!(
        promoted
            .has_permission_to("resources.submit")
            .await
            .unwrap()
    );
    assert!(
        promoted
            .has_permission_to("questions.create")
            .await
            .unwrap()
    );
    assert!(promoted.has_permission_to("comments.create").await.unwrap());
    assert!(!promoted.has_permission_to("articles.create").await.unwrap());
    assert!(
        !promoted
            .has_permission_to("articles.publish")
            .await
            .unwrap()
    );
    assert!(
        !promoted
            .has_permission_to("moderation.review")
            .await
            .unwrap()
    );
    assert!(!promoted.has_permission_to("articles.update").await.unwrap());
}

#[tokio::test]
async fn rbac_member_role_has_v2_member_permissions() {
    let _harness = common::setup().await;
    User::create("Nnedi Okorafor", "nnedi@pulsar.test", "secretpass")
        .await
        .expect("create user");

    let promoted = promote_user("nnedi@pulsar.test", "member")
        .await
        .expect("promote user");

    assert!(promoted.has_role("member").await.unwrap());
    assert!(
        promoted
            .has_permission_to("questions.create")
            .await
            .unwrap()
    );
    assert!(
        promoted
            .has_permission_to("answers.accept_own")
            .await
            .unwrap()
    );
    assert!(promoted.has_permission_to("comments.create").await.unwrap());
    assert!(
        promoted
            .has_permission_to("resources.submit")
            .await
            .unwrap()
    );
    assert!(!promoted.has_permission_to("articles.create").await.unwrap());
    assert!(
        !promoted
            .has_permission_to("articles.publish")
            .await
            .unwrap()
    );
    assert!(!promoted.has_permission_to("taxonomy.manage").await.unwrap());
}

#[tokio::test]
async fn rbac_console_command_promotes_existing_user() {
    let _harness = common::setup().await;
    User::create("Ursula Le Guin", "ursula@pulsar.test", "secretpass")
        .await
        .expect("create user");

    suprnova::console::dispatch_argv(vec![
        "console".to_string(),
        "users:promote".to_string(),
        "--email".to_string(),
        "ursula@pulsar.test".to_string(),
        "--role".to_string(),
        "admin".to_string(),
    ])
    .await
    .expect("dispatch users:promote");

    let user = User::find_by_email("ursula@pulsar.test")
        .await
        .expect("lookup")
        .expect("user exists");
    assert!(user.has_role("admin").await.unwrap());
    for permission in [
        "articles.create",
        "articles.update",
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
    ] {
        assert!(
            user.has_permission_to(permission).await.unwrap(),
            "admin should have {permission}"
        );
    }
}

#[tokio::test]
async fn rbac_seed_default_roles_is_idempotent() {
    let _harness = common::setup().await;

    seed_default_roles().await.expect("first seed");
    seed_default_roles().await.expect("second seed");
}
