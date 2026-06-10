//! Authenticated article authoring controllers.

use chrono::Utc;
use serde::{Deserialize, Serialize};
use suprnova::eloquent::Model;
use suprnova::{
    Auth, FormRequest, FrameworkError, InertiaProps, Redirect, Request, Response, Validate,
    ValidationErrors, handler, inertia_response,
};

use crate::content::articles::parse_tag_input;
use crate::controllers::{
    FormFailure, InertiaCtx, errors_json, inertia_config, inertia_form, validation_failure,
};
use crate::models::article::{Article, ArticleUpdate, NewArticle};
use crate::models::user::User;

#[derive(Clone, Debug, Serialize)]
pub struct AdminArticleRow {
    pub id: i64,
    pub title: String,
    pub slug: String,
    pub status: String,
    pub category: String,
    pub tags: Vec<String>,
    pub updated_at: String,
    pub published_at: Option<String>,
}

#[derive(Clone, Debug, Serialize)]
pub struct ArticleFormState {
    pub id: Option<i64>,
    pub title: String,
    pub slug: String,
    pub category: String,
    pub tags: String,
    pub status: String,
    pub body_markdown: String,
    pub body_html: String,
    pub has_code: bool,
    pub has_math: bool,
}

#[derive(InertiaProps)]
pub struct AdminArticlesIndexProps {
    pub articles: Vec<AdminArticleRow>,
}

#[derive(InertiaProps)]
pub struct AdminArticleEditProps {
    pub article: ArticleFormState,
    pub errors: suprnova::serde_json::Value,
}

#[derive(Deserialize, Validate)]
pub struct ArticleFormRequest {
    #[validate(length(min = 1, message = "Title is required."))]
    pub title: String,
    pub slug: String,
    #[validate(length(min = 1, message = "Category is required."))]
    pub category: String,
    pub tags: String,
    pub status: String,
    #[validate(length(min = 1, message = "Markdown body is required."))]
    pub body_markdown: String,
}

impl FormRequest for ArticleFormRequest {
    fn after_validation(&self) -> Result<(), ValidationErrors> {
        if !matches!(self.status.as_str(), "draft" | "published") {
            let mut errors = ValidationErrors::new();
            errors.add("status", "Status must be draft or published.");
            return Err(errors);
        }
        Ok(())
    }
}

#[handler]
pub async fn index(req: Request) -> Response {
    let articles = Article::all_for_admin()
        .await?
        .into_iter()
        .map(admin_row)
        .collect::<Vec<_>>();
    inertia_response!(
        &req,
        "admin/articles/Index",
        AdminArticlesIndexProps { articles },
        inertia_config()
    )
}

#[handler]
pub async fn create(req: Request) -> Response {
    inertia_response!(
        &req,
        "admin/articles/Edit",
        AdminArticleEditProps {
            article: empty_form(),
            errors: suprnova::serde_json::json!({}),
        },
        inertia_config()
    )
}

#[handler]
pub async fn store(req: Request) -> Response {
    let ctx = InertiaCtx::of(&req);
    let form = match inertia_form::<ArticleFormRequest>(req).await {
        Ok(form) => form,
        Err(FormFailure::Invalid(form, errors)) => {
            return render_edit(&ctx, form_state_from_form(None, &form), errors).await;
        }
        Err(FormFailure::Response(resp)) => return Err(*resp),
    };

    let user = current_user().await?;
    let article = Article::create_from_markdown(NewArticle {
        title: form.title,
        slug: form.slug,
        body_markdown: form.body_markdown,
        author_id: user.id,
        status: form.status,
        source: "first_party".to_string(),
        category: form.category,
        tags: parse_tag_input(&form.tags),
        published_at: None,
    })
    .await?;

    Redirect::to(format!("/admin/articles/{}/edit", article.id)).into()
}

#[handler]
pub async fn edit(req: Request) -> Response {
    let article = find_article(&req).await?;
    inertia_response!(
        &req,
        "admin/articles/Edit",
        AdminArticleEditProps {
            article: form_state(article),
            errors: suprnova::serde_json::json!({}),
        },
        inertia_config()
    )
}

#[handler]
pub async fn update(req: Request) -> Response {
    let ctx = InertiaCtx::of(&req);
    let id = article_id(&req)?;
    let form = match inertia_form::<ArticleFormRequest>(req).await {
        Ok(form) => form,
        Err(FormFailure::Invalid(form, errors)) => {
            return render_edit(&ctx, form_state_from_form(Some(id), &form), errors).await;
        }
        Err(FormFailure::Response(resp)) => return Err(*resp),
    };

    let mut article = Article::find(id)
        .await?
        .ok_or_else(|| FrameworkError::not_found(format!("article `{id}`")))?;
    article
        .update_from_markdown(ArticleUpdate {
            title: form.title,
            slug: form.slug,
            body_markdown: form.body_markdown,
            status: form.status,
            category: form.category,
            tags: parse_tag_input(&form.tags),
        })
        .await?;

    Redirect::to(format!("/admin/articles/{id}/edit")).into()
}

#[handler]
pub async fn publish(req: Request) -> Response {
    let id = article_id(&req)?;
    let mut article = Article::find(id)
        .await?
        .ok_or_else(|| FrameworkError::not_found(format!("article `{id}`")))?;
    if article.published_at.is_none() {
        article.published_at = Some(Utc::now());
    }
    article.status = "published".to_string();
    Model::save(&article).await?;

    Redirect::to(format!("/admin/articles/{id}/edit")).into()
}

async fn current_user() -> Result<User, FrameworkError> {
    Auth::user_as::<User>()
        .await?
        .ok_or(FrameworkError::Unauthorized)
}

async fn find_article(req: &Request) -> Result<Article, FrameworkError> {
    let id = article_id(req)?;
    Article::find(id)
        .await?
        .ok_or_else(|| FrameworkError::not_found(format!("article `{id}`")))
}

fn article_id(req: &Request) -> Result<i64, FrameworkError> {
    req.param("id")
        .map_err(|_| FrameworkError::param("id"))?
        .parse::<i64>()
        .map_err(|_| FrameworkError::param("id"))
}

async fn render_edit(
    ctx: &InertiaCtx,
    article: ArticleFormState,
    errors: ValidationErrors,
) -> Response {
    if ctx.wants_inertia() {
        inertia_response!(
            ctx,
            "admin/articles/Edit",
            {
                "article": article,
                "errors": errors_json(&errors),
            },
            inertia_config()
        )
    } else {
        Err(validation_failure(errors))
    }
}

fn admin_row(article: Article) -> AdminArticleRow {
    let tags = article.tags_vec();
    AdminArticleRow {
        id: article.id,
        title: article.title,
        slug: article.slug,
        status: article.status,
        category: article.category,
        tags,
        updated_at: article.updated_at.to_rfc3339(),
        published_at: article.published_at.map(|dt| dt.to_rfc3339()),
    }
}

fn empty_form() -> ArticleFormState {
    ArticleFormState {
        id: None,
        title: String::new(),
        slug: String::new(),
        category: "Engineering".to_string(),
        tags: String::new(),
        status: "draft".to_string(),
        body_markdown: String::new(),
        body_html: String::new(),
        has_code: false,
        has_math: false,
    }
}

fn form_state(article: Article) -> ArticleFormState {
    let tags = article.tags_vec().join(", ");
    ArticleFormState {
        id: Some(article.id),
        title: article.title,
        slug: article.slug,
        category: article.category,
        tags,
        status: article.status,
        body_markdown: article.body_markdown,
        body_html: article.body_html,
        has_code: article.has_code,
        has_math: article.has_math,
    }
}

fn form_state_from_form(id: Option<i64>, form: &ArticleFormRequest) -> ArticleFormState {
    ArticleFormState {
        id,
        title: form.title.clone(),
        slug: form.slug.clone(),
        category: form.category.clone(),
        tags: form.tags.clone(),
        status: form.status.clone(),
        body_markdown: form.body_markdown.clone(),
        body_html: String::new(),
        has_code: false,
        has_math: false,
    }
}
