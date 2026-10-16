//! Schema.org JSON-LD for public pages.

use suprnova::serde_json::{Value, json};

use crate::content::meta::{site_description, social_image_dimensions, social_image_url};
use crate::controllers::{app_url, site_name};

fn organization_id(base: &str) -> String {
    format!("{base}/#organization")
}

fn website_id(base: &str) -> String {
    format!("{base}/#website")
}

fn product_id(base: &str) -> String {
    format!("{base}/#pulsar")
}

pub fn site_blocks() -> Vec<Value> {
    let base = app_url();
    let name = site_name();
    let repository = std::env::var("SEO_REPOSITORY_URL")
        .unwrap_or_else(|_| "https://github.com/eas4ai/Pulsar".into());
    let language = std::env::var("SEO_LANGUAGE").unwrap_or_else(|_| "en".into());
    vec![
        json!({
            "@context": "https://schema.org",
            "@type": "Organization",
            "@id": organization_id(&base),
            "name": name,
            "url": base,
            "logo": { "@type": "ImageObject", "url": format!("{base}/android-chrome-512x512.png") },
            "sameAs": [repository]
        }),
        json!({
            "@context": "https://schema.org",
            "@type": "WebSite",
            "@id": website_id(&base),
            "name": name,
            "description": site_description(),
            "url": base,
            "publisher": { "@id": organization_id(&base) },
            "inLanguage": language
        }),
        json!({
            "@context": "https://schema.org",
            "@type": "SoftwareSourceCode",
            "@id": product_id(&base),
            "name": name,
            "description": site_description(),
            "url": base,
            "codeRepository": repository,
            "programmingLanguage": { "@type": "ComputerLanguage", "name": "Rust" },
            "runtimePlatform": "Tokio",
            "version": env!("CARGO_PKG_VERSION"),
            "license": "https://spdx.org/licenses/MIT.html",
            "author": { "@id": organization_id(&base) }
        }),
    ]
}

pub fn docs_chapter_blocks(slug: &str, title: &str, excerpt: &str) -> Vec<Value> {
    let base = app_url();
    let url = format!("{base}/docs/{slug}");
    vec![
        json!({
            "@context": "https://schema.org",
            "@type": "TechArticle",
            "headline": title,
            "description": excerpt,
            "url": url,
            "mainEntityOfPage": url,
            "inLanguage": std::env::var("SEO_LANGUAGE").unwrap_or_else(|_| "en".into()),
            "isPartOf": { "@id": website_id(&base) },
            "publisher": { "@id": organization_id(&base) },
            "about": { "@id": product_id(&base) }
        }),
        breadcrumb(&[("Documentation", format!("{base}/docs")), (title, url)]),
    ]
}

pub fn article_blocks(
    slug: &str,
    title: &str,
    excerpt: &str,
    tags: &[String],
    published: Option<&str>,
    modified: &str,
    author: Option<(&str, Option<&str>)>,
) -> Vec<Value> {
    let base = app_url();
    let url = format!("{base}/blog/{slug}");
    let (image_width, image_height) = social_image_dimensions();
    let mut posting = json!({
        "@context": "https://schema.org",
        "@type": "BlogPosting",
        "headline": title,
        "description": excerpt,
        "url": url,
        "mainEntityOfPage": url,
        "dateModified": modified,
        "publisher": { "@id": organization_id(&base) },
        "isPartOf": { "@id": website_id(&base) },
        "inLanguage": std::env::var("SEO_LANGUAGE").unwrap_or_else(|_| "en".into()),
        "image": {
            "@type": "ImageObject",
            "url": social_image_url(),
            "width": image_width,
            "height": image_height
        }
    });
    if let Some(published) = published {
        posting["datePublished"] = json!(published);
    }
    if !tags.is_empty() {
        posting["keywords"] = json!(tags);
    }
    if let Some((name, handle)) = author {
        posting["author"] = match handle {
            Some(handle) => json!({
                "@type": "Person",
                "name": name,
                "url": format!("{base}/members/{handle}")
            }),
            None => json!({ "@type": "Person", "name": name }),
        };
    }
    vec![
        posting,
        breadcrumb(&[("Blog", format!("{base}/blog")), (title, url)]),
    ]
}

fn breadcrumb(trail: &[(&str, String)]) -> Value {
    let items: Vec<Value> = trail
        .iter()
        .enumerate()
        .map(|(index, (name, url))| {
            json!({
                "@type": "ListItem",
                "position": index + 1,
                "name": name,
                "item": url
            })
        })
        .collect();
    json!({
        "@context": "https://schema.org",
        "@type": "BreadcrumbList",
        "itemListElement": items
    })
}
