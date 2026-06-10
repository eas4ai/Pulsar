//! V2 foundation flow tests.

mod common;

mod v2_foundation_flows {
    use chrono::Utc;
    use serde_json::json;

    use suprnova::attrs;
    use suprnova::eloquent::Model;
    use suprnova::mail::Mail;
    use suprnova::sea_orm::{DatabaseBackend, Statement};
    use suprnova::{ConnectionTrait, DB, HasRoles, MustVerifyEmail};

    use super::common::{Client, setup};
    use pulsar::commands::users_promote::seed_default_roles;
    use pulsar::models::badge::Badge;
    use pulsar::models::category::Category;
    use pulsar::models::profile::Profile;
    use pulsar::models::reputation_event::ReputationEvent;
    use pulsar::models::tag::Tag;
    use pulsar::models::topic::Topic;
    use pulsar::models::user::User;

    async fn verified_user(name: &str, email: &str) -> User {
        let mut user = User::create(name, email, "secretpass")
            .await
            .expect("create user");
        user.email_verified_at = Some(Utc::now());
        Model::save(&user).await.expect("verify user");
        user
    }

    async fn login(client: &mut Client, email: &str) {
        let page = client.get("/login").await;
        assert_eq!(page.status, 200, "GET /login should set CSRF cookie");

        let resp = client
            .post_json(
                "/login",
                json!({
                    "email": email,
                    "password": "secretpass",
                    "remember": false,
                }),
            )
            .await;
        assert_eq!(resp.status, 302, "login should redirect: {}", resp.body);
        assert_eq!(resp.location(), "/dashboard");
    }

    async fn public_profile(user: &User, handle: &str, display_name: &str) -> Profile {
        let mut profile = Profile::ensure_for_user(user)
            .await
            .expect("ensure profile for public profile");
        profile.handle = handle.to_string();
        profile.display_name = display_name.to_string();
        Model::save(&profile).await.expect("save public profile");
        profile
    }

    #[tokio::test]
    async fn profile_is_created_for_registered_users() {
        let mut harness = setup().await;
        let addr = harness.spawn_app().await;
        let mut client = Client::new(addr);

        let resp = client.get("/register").await;
        assert_eq!(resp.status, 200, "GET /register must render: {}", resp.body);

        let _mail = Mail::fake();
        let resp = client
            .post_json(
                "/register",
                json!({
                    "name": "Katherine Johnson",
                    "email": "katherine@pulsar.test",
                    "password": "supersecret",
                    "password_confirmation": "supersecret",
                }),
            )
            .await;
        assert_eq!(resp.status, 302, "register must redirect: {}", resp.body);
        assert_eq!(resp.location(), "/dashboard");

        let user = User::find_by_email("katherine@pulsar.test")
            .await
            .expect("lookup registered user")
            .expect("registered user exists");
        assert_eq!(user.name, "Katherine Johnson");

        let profile = Profile::find_by_user_id(user.id)
            .await
            .expect("lookup profile by user id")
            .expect("registered user has a profile");
        assert_eq!(profile.user_id, user.id);
        assert_eq!(profile.display_name, user.name);
    }

    #[tokio::test]
    async fn registration_rejects_names_longer_than_profile_display_name() {
        let mut harness = setup().await;
        let addr = harness.spawn_app().await;
        let mut client = Client::new(addr);
        let email = "too-long-name@pulsar.test";
        let long_name = "A".repeat(121);

        let resp = client.get("/register").await;
        assert_eq!(resp.status, 200, "GET /register must render: {}", resp.body);

        let _mail = Mail::fake();
        let resp = client
            .post_json(
                "/register",
                json!({
                    "name": long_name,
                    "email": email,
                    "password": "supersecret",
                    "password_confirmation": "supersecret",
                }),
            )
            .await;

        assert_eq!(
            resp.status, 422,
            "too-long name should be rejected before user creation: {}",
            resp.body
        );
        assert!(
            resp.body.contains("Name must be at most 120 characters"),
            "validation response should explain the name limit: {}",
            resp.body
        );
        assert!(
            User::find_by_email(email)
                .await
                .expect("lookup rejected user")
                .is_none(),
            "a rejected registration must not create the user"
        );
    }

    #[tokio::test]
    async fn account_delete_removes_registered_users_profile() {
        let mut harness = setup().await;
        let addr = harness.spawn_app().await;
        let mut client = Client::new(addr);
        let email = "delete-profile@pulsar.test";

        let resp = client.get("/register").await;
        assert_eq!(resp.status, 200, "GET /register must render: {}", resp.body);

        let _mail = Mail::fake();
        let resp = client
            .post_json(
                "/register",
                json!({
                    "name": "Delete Profile",
                    "email": email,
                    "password": "supersecret",
                    "password_confirmation": "supersecret",
                }),
            )
            .await;
        assert_eq!(resp.status, 302, "register must redirect: {}", resp.body);

        let user = User::find_by_email(email)
            .await
            .expect("lookup registered user")
            .expect("registered user exists");
        let user_id = user.id;
        assert!(
            Profile::find_by_user_id(user_id)
                .await
                .expect("lookup created profile")
                .is_some(),
            "registration creates a profile before deletion"
        );

        let resp = client
            .delete_json("/profile", json!({ "password": "supersecret" }))
            .await;
        assert_eq!(resp.status, 302, "delete account redirects: {}", resp.body);
        assert_eq!(resp.location(), "/");
        assert!(
            User::find_by_email(email)
                .await
                .expect("lookup deleted user")
                .is_none(),
            "account deletion removes the user"
        );
        assert!(
            Profile::find_by_user_id(user_id)
                .await
                .expect("lookup deleted user's profile")
                .is_none(),
            "account deletion removes the profile"
        );
    }

    #[tokio::test]
    async fn failed_profile_creation_does_not_leave_registered_user() {
        let mut harness = setup().await;
        let addr = harness.spawn_app().await;
        let mut client = Client::new(addr);
        let email = "profile-failure@pulsar.test";

        <Profile as Model>::create(attrs! {
            user_id: 999_i64,
            handle: "user-1",
            display_name: "Handle Collision",
        })
        .await
        .expect("create profile handle collision");

        let resp = client.get("/register").await;
        assert_eq!(resp.status, 200, "GET /register must render: {}", resp.body);

        let _mail = Mail::fake();
        let resp = client
            .post_json(
                "/register",
                json!({
                    "name": "Profile Failure",
                    "email": email,
                    "password": "supersecret",
                    "password_confirmation": "supersecret",
                }),
            )
            .await;

        assert_ne!(
            resp.status, 302,
            "profile creation failure must not complete registration"
        );
        assert!(
            User::find_by_email(email)
                .await
                .expect("lookup compensated user")
                .is_none(),
            "failed profile creation must remove the just-created user"
        );
    }

    #[tokio::test]
    async fn profile_find_by_slug_uses_handle() {
        let _harness = setup().await;
        let user = User::create("Slug User", "profile-slug@pulsar.test", "supersecret")
            .await
            .expect("create user");
        let profile = Profile::ensure_for_user(&user)
            .await
            .expect("ensure profile");

        let found = Profile::find_by_slug(&profile.handle)
            .await
            .expect("lookup profile by slug")
            .expect("profile slug resolves by handle");

        assert_eq!(found.id, profile.id);
    }

    #[tokio::test]
    async fn profile_update_requires_handle() {
        let mut harness = setup().await;
        let email = "handle-required@pulsar.test";
        let user = verified_user("Handle Required", email).await;
        Profile::ensure_for_user(&user)
            .await
            .expect("ensure current profile");
        let addr = harness.spawn_app().await;
        let mut client = Client::new(addr);
        login(&mut client, email).await;

        let resp = client
            .patch_json(
                "/profile",
                json!({
                    "name": "Handle Required",
                    "email": email,
                    "display_name": "Handle Required",
                    "handle": "",
                }),
            )
            .await;

        assert_eq!(resp.status, 422, "blank handle should be rejected");
        assert!(
            resp.body.contains("Handle is required."),
            "validation response should mention required handle: {}",
            resp.body
        );
    }

    #[tokio::test]
    async fn profile_update_requires_slug_safe_handle() {
        let mut harness = setup().await;
        let email = "handle-slug@pulsar.test";
        let user = verified_user("Handle Slug", email).await;
        Profile::ensure_for_user(&user)
            .await
            .expect("ensure current profile");
        let addr = harness.spawn_app().await;
        let mut client = Client::new(addr);
        login(&mut client, email).await;

        let resp = client
            .patch_json(
                "/profile",
                json!({
                    "name": "Handle Slug",
                    "email": email,
                    "display_name": "Handle Slug",
                    "handle": "Bad Handle!",
                }),
            )
            .await;

        assert_eq!(resp.status, 422, "slug-unsafe handle should be rejected");
        assert!(
            resp.body
                .contains("Handle may only contain lowercase letters, numbers, and hyphens."),
            "validation response should explain slug-safe handles: {}",
            resp.body
        );
    }

    #[tokio::test]
    async fn profile_update_requires_unique_handle() {
        let mut harness = setup().await;
        let existing = verified_user("Existing Handle", "existing-handle@pulsar.test").await;
        public_profile(&existing, "taken-handle", "Existing Handle").await;

        let email = "new-handle-owner@pulsar.test";
        let user = verified_user("New Handle Owner", email).await;
        Profile::ensure_for_user(&user)
            .await
            .expect("ensure current profile");
        let addr = harness.spawn_app().await;
        let mut client = Client::new(addr);
        login(&mut client, email).await;

        let resp = client
            .patch_json(
                "/profile",
                json!({
                    "name": "New Handle Owner",
                    "email": email,
                    "display_name": "New Handle Owner",
                    "handle": "taken-handle",
                }),
            )
            .await;

        assert_eq!(resp.status, 422, "duplicate handle should be rejected");
        assert!(
            resp.body.contains("This handle is already taken."),
            "validation response should explain handle uniqueness: {}",
            resp.body
        );
    }

    #[tokio::test]
    async fn profile_update_rejects_unsafe_public_urls() {
        let mut harness = setup().await;
        let email = "unsafe-urls@pulsar.test";
        let user = verified_user("Unsafe URLs", email).await;
        Profile::ensure_for_user(&user)
            .await
            .expect("ensure current profile");
        let addr = harness.spawn_app().await;
        let mut client = Client::new(addr);
        login(&mut client, email).await;

        let resp = client
            .patch_json(
                "/profile",
                json!({
                    "name": "Unsafe URLs",
                    "email": email,
                    "display_name": "Unsafe URLs",
                    "handle": "unsafe-urls",
                    "website_url": "javascript:alert(1)",
                    "github_url": "data:text/html,<h1>x</h1>",
                }),
            )
            .await;

        assert_eq!(resp.status, 422, "unsafe public URLs should be rejected");
        assert!(
            resp.body
                .contains("Website URL must start with http:// or https://.")
                && resp
                    .body
                    .contains("GitHub URL must start with http:// or https://."),
            "validation response should explain public URL requirements: {}",
            resp.body
        );
    }

    #[tokio::test]
    async fn profile_update_rejects_reserved_generated_handles() {
        let mut harness = setup().await;
        let email = "reserved-handle@pulsar.test";
        let user = verified_user("Reserved Handle", email).await;
        Profile::ensure_for_user(&user)
            .await
            .expect("ensure current profile");
        let addr = harness.spawn_app().await;
        let mut client = Client::new(addr);
        login(&mut client, email).await;

        let resp = client
            .patch_json(
                "/profile",
                json!({
                    "name": "Reserved Handle",
                    "email": email,
                    "display_name": "Reserved Handle",
                    "handle": "user-123",
                }),
            )
            .await;

        assert_eq!(
            resp.status, 422,
            "reserved generated handle should be rejected"
        );
        assert!(
            resp.body
                .contains("Handles matching user-{number} are reserved."),
            "validation response should explain reserved handles: {}",
            resp.body
        );
    }

    #[tokio::test]
    async fn profile_update_allows_owner_to_keep_default_generated_handle() {
        let mut harness = setup().await;
        let email = "default-handle@pulsar.test";
        let user = verified_user("Default Handle", email).await;
        let profile = Profile::ensure_for_user(&user)
            .await
            .expect("ensure current profile");
        assert_eq!(profile.handle, format!("user-{}", user.id));
        let addr = harness.spawn_app().await;
        let mut client = Client::new(addr);
        login(&mut client, email).await;

        let resp = client
            .patch_json(
                "/profile",
                json!({
                    "name": "Default Handle",
                    "email": email,
                    "display_name": "Default Handle",
                    "handle": profile.handle,
                    "bio": "Updated while keeping the generated handle.",
                }),
            )
            .await;

        assert_eq!(
            resp.status, 302,
            "owner should be able to keep their generated handle: {}",
            resp.body
        );
        assert_eq!(resp.location(), "/profile");

        let updated = Profile::find_by_user_id(user.id)
            .await
            .expect("reload profile")
            .expect("profile exists");
        assert_eq!(updated.handle, format!("user-{}", user.id));
        assert_eq!(
            updated.bio.as_deref(),
            Some("Updated while keeping the generated handle.")
        );
    }

    #[tokio::test]
    async fn profile_update_rolls_back_user_when_profile_save_fails() {
        let mut harness = setup().await;
        let email = "atomic-profile@pulsar.test";
        let user = verified_user("Atomic Original", email).await;
        Profile::ensure_for_user(&user)
            .await
            .expect("ensure current profile");

        let db = DB::connection().expect("resolve test database connection");
        db.inner()
            .execute(Statement::from_string(
                DatabaseBackend::Sqlite,
                "CREATE TRIGGER fail_profile_update \
                 BEFORE UPDATE ON profiles \
                 BEGIN SELECT RAISE(ABORT, 'forced profile update failure'); END;"
                    .to_string(),
            ))
            .await
            .expect("install failing profile update trigger");

        let addr = harness.spawn_app().await;
        let mut client = Client::new(addr);
        login(&mut client, email).await;

        let resp = client
            .patch_json(
                "/profile",
                json!({
                    "name": "Atomic Changed",
                    "email": "atomic-profile-new@pulsar.test",
                    "display_name": "Atomic Changed",
                    "handle": "atomic-profile",
                    "bio": "This profile save is forced to fail.",
                }),
            )
            .await;

        assert_ne!(
            resp.status, 302,
            "profile failure should not report a successful update"
        );
        assert!(
            User::find_by_email("atomic-profile-new@pulsar.test")
                .await
                .expect("lookup new email")
                .is_none(),
            "failed profile save must not leave the user under the new email"
        );
        let reloaded = User::find_by_email(email)
            .await
            .expect("lookup original email")
            .expect("original user row should remain");
        assert_eq!(reloaded.name, "Atomic Original");
        assert_eq!(reloaded.email, email);
        assert!(
            reloaded.is_email_verified(),
            "failed profile save must not clear email verification"
        );
    }

    #[tokio::test]
    async fn profile_update_rejects_whitespace_required_values() {
        let mut harness = setup().await;
        let email = "whitespace-profile@pulsar.test";
        let user = verified_user("Whitespace Profile", email).await;
        Profile::ensure_for_user(&user)
            .await
            .expect("ensure current profile");
        let addr = harness.spawn_app().await;
        let mut client = Client::new(addr);
        login(&mut client, email).await;

        let resp = client
            .patch_json(
                "/profile",
                json!({
                    "name": "   ",
                    "email": email,
                    "display_name": "   ",
                    "handle": "   ",
                }),
            )
            .await;

        assert_eq!(
            resp.status, 422,
            "whitespace required values should be rejected"
        );
        assert!(
            resp.body.contains("Name is required.")
                && resp.body.contains("Display name is required.")
                && resp.body.contains("Handle is required."),
            "validation response should mention all whitespace-only fields: {}",
            resp.body
        );
    }

    #[tokio::test]
    async fn profile_update_enforces_database_backed_lengths() {
        let mut harness = setup().await;
        let email = "length-profile@pulsar.test";
        let user = verified_user("Length Profile", email).await;
        Profile::ensure_for_user(&user)
            .await
            .expect("ensure current profile");
        let addr = harness.spawn_app().await;
        let mut client = Client::new(addr);
        login(&mut client, email).await;

        let long_url = format!("https://example.com/{}", "a".repeat(481));
        let resp = client
            .patch_json(
                "/profile",
                json!({
                    "name": "N".repeat(121),
                    "email": email,
                    "display_name": "D".repeat(121),
                    "handle": "h".repeat(65),
                    "avatar_url": long_url,
                    "website_url": format!("https://example.com/{}", "w".repeat(481)),
                    "github_url": format!("https://github.com/{}", "g".repeat(482)),
                    "location": "L".repeat(121),
                    "timezone": "T".repeat(81),
                }),
            )
            .await;

        assert_eq!(
            resp.status, 422,
            "overlong profile fields should be rejected"
        );
        for expected in [
            "Name must be at most 120 characters.",
            "Display name must be at most 120 characters.",
            "Handle must be at most 64 characters.",
            "Avatar URL must be at most 500 characters.",
            "Website URL must be at most 500 characters.",
            "GitHub URL must be at most 500 characters.",
            "Location must be at most 120 characters.",
            "Timezone must be at most 80 characters.",
        ] {
            assert!(
                resp.body.contains(expected),
                "validation response should contain `{expected}`: {}",
                resp.body
            );
        }
    }

    #[tokio::test]
    async fn profile_update_saves_public_member_fields() {
        let mut harness = setup().await;
        let email = "profile-fields@pulsar.test";
        let user = verified_user("Profile Fields", email).await;
        Profile::ensure_for_user(&user)
            .await
            .expect("ensure current profile");
        let addr = harness.spawn_app().await;
        let mut client = Client::new(addr);
        login(&mut client, email).await;

        let resp = client
            .patch_json(
                "/profile",
                json!({
                    "name": "Grace Account",
                    "email": email,
                    "display_name": "Grace Hopper",
                    "handle": "grace-hopper",
                    "bio": "Compiler pioneer and naval officer.",
                    "avatar_url": "https://cdn.pulsar.test/grace.png",
                    "website_url": "https://grace.example",
                    "github_url": "https://github.com/grace",
                    "location": "Arlington, VA",
                    "timezone": "America/New_York",
                }),
            )
            .await;

        assert_eq!(resp.status, 302, "profile field update: {}", resp.body);
        assert_eq!(resp.location(), "/profile");

        let updated = User::find_by_email(email)
            .await
            .expect("reload user")
            .expect("updated user exists");
        assert_eq!(updated.name, "Grace Account");

        let profile = Profile::find_by_user_id(updated.id)
            .await
            .expect("reload profile")
            .expect("profile exists");
        assert_eq!(profile.handle, "grace-hopper");
        assert_eq!(profile.display_name, "Grace Hopper");
        assert_eq!(
            profile.bio.as_deref(),
            Some("Compiler pioneer and naval officer.")
        );
        assert_eq!(
            profile.avatar_url.as_deref(),
            Some("https://cdn.pulsar.test/grace.png")
        );
        assert_eq!(
            profile.website_url.as_deref(),
            Some("https://grace.example")
        );
        assert_eq!(
            profile.github_url.as_deref(),
            Some("https://github.com/grace")
        );
        assert_eq!(profile.location.as_deref(), Some("Arlington, VA"));
        assert_eq!(profile.timezone.as_deref(), Some("America/New_York"));

        let resp = client.get("/members/grace-hopper").await;
        assert_eq!(
            resp.status, 200,
            "member profile should render: {}",
            resp.body
        );
        assert!(
            resp.body.contains("Grace Hopper"),
            "public member page should include display name: {}",
            resp.body
        );
    }

    #[tokio::test]
    async fn members_index_lists_public_profiles() {
        let mut harness = setup().await;
        let ada = verified_user("Ada Account", "ada-members@pulsar.test").await;
        public_profile(&ada, "ada-lovelace", "Ada Lovelace").await;
        let alan = verified_user("Alan Account", "alan-members@pulsar.test").await;
        public_profile(&alan, "alan-turing", "Alan Turing").await;
        let addr = harness.spawn_app().await;
        let mut client = Client::new(addr);

        let resp = client.get("/members").await;

        assert_eq!(
            resp.status, 200,
            "members index should render: {}",
            resp.body
        );
        assert!(
            resp.body.contains("Ada Lovelace") && resp.body.contains("Alan Turing"),
            "members index should include public profiles: {}",
            resp.body
        );
    }

    #[tokio::test]
    async fn member_profile_is_visible_by_handle() {
        let mut harness = setup().await;
        let user = verified_user("Visible Account", "visible-member@pulsar.test").await;
        let mut profile = public_profile(&user, "visible-member", "Visible Member").await;
        profile.bio = Some("Visible public biography.".to_string());
        profile.website_url = Some("https://visible.example".to_string());
        Model::save(&profile)
            .await
            .expect("save visible public profile fields");
        let addr = harness.spawn_app().await;
        let mut client = Client::new(addr);

        let resp = client.get("/members/visible-member").await;

        assert_eq!(
            resp.status, 200,
            "member profile should render: {}",
            resp.body
        );
        assert!(
            resp.body.contains("Visible Member")
                && resp.body.contains("Visible public biography.")
                && resp.body.contains("visible.example")
                && resp.body.contains("contribution_counts")
                && resp.body.contains("badges"),
            "member profile should expose summary props: {}",
            resp.body
        );

        let missing = client.get("/members/not-a-member").await;
        assert_eq!(missing.status, 404, "unknown handles should 404");
    }

    #[tokio::test]
    async fn badge_find_by_slug_uses_key() {
        let _harness = setup().await;
        let badge = <Badge as Model>::create(attrs! {
            key: "founding-member",
            name: "Founding Member",
            role_scoped: false,
        })
        .await
        .expect("create badge");

        let found = Badge::find_by_slug("founding-member")
            .await
            .expect("lookup badge by slug")
            .expect("badge slug resolves by key");

        assert_eq!(found.id, badge.id);
    }

    #[tokio::test]
    async fn taxonomy_models_find_by_slug() {
        let _harness = setup().await;
        let category = <Category as Model>::create(attrs! {
            name: "Programming",
            slug: "programming",
            sort_order: 0,
            is_visible: true,
        })
        .await
        .expect("create category");
        let topic = <Topic as Model>::create(attrs! {
            name: "Rust",
            slug: "rust",
            sort_order: 0,
            is_visible: true,
        })
        .await
        .expect("create topic");
        let tag = <Tag as Model>::create(attrs! {
            name: "Async",
            slug: "async",
        })
        .await
        .expect("create tag");

        assert_eq!(
            Category::find_by_slug("programming")
                .await
                .expect("lookup category")
                .expect("category exists")
                .id,
            category.id
        );
        assert_eq!(
            Topic::find_by_slug("rust")
                .await
                .expect("lookup topic")
                .expect("topic exists")
                .id,
            topic.id
        );
        assert_eq!(
            Tag::find_by_slug("async")
                .await
                .expect("lookup tag")
                .expect("tag exists")
                .id,
            tag.id
        );
    }

    #[tokio::test]
    async fn reputation_events_can_be_loaded_for_user_newest_first() {
        let _harness = setup().await;
        let user = User::create("Reputation User", "reputation@pulsar.test", "supersecret")
            .await
            .expect("create user");
        let other = User::create("Other User", "other-reputation@pulsar.test", "supersecret")
            .await
            .expect("create other user");

        let first = <ReputationEvent as Model>::create(attrs! {
            user_id: user.id,
            event_type: "article_published",
            target_type: "Article",
            target_id: 10_i64,
            weight: 5,
        })
        .await
        .expect("create first event");
        let second = <ReputationEvent as Model>::create(attrs! {
            user_id: user.id,
            event_type: "comment_featured",
            target_type: "Comment",
            target_id: 20_i64,
            weight: 3,
        })
        .await
        .expect("create second event");
        <ReputationEvent as Model>::create(attrs! {
            user_id: other.id,
            event_type: "other_user_event",
            target_type: "Article",
            target_id: 30_i64,
            weight: 1,
        })
        .await
        .expect("create other user event");

        let events = ReputationEvent::for_user(user.id)
            .await
            .expect("load reputation events");

        assert_eq!(events.len(), 2);
        assert_eq!(events[0].id, second.id);
        assert_eq!(events[1].id, first.id);
    }

    #[tokio::test]
    async fn member_cannot_open_admin_taxonomy() {
        let mut harness = setup().await;
        seed_default_roles().await.expect("seed roles");
        let user = verified_user("Taxonomy Member", "taxonomy-member@pulsar.test").await;
        user.assign_role("member").await.expect("assign member");
        let addr = harness.spawn_app().await;
        let mut client = Client::new(addr);
        login(&mut client, "taxonomy-member@pulsar.test").await;

        let resp = client.get("/admin/taxonomy").await;
        assert_eq!(resp.status, 403);
    }

    #[tokio::test]
    async fn member_cannot_create_admin_category() {
        let mut harness = setup().await;
        seed_default_roles().await.expect("seed roles");
        let user = verified_user("Taxonomy Writer", "taxonomy-writer@pulsar.test").await;
        user.assign_role("member").await.expect("assign member");
        let addr = harness.spawn_app().await;
        let mut client = Client::new(addr);
        login(&mut client, "taxonomy-writer@pulsar.test").await;

        let resp = client
            .post_json(
                "/admin/categories",
                json!({
                    "name": "Forbidden Category",
                    "slug": "forbidden-category",
                    "description": "Members cannot manage taxonomy.",
                    "sort_order": 5,
                    "is_visible": true,
                }),
            )
            .await;

        assert_eq!(resp.status, 403);
        assert!(
            Category::find_by_slug("forbidden-category")
                .await
                .expect("lookup forbidden category")
                .is_none(),
            "denied taxonomy writes must not create a category"
        );
    }

    #[tokio::test]
    async fn admin_can_create_taxonomy_category() {
        let mut harness = setup().await;
        seed_default_roles().await.expect("seed roles");
        let user = verified_user("Taxonomy Admin", "taxonomy-admin@pulsar.test").await;
        user.assign_role("admin").await.expect("assign admin");
        let addr = harness.spawn_app().await;
        let mut client = Client::new(addr);
        login(&mut client, "taxonomy-admin@pulsar.test").await;

        let resp = client
            .post_json(
                "/admin/categories",
                json!({
                    "name": "Programming",
                    "slug": "programming",
                    "description": "Languages, tooling, and software practice.",
                    "sort_order": 10,
                    "is_visible": true,
                }),
            )
            .await;

        assert_eq!(
            resp.status, 302,
            "admin category creation should redirect: {}",
            resp.body
        );
        assert_eq!(resp.location(), "/admin/taxonomy");

        let category = Category::find_by_slug("programming")
            .await
            .expect("lookup created category")
            .expect("created category exists");
        assert_eq!(category.name, "Programming");
        assert_eq!(
            category.description.as_deref(),
            Some("Languages, tooling, and software practice.")
        );
        assert_eq!(category.sort_order, 10);
        assert!(category.is_visible);
    }

    #[tokio::test]
    async fn public_topics_show_visible_topics_only() {
        let mut harness = setup().await;
        <Topic as Model>::create(attrs! {
            name: "Rust Programming",
            slug: "rust-programming",
            description: "Ownership, async systems, and practical Rust.",
            sort_order: 1,
            is_visible: true,
        })
        .await
        .expect("create visible topic");
        <Topic as Model>::create(attrs! {
            name: "Private Planning",
            slug: "private-planning",
            description: "Hidden taxonomy work.",
            sort_order: 2,
            is_visible: false,
        })
        .await
        .expect("create hidden topic");
        let addr = harness.spawn_app().await;
        let mut client = Client::new(addr);

        let index = client.get("/topics").await;
        assert_eq!(
            index.status, 200,
            "topics index should render: {}",
            index.body
        );
        assert!(
            index.body.contains("Rust Programming") && !index.body.contains("Private Planning"),
            "topics index should include visible topics only: {}",
            index.body
        );

        let visible = client.get("/topics/rust-programming").await;
        assert_eq!(
            visible.status, 200,
            "visible topic detail should render: {}",
            visible.body
        );
        assert!(
            visible.body.contains("Rust Programming")
                && visible.body.contains("contribution_counts"),
            "visible topic detail should expose empty contribution counts: {}",
            visible.body
        );

        let hidden = client.get("/topics/private-planning").await;
        assert_eq!(hidden.status, 404, "hidden topic detail should 404");
    }

    #[tokio::test]
    async fn moderator_can_open_moderation_access_surface() {
        let mut harness = setup().await;
        seed_default_roles().await.expect("seed roles");
        let user = verified_user("Queue Moderator", "queue-moderator@pulsar.test").await;
        user.assign_role("moderator")
            .await
            .expect("assign moderator");
        let addr = harness.spawn_app().await;
        let mut client = Client::new(addr);
        login(&mut client, "queue-moderator@pulsar.test").await;

        let resp = client.get("/moderation").await;
        assert_eq!(
            resp.status, 200,
            "moderator should reach moderation foundation surface: {}",
            resp.body
        );
    }

    #[tokio::test]
    async fn admin_can_open_admin_users() {
        let mut harness = setup().await;
        seed_default_roles().await.expect("seed roles");
        let user = verified_user("Users Admin", "users-admin@pulsar.test").await;
        user.assign_role("admin").await.expect("assign admin");
        let addr = harness.spawn_app().await;
        let mut client = Client::new(addr);
        login(&mut client, "users-admin@pulsar.test").await;

        let resp = client.get("/admin/users").await;
        assert_eq!(
            resp.status, 200,
            "admin should reach users foundation surface: {}",
            resp.body
        );
    }
}
