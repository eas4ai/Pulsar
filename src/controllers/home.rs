use serde::Serialize;
use suprnova::{InertiaProps, Request, Response, handler, inertia_response};

use crate::controllers::inertia_config;

#[derive(Serialize)]
pub struct LandingFeature {
    pub icon: String,
    pub title: String,
    pub body: String,
}

#[derive(Serialize)]
pub struct LandingCapability {
    pub icon: String,
    pub title: String,
    pub body: String,
}

#[derive(Serialize)]
pub struct LandingCodeSample {
    pub language: String,
    pub code: String,
}

#[derive(InertiaProps)]
pub struct HomeProps {
    pub headline: String,
    pub subheadline: String,
    pub features: Vec<LandingFeature>,
    pub capabilities: Vec<LandingCapability>,
    pub sample: LandingCodeSample,
}

#[handler]
pub async fn index(req: Request) -> Response {
    inertia_response!(
        &req,
        "Home",
        HomeProps {
            headline: "Ship a Suprnova product site without rebuilding the rails.".to_string(),
            subheadline: "Pulsar packages the product surfaces teams keep rebuilding: polished Inertia pages, account flows, static assets, and a path toward docs and publishing.".to_string(),
            features: vec![
                LandingFeature {
                    icon: "mdi-rocket-launch-outline".to_string(),
                    title: "Launch-ready shell".to_string(),
                    body: "A real landing page, auth screens, dashboard layout, and responsive Vuetify system wired through Inertia.".to_string(),
                },
                LandingFeature {
                    icon: "mdi-shield-check-outline".to_string(),
                    title: "Framework-backed flows".to_string(),
                    body: "Registration, login, sessions, verification, password reset, and profile management use Suprnova primitives end to end.".to_string(),
                },
                LandingFeature {
                    icon: "mdi-file-code-outline".to_string(),
                    title: "Production asset path".to_string(),
                    body: "Vite builds into the public tree and Suprnova serves the generated assets with one static fallback.".to_string(),
                },
                LandingFeature {
                    icon: "mdi-source-branch".to_string(),
                    title: "Room to grow".to_string(),
                    body: "Docs, article publishing, RSS, and RBAC are planned as first-party kit domains instead of bolt-ons.".to_string(),
                },
            ],
            capabilities: vec![
                LandingCapability {
                    icon: "mdi-account-check-outline".to_string(),
                    title: "Accounts wired".to_string(),
                    body: "Registration, login, sessions, verification, reset links, and profile edits are ready to extend.".to_string(),
                },
                LandingCapability {
                    icon: "mdi-book-open-page-variant-outline".to_string(),
                    title: "Docs generated".to_string(),
                    body: "Markdown docs build into searchable chapters with navigation, excerpts, and anchored headings.".to_string(),
                },
                LandingCapability {
                    icon: "mdi-rss".to_string(),
                    title: "Publishing included".to_string(),
                    body: "Articles, categories, tags, drafts, scheduled publishing, and the RSS feed share one content path.".to_string(),
                },
                LandingCapability {
                    icon: "mdi-account-group-outline".to_string(),
                    title: "Profiles included".to_string(),
                    body: "Member pages, contribution counts, badges, and social links establish the community layer.".to_string(),
                },
                LandingCapability {
                    icon: "mdi-view-dashboard-outline".to_string(),
                    title: "Role dashboards".to_string(),
                    body: "Authenticated users and admins land in purpose-built screens instead of generic placeholder pages.".to_string(),
                },
                LandingCapability {
                    icon: "mdi-tag-multiple-outline".to_string(),
                    title: "Taxonomy admin".to_string(),
                    body: "Categories, topics, and tags can be maintained through the admin UI with visible slugs and sorting.".to_string(),
                },
            ],
            sample: LandingCodeSample {
                language: "rust".to_string(),
                code: r#"routes! {
    get!("/", controllers::home::index),
    group!("/", {
        get!("/dashboard", controllers::dashboard::index),
    }).middleware(middleware::authenticate::auth()),
    fallback!(StaticFiles::public().handler()),
}"#
                .to_string(),
            },
        },
        inertia_config()
    )
}
