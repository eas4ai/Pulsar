//! Documentation controllers.

use crate::content::docs_props::{DocsCatalog, DocsChapter};
use suprnova::{InertiaProps, Request, Response, handler, inertia_response};

use crate::content::docs::{first_chapter_slug, load_catalog, load_chapter};
use crate::content::meta::{PageMeta, strip_leading_title};
use crate::content::schema;
use crate::controllers::{inertia_config_titled, site_name, with_page_head};

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
    let title = chapter.title.clone();
    let description = strip_leading_title(&chapter.excerpt, &title);
    let path = format!("/docs/{}", chapter.slug);
    let blocks = schema::docs_chapter_blocks(&chapter.slug, &title, &description);

    let response = inertia_response!(
        &req,
        "docs/Index",
        DocsPageProps {
            catalog: catalog.into(),
            chapter: chapter.into()
        },
        inertia_config_titled(&title)
    );
    with_page_head(
        response,
        &PageMeta::article(format!("{title} · {}", site_name()), description, path).with_markdown(),
        &blocks,
    )
}

#[handler]
pub async fn show(req: Request) -> Response {
    let slug = req
        .param("slug")
        .map_err(|_| suprnova::FrameworkError::param("slug"))?;
    let catalog = load_catalog()?;
    let chapter = load_chapter(slug)?;
    let title = chapter.title.clone();
    let description = strip_leading_title(&chapter.excerpt, &title);
    let path = format!("/docs/{}", chapter.slug);
    let blocks = schema::docs_chapter_blocks(&chapter.slug, &title, &description);

    let response = inertia_response!(
        &req,
        "docs/Show",
        DocsPageProps {
            catalog: catalog.into(),
            chapter: chapter.into()
        },
        inertia_config_titled(&title)
    );
    with_page_head(
        response,
        &PageMeta::article(format!("{title} · {}", site_name()), description, path).with_markdown(),
        &blocks,
    )
}
