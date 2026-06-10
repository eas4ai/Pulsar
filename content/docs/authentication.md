# Authentication

Pulsar ships the common account workflows through Suprnova middleware, session storage, and the `User` model.

## Included Flows

- Registration and login live under `/register` and `/login`.
- Email verification uses `/verify-email` plus signed verification-token links.
- Password reset uses `/forgot-password` and `/reset-password`.
- Profile updates live under `/profile`, including password changes and account deletion.

## Route Gates

Guest-only pages use `middleware::authenticate::guest()`. Signed-in routes use `auth()`, and product routes that require verified email compose `auth()` with `verified()`.

```rust
group!("/", {
    get!("/dashboard", controllers::dashboard::index),
})
    .middleware(middleware::authenticate::auth())
    .middleware(middleware::authenticate::verified())
```

Continue with [deployment](deployment.md) once auth-sensitive pages are complete.
