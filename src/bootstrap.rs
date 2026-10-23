//! Application Bootstrap
//!
//! This is where you register global middleware and services that need runtime configuration.
//! Services that don't need runtime config can use `#[service(ConcreteType)]` instead.
//!
//! # Example
//!
//! ```rust,ignore
//! // For services with no runtime config, use the macro:
//! #[service(RedisCache)]
//! pub trait CacheStore { ... }
//!
//! // For services needing runtime config, register here:
//! pub async fn register() {
//!     // Initialize database
//!     DB::init().await.expect("Failed to connect to database");
//!
//!     // Global middleware
//!     global_middleware!(middleware::LoggingMiddleware);
//!
//!     // Services
//!     bind!(dyn Database, PostgresDB::new());
//! }
//! ```

use std::sync::Arc;

use suprnova::{
    App, Auth, AuthConfig, AuthManager, CsrfMiddleware, DB, EloquentUserProvider, FrameworkError,
    Frontend, IncludeMiddleware, Inertia, InertiaConfig, SecurityHeaders, SessionConfig,
    SessionMiddleware, global_middleware, serde_json,
};

use crate::middleware;
use crate::models::user::User;

/// Share each prop by name so RenderCache can validate its dependencies.
pub fn register_inertia_shared_data() {
    App::inertia_share_lazy("auth", || async {
        let user = Auth::user_as::<User>().await?;
        Ok::<_, FrameworkError>(serde_json::json!({
            "user": user.map(|u| serde_json::json!({
                "id": u.id,
                "name": u.name.unwrap_or_else(|| "Account".to_owned()),
                "email": u.email,
            })),
        }))
    });
    App::inertia_share_lazy("root", || async {
        Ok::<_, FrameworkError>(suprnova::url::root())
    });
}

#[suprnova::inertia_root(path = "app.html")]
pub struct AppDocument;

/// Services used by the server, workers and console commands.
pub async fn register() {
    // Console and server bootstrap both need Crypt before Magnetar is installed.
    suprnova::boot::initialize_crypt_or_exit();

    // Initialize database connection
    DB::init().await.expect("Failed to connect to database");
    let db = DB::connection().expect("DB not initialized");
    let magnetar = suprnova::MagnetarConfig::from_sea_orm(db.inner().clone()).passkey_config(
        suprnova::PasskeyConfig {
            rp_id: std::env::var("PASSKEY_RP_ID").unwrap_or_else(|_| "localhost".to_string()),
            rp_origin: std::env::var("PASSKEY_RP_ORIGIN")
                .unwrap_or_else(|_| "http://localhost".to_string()),
        },
    );
    suprnova::init_magnetar(magnetar)
        .await
        .expect("Failed to initialize Magnetar");

    // Authentication: register the AuthManager (the config/auth.php analogue)
    // and a user provider so `Auth::attempt` and `Auth::user_as::<User>()`
    // resolve users. `EloquentUserProvider<User>` queries the typed model; the
    // The HTTP session middleware persists the authenticated id across requests.
    App::singleton(AuthManager::new(AuthConfig::from_env()));
    Auth::register_provider("users", Arc::new(EloquentUserProvider::<User>::new()))
        .expect("register users provider");

    crate::live::authorize();
    App::singleton(crate::live::registry().expect("Live component registry"));
}

/// HTTP middleware and assets are needed only by processes that serve pages.
pub fn register_http_stack() {
    try_register_http_stack()
        .expect("HTTP stack: check security settings and the built frontend manifest");
}

pub fn try_register_http_stack() -> Result<(), FrameworkError> {
    let catalog = suprnova::live::assets::live_asset_catalog()
        .map_err(|error| FrameworkError::internal(format!("Live async assets: {error}")))?;
    let artifact = catalog.artifact(suprnova::live::assets::ArtifactRole::AsyncEsm);
    suprnova::View::share(
        "live_async_url",
        format!(
            "{}/{}/{}",
            suprnova::live::assets::LIVE_ASSET_PATH_PREFIX,
            catalog.identity(),
            artifact.file()
        ),
    );
    suprnova::View::share("live_async_sri", artifact.sri().to_owned());
    // Security policy wraps the complete HTTP stack so error pages and
    // framework-generated asset tags share the request's CSP nonce.
    global_middleware!(SecurityHeaders::from_env()?);

    // Global middleware (runs on every request in registration order)
    global_middleware!(middleware::LoggingMiddleware);

    // Public prose has a discoverable `.md` twin for agents and text-only
    // clients. Zero-age caching prevents a withdrawn article remaining public.
    global_middleware!(
        suprnova_markdown::MarkdownMiddleware::new(crate::content::markdown::PublicMarkdown)
            .max_age(0)
    );

    // Session middleware (required for authentication)
    let session_config = SessionConfig::from_env();

    // This outer layer removes disposable session/CSRF cookies from public
    // responses. If another cookie must be delivered, it preserves the cookie
    // and changes that response to private instead.
    global_middleware!(middleware::PublicCacheCookies::new(
        session_config
            .cookie_prefix
            .apply(&session_config.cookie_name),
    ));

    global_middleware!(SessionMiddleware::new(session_config.clone()));

    // CSRF protection (validates tokens on POST/PUT/PATCH/DELETE)
    global_middleware!(CsrfMiddleware::new().with_session_config(&session_config));

    // Parse `?include=`/`?exclude=`/`?only=`/`?except=` and `?fields[...]=`
    // into the per-request task-local so `#[derive(Data)]` responses,
    // `Resource::single`, and `Prop::Lazy` resolution honour the client's
    // requested shape out of the box. Without this, Data DTOs silently
    // ignore include/fieldset query parameters.
    global_middleware!(IncludeMiddleware);

    Inertia::install(&inertia_config())?;
    register_inertia_shared_data();
    Ok(())
}

/// The page shell and entries used by every Inertia response.
pub fn inertia_config() -> InertiaConfig {
    let config = match suprnova::Environment::detect() {
        suprnova::Environment::Local
        | suprnova::Environment::Development
        | suprnova::Environment::Testing => InertiaConfig::new(),
        _ => InertiaConfig::new().production(),
    };
    config
        .frontend(Frontend::Vue)
        .entry_points(["src/vuetify.scss", "src/app.css"])
        .root_template(suprnova::InertiaRootTemplate::of::<AppDocument>())
}
