//! Documentation controllers.

use suprnova::content::{DocsCatalog, DocsChapter};
use suprnova::{InertiaProps, Request, Response, handler, inertia_response};

use crate::content::docs::{first_chapter_slug, load_catalog, load_chapter};
use crate::controllers::inertia_config;

#[derive(InertiaProps)]
pub struct DocsPageProps {
    pub catalog: DocsCatalog,
    pub chapter: DocsChapter,
}

#[handler]
pub async fn index(req: Request) -> Response {
    let catalog = load_catalog()?;
    let slug = first_chapter_slug(&catalog)?;
    let chapter = load_chapter(slug)?;

    inertia_response!(
        &req,
        "docs/Index",
        DocsPageProps { catalog, chapter },
        inertia_config()
    )
}

#[handler]
pub async fn show(req: Request) -> Response {
    let slug = req
        .param("slug")
        .map_err(|_| suprnova::FrameworkError::param("slug"))?;
    let catalog = load_catalog()?;
    let chapter = load_chapter(slug)?;

    inertia_response!(
        &req,
        "docs/Show",
        DocsPageProps { catalog, chapter },
        inertia_config()
    )
}
