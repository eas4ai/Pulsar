//! V2 foundation flow tests.

mod common;

mod v2_foundation_flows {
    use serde_json::json;

    use suprnova::attrs;
    use suprnova::eloquent::Model;
    use suprnova::mail::Mail;

    use super::common::{Client, setup};
    use pulsar::models::badge::Badge;
    use pulsar::models::profile::Profile;
    use pulsar::models::reputation_event::ReputationEvent;
    use pulsar::models::user::User;

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
}
