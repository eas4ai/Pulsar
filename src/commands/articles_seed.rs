//! Seed sample first-party articles.

use chrono::{Duration, Utc};
use sea_orm_migration::MigratorTrait;
use suprnova::eloquent::Model;
use suprnova::{Command, DB, FrameworkError, HasRoles, TypedCommand};

use crate::commands::users_promote::seed_default_roles;
use crate::migrations::Migrator;
use crate::models::article::{Article, NewArticle};
use crate::models::user::User;

#[derive(clap::Parser, Command, Debug)]
#[console(name = "articles:seed", description = "Seed Pulsar sample articles")]
pub struct ArticlesSeedCommand;

#[suprnova::__async_trait]
impl TypedCommand for ArticlesSeedCommand {
    async fn run(self) -> Result<(), FrameworkError> {
        seed_articles().await?;
        println!("Seeded Pulsar sample articles.");
        Ok(())
    }
}

pub async fn seed_articles() -> Result<(), FrameworkError> {
    let conn = DB::connection()?;
    Migrator::up(conn.inner(), None).await?;

    seed_default_roles().await?;
    let mut author = match User::find_by_email("editor@pulsar.test").await? {
        Some(user) => user,
        None => User::create("Pulsar Editor", "editor@pulsar.test", "secretpass").await?,
    };
    if author.email_verified_at.is_none() {
        author.email_verified_at = Some(Utc::now());
        Model::save(&author).await?;
    }
    author.assign_role("author").await?;

    for sample in sample_articles(author.id) {
        if Article::find_by_slug(&sample.slug).await?.is_none() {
            Article::create_from_markdown(sample).await?;
        }
    }

    Ok(())
}

fn sample_articles(author_id: i64) -> Vec<NewArticle> {
    vec![
        NewArticle {
            title: "Pulsar v1 Publishing".to_string(),
            slug: "pulsar-v1-publishing".to_string(),
            body_markdown: r#"# Pulsar v1 Publishing

Pulsar now includes first-party articles with rendered Markdown, RSS, and author-protected editing.

```rust
rtk cargo run --bin console -- articles:seed
```

Authors can publish release notes without leaving the app shell."#
                .to_string(),
            author_id,
            status: "published".to_string(),
            source: "first_party".to_string(),
            category: "Release".to_string(),
            tags: vec!["release".to_string(), "articles".to_string()],
            published_at: Some(Utc::now() - Duration::days(1)),
        },
        NewArticle {
            title: "Building With Suprnova Content".to_string(),
            slug: "building-with-suprnova-content".to_string(),
            body_markdown: r#"# Building With Suprnova Content

The same renderer powers docs and articles, keeping highlighting, headings, and excerpts consistent.

Math markers like $E=mc^2$ survive rendering for layouts that need to detect technical content."#
                .to_string(),
            author_id,
            status: "published".to_string(),
            source: "first_party".to_string(),
            category: "Engineering".to_string(),
            tags: vec!["suprnova".to_string(), "markdown".to_string()],
            published_at: Some(Utc::now() - Duration::days(2)),
        },
        NewArticle {
            title: "Draft Editorial Workflow".to_string(),
            slug: "draft-editorial-workflow".to_string(),
            body_markdown: "# Draft Editorial Workflow\n\nDrafts stay out of `/blog` and `/feed.xml` until an author publishes them.".to_string(),
            author_id,
            status: "draft".to_string(),
            source: "first_party".to_string(),
            category: "Workflow".to_string(),
            tags: vec!["drafts".to_string()],
            published_at: None,
        },
    ]
}
