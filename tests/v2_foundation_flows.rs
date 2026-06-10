//! V2 foundation flow tests.

mod common;

mod v2_foundation_flows {
    use serde_json::json;

    use suprnova::mail::Mail;

    use super::common::{Client, setup};
    use pulsar::models::profile::Profile;
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
}
