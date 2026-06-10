//! Profile-management controller.
//!
//! Four authenticated handlers for the signed-in user to manage their own
//! account. They sit in the auth-but-not-verified route group, so an
//! unverified-yet-logged-in user can still update their profile (and, by
//! changing their email, re-trigger verification):
//!
//! - `GET    /profile`          — render the profile page seeded from the
//!   current user.
//! - `PATCH  /profile`          — update name/email. Changing the email nulls
//!   the verification timestamp and re-sends the verification link
//!   (best-effort — a mail failure must not 500 the update).
//! - `PUT    /profile/password` — rotate the password, gated on the current
//!   password.
//! - `DELETE /profile`          — password-gated account deletion: verify the
//!   password, log out, then delete the row.
//!
//! Validation runs through `controllers::inertia_form`: an `X-Inertia`
//! submission that fails re-renders the Profile page with a flat `errors`
//! prop the client merges into `form.errors`, while non-Inertia clients get
//! the Laravel-style 422 `{ message, errors }` envelope.

use serde::{Deserialize, Serialize};
use suprnova::auth_flows::EmailVerification;
use suprnova::{
    Auth, CanResetPassword, FormRequest, FrameworkError, InertiaProps, Model, MustVerifyEmail,
    Request, Response, Validate, ValidationErrors, handler, hashing, inertia_response, redirect,
};

use crate::controllers::{
    FormFailure, InertiaCtx, errors_json, inertia_config, inertia_form, validation_failure,
};
use crate::models::profile::Profile;
use crate::models::user::User;

// ============================================================================
// Props
// ============================================================================

#[derive(InertiaProps)]
pub struct ProfileProps {
    pub name: String,
    pub email: String,
    pub email_verified: bool,
    pub profile: ProfileFormProps,
}

#[derive(Serialize)]
pub struct ProfileFormProps {
    pub handle: String,
    pub display_name: String,
    pub bio: Option<String>,
    pub avatar_url: Option<String>,
    pub website_url: Option<String>,
    pub github_url: Option<String>,
    pub location: Option<String>,
    pub timezone: Option<String>,
}

// ============================================================================
// Requests
// ============================================================================

#[derive(Deserialize, Validate)]
pub struct UpdateProfileRequest {
    #[validate(length(min = 1, message = "Name is required."))]
    pub name: String,
    #[validate(email(message = "Enter a valid email address."))]
    pub email: String,
    pub display_name: Option<String>,
    pub handle: Option<String>,
    pub bio: Option<String>,
    pub avatar_url: Option<String>,
    pub website_url: Option<String>,
    pub github_url: Option<String>,
    pub location: Option<String>,
    pub timezone: Option<String>,
}

impl FormRequest for UpdateProfileRequest {}

#[derive(Deserialize, Validate)]
pub struct UpdatePasswordRequest {
    pub current_password: String,
    #[validate(length(min = 8, message = "Password must be at least 8 characters."))]
    pub password: String,
    #[validate(length(min = 1, message = "Confirm your new password."))]
    pub password_confirmation: String,
}

impl FormRequest for UpdatePasswordRequest {
    /// Cross-field check: the new password and its confirmation must match.
    /// Runs after the per-field rules, so `password` is already known to be
    /// well-formed (>= 8 chars) before we compare them.
    fn after_validation(&self) -> Result<(), ValidationErrors> {
        if self.password != self.password_confirmation {
            let mut errs = ValidationErrors::new();
            errs.add("password_confirmation", "Passwords do not match.");
            return Err(errs);
        }
        Ok(())
    }
}

/// The delete body carries only the confirming password. There are no shape
/// rules — the password is checked against the stored hash in the handler,
/// and a mismatch surfaces on the `password` field.
#[derive(Deserialize, Validate)]
pub struct DeleteAccountRequest {
    pub password: String,
}

impl FormRequest for DeleteAccountRequest {}

// ============================================================================
// Helpers
// ============================================================================

/// Resolve the currently authenticated user as the concrete `User` model.
/// These routes are behind `AuthMiddleware`, so a missing user means the
/// session expired between the gate and the handler — surface it as a 401.
async fn current_user() -> Result<User, FrameworkError> {
    Auth::user_as::<User>()
        .await?
        .ok_or(FrameworkError::Unauthorized)
}

/// Deliver profile validation errors: re-render the Profile page (seeded
/// from the current user, plus the `errors` prop) for Inertia submissions,
/// 422 envelope for everything else.
async fn render_profile(ctx: &InertiaCtx, errors: ValidationErrors) -> Response {
    if ctx.wants_inertia() {
        let user = current_user().await?;
        let profile = Profile::ensure_for_user(&user).await?;
        inertia_response!(
            ctx,
            "Profile",
            {
                "name": user.name,
                "email": user.email,
                "email_verified": user.is_email_verified(),
                "profile": profile_form_props(profile),
                "errors": errors_json(&errors),
            },
            inertia_config()
        )
    } else {
        Err(validation_failure(errors))
    }
}

struct PublicProfileInput {
    handle: String,
    display_name: String,
    bio: Option<String>,
    avatar_url: Option<String>,
    website_url: Option<String>,
    github_url: Option<String>,
    location: Option<String>,
    timezone: Option<String>,
}

fn profile_form_props(profile: Profile) -> ProfileFormProps {
    ProfileFormProps {
        handle: profile.handle,
        display_name: profile.display_name,
        bio: profile.bio,
        avatar_url: profile.avatar_url,
        website_url: profile.website_url,
        github_url: profile.github_url,
        location: profile.location,
        timezone: profile.timezone,
    }
}

fn normalize_optional(value: &Option<String>) -> Option<String> {
    value
        .as_deref()
        .map(str::trim)
        .filter(|value| !value.is_empty())
        .map(str::to_string)
}

fn is_slug_safe_handle(value: &str) -> bool {
    value
        .chars()
        .all(|ch| ch.is_ascii_lowercase() || ch.is_ascii_digit() || ch == '-')
}

fn public_profile_input(
    form: &UpdateProfileRequest,
    name: &str,
    errors: &mut ValidationErrors,
) -> Option<PublicProfileInput> {
    let handle = match normalize_optional(&form.handle) {
        Some(handle) => {
            if !is_slug_safe_handle(&handle) {
                errors.add(
                    "handle",
                    "Handle may only contain lowercase letters, numbers, and hyphens.",
                );
            }
            Some(handle)
        }
        None => {
            errors.add("handle", "Handle is required.");
            None
        }
    };

    handle.map(|handle| PublicProfileInput {
        handle,
        display_name: normalize_optional(&form.display_name).unwrap_or_else(|| name.to_string()),
        bio: normalize_optional(&form.bio),
        avatar_url: normalize_optional(&form.avatar_url),
        website_url: normalize_optional(&form.website_url),
        github_url: normalize_optional(&form.github_url),
        location: normalize_optional(&form.location),
        timezone: normalize_optional(&form.timezone),
    })
}

// ============================================================================
// Handlers
// ============================================================================

/// `GET /profile` — render the profile page seeded from the current user.
#[handler]
pub async fn show(req: Request) -> Response {
    let user = current_user().await?;
    let profile = Profile::ensure_for_user(&user).await?;
    inertia_response!(
        &req,
        "Profile",
        ProfileProps {
            name: user.name.clone(),
            email: user.email.clone(),
            email_verified: user.is_email_verified(),
            profile: profile_form_props(profile),
        },
        inertia_config()
    )
}

/// `PATCH /profile` — update name/email.
///
/// Changing the email address invalidates the existing verification: we null
/// the `email_verified_at` timestamp and re-send the verification link. That
/// send is best-effort — the row update has already succeeded, so a mail
/// failure (e.g. misconfigured `MAIL_FROM`) is logged and swallowed rather
/// than 500-ing the update. The user lands back on `/profile` showing
/// "not verified" and can resend from the verification notice.
#[handler]
pub async fn update(req: Request) -> Response {
    let ctx = InertiaCtx::of(&req);
    let form = match inertia_form::<UpdateProfileRequest>(req).await {
        Ok(form) => form,
        Err(FormFailure::Invalid(_, errors)) => return render_profile(&ctx, errors).await,
        Err(FormFailure::Response(resp)) => return Err(*resp),
    };

    let mut user = current_user().await?;
    let mut profile = Profile::ensure_for_user(&user).await?;
    let email_changed = user.email != form.email;
    let name = form.name.trim().to_string();
    let mut errors = ValidationErrors::new();
    let profile_input = public_profile_input(&form, &name, &mut errors);

    // Guard the `users.email` unique constraint: if the new address belongs to
    // a *different* account, surface the error on `email` rather than letting
    // the insert/update hit the DB constraint and 500. Re-submitting the same
    // address (email unchanged) is fine and skips the lookup.
    if email_changed
        && let Some(existing) = User::find_by_email(&form.email).await?
        && existing.id != user.id
    {
        errors.add("email", "This email is already registered.");
    }

    if let Some(input) = &profile_input
        && let Some(existing) = Profile::find_by_handle(&input.handle).await?
        && existing.user_id != user.id
    {
        errors.add("handle", "This handle is already taken.");
    }

    if !errors.errors.is_empty() {
        return render_profile(&ctx, errors).await;
    }

    let profile_input = profile_input.expect("validated profile input exists");

    user.name = name;
    user.email = form.email;
    if email_changed {
        user.set_email_verified_at(None);
    }
    Model::save(&user).await?;

    profile.handle = profile_input.handle;
    profile.display_name = profile_input.display_name;
    profile.bio = profile_input.bio;
    profile.avatar_url = profile_input.avatar_url;
    profile.website_url = profile_input.website_url;
    profile.github_url = profile_input.github_url;
    profile.location = profile_input.location;
    profile.timezone = profile_input.timezone;
    Model::save(&profile).await?;

    if email_changed {
        let base = format!("{}/verify-email/verify", crate::controllers::app_url());
        if let Err(err) = EmailVerification::send_link(&user, &base).await {
            tracing::warn!(error = %err, "failed to send verification email after email change");
        }
    }

    redirect!("/profile").into()
}

/// `PUT /profile/password` — rotate the password.
///
/// Gated on the current password: a wrong `current_password` surfaces on
/// that field rather than failing silently. On success the new (hashed)
/// password is persisted through `set_password_hash` + `save`.
#[handler]
pub async fn update_password(req: Request) -> Response {
    let ctx = InertiaCtx::of(&req);
    let form = match inertia_form::<UpdatePasswordRequest>(req).await {
        Ok(form) => form,
        Err(FormFailure::Invalid(_, errors)) => return render_profile(&ctx, errors).await,
        Err(FormFailure::Response(resp)) => return Err(*resp),
    };

    let mut user = current_user().await?;

    if !user.verify_password(&form.current_password)? {
        let mut errors = ValidationErrors::new();
        errors.add("current_password", "The current password is incorrect.");
        return render_profile(&ctx, errors).await;
    }

    user.set_password_hash(&hashing::hash(&form.password)?);
    Model::save(&user).await?;

    redirect!("/profile").into()
}

/// `DELETE /profile` — password-gated account deletion.
///
/// Verify the confirming password (wrong → error on `password`), then log
/// the session out and delete the user row. Deletion happens last so an
/// already-logged-out-then-failed-delete can't leave a ghost session pointing
/// at a live account; if the delete fails the user is logged out and re-auth
/// is required, which is the safe direction.
#[handler]
pub async fn destroy(req: Request) -> Response {
    let ctx = InertiaCtx::of(&req);
    let form = match inertia_form::<DeleteAccountRequest>(req).await {
        Ok(form) => form,
        Err(FormFailure::Invalid(_, errors)) => return render_profile(&ctx, errors).await,
        Err(FormFailure::Response(resp)) => return Err(*resp),
    };

    let user = current_user().await?;

    if !user.verify_password(&form.password)? {
        let mut errors = ValidationErrors::new();
        errors.add("password", "The password is incorrect.");
        return render_profile(&ctx, errors).await;
    }

    if let Some(profile) = Profile::find_by_user_id(user.id).await? {
        Model::delete(profile).await?;
    }

    Auth::logout().await?;
    Model::delete(user).await?;

    redirect!("/").into()
}
