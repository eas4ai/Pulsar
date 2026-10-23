//! Documentation props kept local so the TypeScript generator can describe them.

use suprnova::InertiaProps;

#[derive(InertiaProps)]
pub struct DocsCatalog {
    pub chapters: Vec<DocsCatalogEntry>,
    pub search: Vec<DocsSearchEntry>,
}

impl From<suprnova::content::DocsCatalog> for DocsCatalog {
    fn from(value: suprnova::content::DocsCatalog) -> Self {
        Self {
            chapters: value.chapters.into_iter().map(Into::into).collect(),
            search: value.search.into_iter().map(Into::into).collect(),
        }
    }
}

#[derive(InertiaProps)]
pub struct DocsChapter {
    pub slug: String,
    pub title: String,
    pub html: String,
    pub excerpt: String,
    pub headings: Vec<DocsHeading>,
    pub previous: Option<String>,
    pub next: Option<String>,
}

impl From<suprnova::content::DocsChapter> for DocsChapter {
    fn from(value: suprnova::content::DocsChapter) -> Self {
        Self {
            slug: value.slug,
            title: value.title,
            html: value.html,
            excerpt: value.excerpt,
            headings: value.headings.into_iter().map(Into::into).collect(),
            previous: value.previous,
            next: value.next,
        }
    }
}

#[derive(InertiaProps)]
pub struct DocsCatalogEntry {
    pub slug: String,
    pub title: String,
    pub excerpt: String,
    pub headings: Vec<DocsHeading>,
    pub previous: Option<String>,
    pub next: Option<String>,
}

impl From<suprnova::content::DocsCatalogEntry> for DocsCatalogEntry {
    fn from(value: suprnova::content::DocsCatalogEntry) -> Self {
        Self {
            slug: value.slug,
            title: value.title,
            excerpt: value.excerpt,
            headings: value.headings.into_iter().map(Into::into).collect(),
            previous: value.previous,
            next: value.next,
        }
    }
}

#[derive(InertiaProps)]
pub struct DocsSearchEntry {
    pub slug: String,
    pub title: String,
    pub excerpt: String,
    pub headings: Vec<DocsHeading>,
    pub plain_text: String,
}

impl From<suprnova::content::DocsSearchEntry> for DocsSearchEntry {
    fn from(value: suprnova::content::DocsSearchEntry) -> Self {
        Self {
            slug: value.slug,
            title: value.title,
            excerpt: value.excerpt,
            headings: value.headings.into_iter().map(Into::into).collect(),
            plain_text: value.plain_text,
        }
    }
}

#[derive(InertiaProps)]
pub struct DocsHeading {
    pub level: u8,
    pub id: String,
    pub title: String,
}

impl From<suprnova::content::Heading> for DocsHeading {
    fn from(value: suprnova::content::Heading) -> Self {
        Self {
            level: value.level,
            id: value.id,
            title: value.title,
        }
    }
}
