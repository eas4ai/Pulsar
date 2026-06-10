use suprnova::{
    PermissionMiddleware, StaticFiles, delete, fallback, get, group, patch, post, put, routes,
};

use crate::controllers;
use crate::middleware;
use crate::models::user::User;

routes! {
    // Public routes
    get!("/", controllers::home::index),
    get!("/docs", controllers::docs::index),
    get!("/docs/{slug}", controllers::docs::show),
    get!("/blog", controllers::articles::index),
    get!("/blog/{slug}", controllers::articles::show),
    get!("/feed.xml", controllers::feed::rss),
    get!("/members", controllers::members::index),
    get!("/members/{handle}", controllers::members::show),

    // The verification-token consume endpoint is self-contained: the token in
    // the query string is the proof, so no session is needed. Keep it public
    // so a link opened on a logged-out device still verifies (gating it behind
    // `auth()` would redirect to `/login` and drop the token).
    get!("/verify-email/verify", controllers::verify_email::verify),

    // Guest-only routes (redirect to dashboard if logged in)
    group!("/", {
        get!("/login", controllers::auth::show_login),
        post!("/login", controllers::auth::login),
        get!("/register", controllers::auth::show_register),
        post!("/register", controllers::auth::register),
        get!("/forgot-password", controllers::password_reset::show_request),
        post!("/forgot-password", controllers::password_reset::send_link),
        get!("/reset-password", controllers::password_reset::show_reset),
        post!("/reset-password", controllers::password_reset::reset),
    }).middleware(middleware::authenticate::guest()),

    // Authenticated, verification NOT required. An unverified-but-logged-in
    // user must be able to view the notice, resend the link, and log out — so
    // these stay off the `verified` gate. (The verify-token endpoint itself is
    // public, above — it needs no session.)
    group!("/", {
        get!("/verify-email", controllers::verify_email::show_notice),
        post!("/email/verification-notification", controllers::verify_email::resend),
        post!("/logout", controllers::auth::logout),

        // Self-service profile management. Kept off the `verified` gate so an
        // unverified-but-logged-in user can still update their details — and,
        // by changing their email, re-trigger verification.
        get!("/profile", controllers::profile::show),
        patch!("/profile", controllers::profile::update),
        put!("/profile/password", controllers::profile::update_password),
        delete!("/profile", controllers::profile::destroy),
    }).middleware(middleware::authenticate::auth()),

    // Authenticated AND email-verified. `verified()` composes after `auth()`,
    // so an unverified user is redirected to `/verify-email` and an
    // unauthenticated one to `/login`.
    group!("/", {
        get!("/dashboard", controllers::dashboard::index),
    })
        .middleware(middleware::authenticate::auth())
        .middleware(middleware::authenticate::verified()),

    get!("/admin/articles", controllers::admin_articles::index)
        .middleware(middleware::authenticate::auth())
        .middleware(middleware::authenticate::verified())
        .middleware(PermissionMiddleware::<User>::new("articles.create")),
    get!("/admin/articles/create", controllers::admin_articles::create)
        .middleware(middleware::authenticate::auth())
        .middleware(middleware::authenticate::verified())
        .middleware(PermissionMiddleware::<User>::new("articles.create")),
    post!("/admin/articles", controllers::admin_articles::store)
        .middleware(middleware::authenticate::auth())
        .middleware(middleware::authenticate::verified())
        .middleware(PermissionMiddleware::<User>::new("articles.create")),
    get!("/admin/articles/{id}/edit", controllers::admin_articles::edit)
        .middleware(middleware::authenticate::auth())
        .middleware(middleware::authenticate::verified())
        .middleware(PermissionMiddleware::<User>::new("articles.update")),
    put!("/admin/articles/{id}", controllers::admin_articles::update)
        .middleware(middleware::authenticate::auth())
        .middleware(middleware::authenticate::verified())
        .middleware(PermissionMiddleware::<User>::new("articles.update")),
    post!("/admin/articles/{id}/publish", controllers::admin_articles::publish)
        .middleware(middleware::authenticate::auth())
        .middleware(middleware::authenticate::verified())
        .middleware(PermissionMiddleware::<User>::new("articles.publish")),
    get!("/admin/taxonomy", controllers::admin_taxonomy::index)
        .middleware(middleware::authenticate::auth())
        .middleware(middleware::authenticate::verified())
        .middleware(PermissionMiddleware::<User>::new("taxonomy.manage")),
    get!("/admin/users", controllers::admin_users::index)
        .middleware(middleware::authenticate::auth())
        .middleware(middleware::authenticate::verified())
        .middleware(PermissionMiddleware::<User>::new("users.manage")),
    get!("/moderation", controllers::moderation::index)
        .middleware(middleware::authenticate::auth())
        .middleware(middleware::authenticate::verified())
        .middleware(PermissionMiddleware::<User>::new("moderation.review")),

    fallback!(StaticFiles::public()
        .cache_control("public, max-age=86400")
        .handler()),
}
