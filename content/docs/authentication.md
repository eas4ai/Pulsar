# Authentication

Pulsar ships account workflows through Suprnova sessions, middleware, signed links, mail, and the local `User` and `Profile` models. The goal is to make common product accounts work before you write domain features.

## Public and Guest Routes

- `/login` and `/register` are guest-only.
- `/forgot-password` sends a reset link.
- `/reset-password` accepts the token and new password.
- `/verify-email/verify` is public so a link can be opened from a logged-out browser.

Guest-only routes use `middleware::authenticate::guest()`, which redirects signed-in users away from login and registration screens.

## Authenticated Routes

These routes require a session but do not require verified email:

- `/verify-email`
- `/email/verification-notification`
- `/logout`
- `/profile`
- `/profile/password`

Keeping profile routes available before verification lets users correct a mistyped email address and request a new verification link.

## Verified Routes

Product routes that need a trusted account compose `auth()` with `verified()`.

```rust
group!("/", {
    get!("/dashboard", controllers::dashboard::index),
})
    .middleware(middleware::authenticate::auth())
    .middleware(middleware::authenticate::verified())
```

The dashboard and admin surfaces use this pattern. Admin pages also add permission middleware such as `PermissionMiddleware::<User>::new("articles.create")`.

## Profiles

Pulsar creates and maintains public profile data separately from the user account row. The account row owns credentials and email verification. The profile row owns display name, handle, bio, avatar URL, website, GitHub URL, location, and timezone.

Users edit both from `/profile`. Handles are lowercase, URL-safe, and unique. Public member pages use `/members/{handle}`.

## Mail

Local development defaults to `MAIL_DRIVER=log`. Verification and password reset links are logged instead of sent. For real email, configure SMTP settings in `.env` and set `MAIL_DRIVER=smtp`.

Continue with [Frontend and Design System](frontend.md).
