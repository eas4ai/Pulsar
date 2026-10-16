//! Machine-readable maps of the public documentation and published articles.

use suprnova::{HttpResponse, Request, Response, handler};

use crate::content::docs::{chapter_markdown, load_catalog};
use crate::content::meta::site_description;
use crate::controllers::{app_url, site_name};
use crate::models::article::Article;

#[handler]
pub async fn index(_req: Request) -> Response {
    let base = app_url();
    let name = site_name();
    let mut out = format!(
        "# {name}\n\n> {}\n\n- Version: {}\n- Framework: Suprnova {}\n- Whole site content: {base}/llms-full.txt\n\n## Documentation\n\n",
        site_description(),
        env!("CARGO_PKG_VERSION"),
        suprnova::VERSION,
    );
    for chapter in load_catalog()?.chapters {
        out.push_str(&format!(
            "- [{}]({base}/docs/{}.md): {}\n",
            chapter.title, chapter.slug, chapter.excerpt
        ));
    }

    let articles = Article::published().await?;
    if !articles.is_empty() {
        out.push_str("\n## Blog\n\n");
        for article in articles {
            out.push_str(&format!(
                "- [{}]({base}/blog/{}.md): {}\n",
                article.title, article.slug, article.excerpt
            ));
        }
    }
    out.push_str(&format!(
        "\n## Optional\n\n- [Blog index]({base}/blog)\n- [Members]({base}/members)\n- [Topics]({base}/topics)\n"
    ));

    text_response(out, false)
}

#[handler]
pub async fn full(_req: Request) -> Response {
    let base = app_url();
    let name = site_name();
    let mut out = format!(
        "# {name} - complete public content\n\n> {}\n\nIndividual sources are at {base}/docs/<slug>.md and {base}/blog/<slug>.md. The index is at {base}/llms.txt.\n",
        site_description()
    );

    for chapter in load_catalog()?.chapters {
        let Ok(markdown) = chapter_markdown(&chapter.slug) else {
            continue;
        };
        out.push_str(&format!(
            "\n---\n\n<!-- /docs/{} -->\n\n{}\n",
            chapter.slug, markdown
        ));
    }

    let articles = Article::published().await?;
    if !articles.is_empty() {
        out.push_str("\n# Blog\n");
        for article in articles {
            out.push_str(&format!(
                "\n---\n\n<!-- /blog/{} -->\n\n{}\n",
                article.slug, article.body_markdown
            ));
        }
    }

    text_response(out, true)
}

fn text_response(body: String, noindex: bool) -> Response {
    let mut response = HttpResponse::bytes_body(body.into_bytes(), "text/plain; charset=utf-8")
        .header("Cache-Control", "public, max-age=0");
    if noindex {
        response = response.header("X-Robots-Tag", "noindex");
    }
    response.ok()
}
