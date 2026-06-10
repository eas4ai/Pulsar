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
pub struct LandingMetric {
    pub value: String,
    pub label: String,
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
    pub metrics: Vec<LandingMetric>,
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
            metrics: vec![
                LandingMetric {
                    value: "8765".to_string(),
                    label: "backend dev port".to_string(),
                },
                LandingMetric {
                    value: "5765".to_string(),
                    label: "Vite dev port".to_string(),
                },
                LandingMetric {
                    value: "1".to_string(),
                    label: "public asset fallback".to_string(),
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
