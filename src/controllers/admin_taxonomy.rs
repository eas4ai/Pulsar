//! Authenticated taxonomy management controllers.

use serde::{Deserialize, Serialize};
use suprnova::eloquent::{Direction, Model};
use suprnova::{
    FormRequest, FrameworkError, InertiaProps, Redirect, Request, Response, Validate,
    ValidationErrors, attrs, handler, inertia_response,
};

use crate::controllers::{
    FormFailure, InertiaCtx, errors_json, inertia_config, inertia_form, validation_failure,
};
use crate::models::category::Category;
use crate::models::tag::Tag;
use crate::models::topic::Topic;

const NAME_MAX_LENGTH: usize = 120;
const SLUG_MAX_LENGTH: usize = 140;

#[derive(Clone, Debug, Serialize)]
pub struct AdminTaxonomyTermRow {
    pub id: i64,
    pub name: String,
    pub slug: String,
    pub description: Option<String>,
    pub sort_order: i32,
    pub is_visible: bool,
}

#[derive(Clone, Debug, Serialize)]
pub struct AdminTagRow {
    pub id: i64,
    pub name: String,
    pub slug: String,
}

#[derive(InertiaProps)]
pub struct AdminTaxonomyProps {
    pub categories: Vec<AdminTaxonomyTermRow>,
    pub topics: Vec<AdminTaxonomyTermRow>,
    pub tags: Vec<AdminTagRow>,
    pub errors: suprnova::serde_json::Value,
}

#[derive(Deserialize, Validate)]
pub struct TaxonomyTermFormRequest {
    pub name: String,
    pub slug: String,
    pub description: Option<String>,
    pub sort_order: i32,
    pub is_visible: bool,
}

impl FormRequest for TaxonomyTermFormRequest {}

#[derive(Deserialize, Validate)]
pub struct TagFormRequest {
    pub name: String,
    pub slug: String,
}

impl FormRequest for TagFormRequest {}

struct TaxonomyTermInput {
    name: String,
    slug: String,
    description: Option<String>,
    sort_order: i32,
    is_visible: bool,
}

struct TagInput {
    name: String,
    slug: String,
}

#[handler]
pub async fn index(req: Request) -> Response {
    inertia_response!(
        &req,
        "admin/taxonomy/Index",
        AdminTaxonomyProps {
            categories: category_rows().await?,
            topics: topic_rows().await?,
            tags: tag_rows().await?,
            errors: suprnova::serde_json::json!({}),
        },
        inertia_config()
    )
}

#[handler]
pub async fn store_category(req: Request) -> Response {
    let ctx = InertiaCtx::of(&req);
    let form = match inertia_form::<TaxonomyTermFormRequest>(req).await {
        Ok(form) => form,
        Err(FormFailure::Invalid(_, errors)) => return render_index(&ctx, errors).await,
        Err(FormFailure::Response(resp)) => return Err(*resp),
    };
    let input = match validate_term_form(&form, None, TermKind::Category).await? {
        Ok(input) => input,
        Err(errors) => return render_index(&ctx, errors).await,
    };

    <Category as Model>::create(attrs! {
        name: input.name,
        slug: input.slug,
        description: input.description,
        sort_order: input.sort_order,
        is_visible: input.is_visible,
    })
    .await?;

    Redirect::to("/admin/taxonomy").into()
}

#[handler]
pub async fn update_category(req: Request) -> Response {
    let ctx = InertiaCtx::of(&req);
    let id = id_param(&req)?;
    let form = match inertia_form::<TaxonomyTermFormRequest>(req).await {
        Ok(form) => form,
        Err(FormFailure::Invalid(_, errors)) => return render_index(&ctx, errors).await,
        Err(FormFailure::Response(resp)) => return Err(*resp),
    };
    let input = match validate_term_form(&form, Some(id), TermKind::Category).await? {
        Ok(input) => input,
        Err(errors) => return render_index(&ctx, errors).await,
    };

    let mut category = Category::find(id)
        .await?
        .ok_or_else(|| FrameworkError::not_found(format!("category `{id}`")))?;
    category.name = input.name;
    category.slug = input.slug;
    category.description = input.description;
    category.sort_order = input.sort_order;
    category.is_visible = input.is_visible;
    Model::save(&category).await?;

    Redirect::to("/admin/taxonomy").into()
}

#[handler]
pub async fn store_topic(req: Request) -> Response {
    let ctx = InertiaCtx::of(&req);
    let form = match inertia_form::<TaxonomyTermFormRequest>(req).await {
        Ok(form) => form,
        Err(FormFailure::Invalid(_, errors)) => return render_index(&ctx, errors).await,
        Err(FormFailure::Response(resp)) => return Err(*resp),
    };
    let input = match validate_term_form(&form, None, TermKind::Topic).await? {
        Ok(input) => input,
        Err(errors) => return render_index(&ctx, errors).await,
    };

    <Topic as Model>::create(attrs! {
        name: input.name,
        slug: input.slug,
        description: input.description,
        sort_order: input.sort_order,
        is_visible: input.is_visible,
    })
    .await?;

    Redirect::to("/admin/taxonomy").into()
}

#[handler]
pub async fn update_topic(req: Request) -> Response {
    let ctx = InertiaCtx::of(&req);
    let id = id_param(&req)?;
    let form = match inertia_form::<TaxonomyTermFormRequest>(req).await {
        Ok(form) => form,
        Err(FormFailure::Invalid(_, errors)) => return render_index(&ctx, errors).await,
        Err(FormFailure::Response(resp)) => return Err(*resp),
    };
    let input = match validate_term_form(&form, Some(id), TermKind::Topic).await? {
        Ok(input) => input,
        Err(errors) => return render_index(&ctx, errors).await,
    };

    let mut topic = Topic::find(id)
        .await?
        .ok_or_else(|| FrameworkError::not_found(format!("topic `{id}`")))?;
    topic.name = input.name;
    topic.slug = input.slug;
    topic.description = input.description;
    topic.sort_order = input.sort_order;
    topic.is_visible = input.is_visible;
    Model::save(&topic).await?;

    Redirect::to("/admin/taxonomy").into()
}

#[handler]
pub async fn store_tag(req: Request) -> Response {
    let ctx = InertiaCtx::of(&req);
    let form = match inertia_form::<TagFormRequest>(req).await {
        Ok(form) => form,
        Err(FormFailure::Invalid(_, errors)) => return render_index(&ctx, errors).await,
        Err(FormFailure::Response(resp)) => return Err(*resp),
    };
    let input = match validate_tag_form(&form, None).await? {
        Ok(input) => input,
        Err(errors) => return render_index(&ctx, errors).await,
    };

    <Tag as Model>::create(attrs! {
        name: input.name,
        slug: input.slug,
    })
    .await?;

    Redirect::to("/admin/taxonomy").into()
}

#[handler]
pub async fn update_tag(req: Request) -> Response {
    let ctx = InertiaCtx::of(&req);
    let id = id_param(&req)?;
    let form = match inertia_form::<TagFormRequest>(req).await {
        Ok(form) => form,
        Err(FormFailure::Invalid(_, errors)) => return render_index(&ctx, errors).await,
        Err(FormFailure::Response(resp)) => return Err(*resp),
    };
    let input = match validate_tag_form(&form, Some(id)).await? {
        Ok(input) => input,
        Err(errors) => return render_index(&ctx, errors).await,
    };

    let mut tag = Tag::find(id)
        .await?
        .ok_or_else(|| FrameworkError::not_found(format!("tag `{id}`")))?;
    tag.name = input.name;
    tag.slug = input.slug;
    Model::save(&tag).await?;

    Redirect::to("/admin/taxonomy").into()
}

async fn render_index(ctx: &InertiaCtx, errors: ValidationErrors) -> Response {
    if ctx.wants_inertia() {
        inertia_response!(
            ctx,
            "admin/taxonomy/Index",
            {
                "categories": category_rows().await?,
                "topics": topic_rows().await?,
                "tags": tag_rows().await?,
                "errors": errors_json(&errors),
            },
            inertia_config()
        )
    } else {
        Err(validation_failure(errors))
    }
}

async fn category_rows() -> Result<Vec<AdminTaxonomyTermRow>, FrameworkError> {
    Ok(<Category as Model>::query()
        .order_by("sort_order", Direction::Asc)
        .order_by("name", Direction::Asc)
        .order_by("id", Direction::Asc)
        .get()
        .await?
        .into_vec()
        .into_iter()
        .map(category_row)
        .collect())
}

async fn topic_rows() -> Result<Vec<AdminTaxonomyTermRow>, FrameworkError> {
    Ok(<Topic as Model>::query()
        .order_by("sort_order", Direction::Asc)
        .order_by("name", Direction::Asc)
        .order_by("id", Direction::Asc)
        .get()
        .await?
        .into_vec()
        .into_iter()
        .map(topic_row)
        .collect())
}

async fn tag_rows() -> Result<Vec<AdminTagRow>, FrameworkError> {
    Ok(<Tag as Model>::query()
        .order_by("name", Direction::Asc)
        .order_by("id", Direction::Asc)
        .get()
        .await?
        .into_vec()
        .into_iter()
        .map(tag_row)
        .collect())
}

fn category_row(category: Category) -> AdminTaxonomyTermRow {
    AdminTaxonomyTermRow {
        id: category.id,
        name: category.name,
        slug: category.slug,
        description: category.description,
        sort_order: category.sort_order,
        is_visible: category.is_visible,
    }
}

fn topic_row(topic: Topic) -> AdminTaxonomyTermRow {
    AdminTaxonomyTermRow {
        id: topic.id,
        name: topic.name,
        slug: topic.slug,
        description: topic.description,
        sort_order: topic.sort_order,
        is_visible: topic.is_visible,
    }
}

fn tag_row(tag: Tag) -> AdminTagRow {
    AdminTagRow {
        id: tag.id,
        name: tag.name,
        slug: tag.slug,
    }
}

#[derive(Clone, Copy)]
enum TermKind {
    Category,
    Topic,
}

async fn validate_term_form(
    form: &TaxonomyTermFormRequest,
    current_id: Option<i64>,
    kind: TermKind,
) -> Result<Result<TaxonomyTermInput, ValidationErrors>, FrameworkError> {
    let mut errors = ValidationErrors::new();
    let name = validate_required_text("name", "Name", &form.name, NAME_MAX_LENGTH, &mut errors);
    let slug = validate_slug(&form.slug, &mut errors);
    if let Some(slug) = &slug {
        validate_unique_term_slug(kind, slug, current_id, &mut errors).await?;
    }

    if !errors.errors.is_empty() {
        return Ok(Err(errors));
    }

    Ok(Ok(TaxonomyTermInput {
        name: name.expect("validated name exists"),
        slug: slug.expect("validated slug exists"),
        description: normalize_optional(&form.description),
        sort_order: form.sort_order,
        is_visible: form.is_visible,
    }))
}

async fn validate_tag_form(
    form: &TagFormRequest,
    current_id: Option<i64>,
) -> Result<Result<TagInput, ValidationErrors>, FrameworkError> {
    let mut errors = ValidationErrors::new();
    let name = validate_required_text("name", "Name", &form.name, NAME_MAX_LENGTH, &mut errors);
    let slug = validate_slug(&form.slug, &mut errors);
    if let Some(slug) = &slug
        && let Some(existing) = Tag::find_by_slug(slug).await?
        && Some(existing.id) != current_id
    {
        errors.add("slug", "Slug is already in use.");
    }

    if !errors.errors.is_empty() {
        return Ok(Err(errors));
    }

    Ok(Ok(TagInput {
        name: name.expect("validated name exists"),
        slug: slug.expect("validated slug exists"),
    }))
}

async fn validate_unique_term_slug(
    kind: TermKind,
    slug: &str,
    current_id: Option<i64>,
    errors: &mut ValidationErrors,
) -> Result<(), FrameworkError> {
    match kind {
        TermKind::Category => {
            if let Some(existing) = Category::find_by_slug(slug).await?
                && Some(existing.id) != current_id
            {
                errors.add("slug", "Slug is already in use.");
            }
        }
        TermKind::Topic => {
            if let Some(existing) = Topic::find_by_slug(slug).await?
                && Some(existing.id) != current_id
            {
                errors.add("slug", "Slug is already in use.");
            }
        }
    }
    Ok(())
}

fn validate_required_text(
    field: &str,
    label: &str,
    value: &str,
    max_len: usize,
    errors: &mut ValidationErrors,
) -> Option<String> {
    let trimmed = value.trim();
    if trimmed.is_empty() {
        errors.add(field, format!("{label} is required."));
        return None;
    }
    if trimmed.chars().count() > max_len {
        errors.add(
            field,
            format!("{label} must be at most {max_len} characters."),
        );
    }
    Some(trimmed.to_string())
}

fn validate_slug(value: &str, errors: &mut ValidationErrors) -> Option<String> {
    let slug = validate_required_text("slug", "Slug", value, SLUG_MAX_LENGTH, errors)?;
    if !slug
        .chars()
        .all(|ch| ch.is_ascii_lowercase() || ch.is_ascii_digit() || ch == '-')
    {
        errors.add(
            "slug",
            "Slug may only contain lowercase letters, numbers, and hyphens.",
        );
    }
    Some(slug)
}

fn normalize_optional(value: &Option<String>) -> Option<String> {
    value
        .as_deref()
        .map(str::trim)
        .filter(|value| !value.is_empty())
        .map(str::to_string)
}

fn id_param(req: &Request) -> Result<i64, FrameworkError> {
    req.param("id")
        .map_err(|_| FrameworkError::param("id"))?
        .parse::<i64>()
        .map_err(|_| FrameworkError::param("id"))
}
