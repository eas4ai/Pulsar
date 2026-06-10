//! Public taxonomy page controllers.

use serde::Serialize;
use suprnova::eloquent::{Direction, Model};
use suprnova::{FrameworkError, InertiaProps, Request, Response, handler, inertia_response};

use crate::controllers::inertia_config;
use crate::models::category::Category;
use crate::models::tag::Tag;
use crate::models::topic::Topic;

#[derive(Clone, Debug, Serialize)]
pub struct TaxonomyListItem {
    pub name: String,
    pub slug: String,
    pub description: Option<String>,
    pub contribution_counts: TaxonomyContributionCounts,
}

#[derive(Clone, Debug, Serialize)]
pub struct TaxonomyDetail {
    pub name: String,
    pub slug: String,
    pub description: Option<String>,
    pub contribution_counts: TaxonomyContributionCounts,
    pub contributions: TaxonomyContributions,
}

#[derive(Clone, Debug, Serialize)]
pub struct TaxonomyContributionCounts {
    pub articles: u32,
    pub resources: u32,
    pub questions: u32,
}

#[derive(Clone, Debug, Serialize)]
pub struct TaxonomyContributions {
    pub articles: Vec<String>,
    pub resources: Vec<String>,
    pub questions: Vec<String>,
}

#[derive(InertiaProps)]
pub struct TopicsIndexProps {
    pub topics: Vec<TaxonomyListItem>,
}

#[derive(InertiaProps)]
pub struct TopicShowProps {
    pub topic: TaxonomyDetail,
}

#[derive(InertiaProps)]
pub struct CategoryShowProps {
    pub category: TaxonomyDetail,
}

#[derive(InertiaProps)]
pub struct TagShowProps {
    pub tag: TaxonomyDetail,
}

#[handler]
pub async fn topics_index(req: Request) -> Response {
    let topics = <Topic as Model>::query()
        .filter("is_visible", true)
        .order_by("sort_order", Direction::Asc)
        .order_by("name", Direction::Asc)
        .order_by("id", Direction::Asc)
        .get()
        .await?
        .into_vec()
        .into_iter()
        .map(topic_list_item)
        .collect::<Vec<_>>();

    inertia_response!(
        &req,
        "topics/Index",
        TopicsIndexProps { topics },
        inertia_config()
    )
}

#[handler]
pub async fn topic_show(req: Request) -> Response {
    let slug = req
        .param("slug")
        .map_err(|_| FrameworkError::param("slug"))?;
    let topic = Topic::find_by_slug(slug)
        .await?
        .filter(|topic| topic.is_visible)
        .ok_or_else(|| FrameworkError::not_found(format!("topic `{slug}`")))?;

    inertia_response!(
        &req,
        "topics/Show",
        TopicShowProps {
            topic: topic_detail(topic)
        },
        inertia_config()
    )
}

#[handler]
pub async fn tag_show(req: Request) -> Response {
    let slug = req
        .param("slug")
        .map_err(|_| FrameworkError::param("slug"))?;
    let tag = Tag::find_by_slug(slug)
        .await?
        .ok_or_else(|| FrameworkError::not_found(format!("tag `{slug}`")))?;

    inertia_response!(
        &req,
        "tags/Show",
        TagShowProps {
            tag: tag_detail(tag)
        },
        inertia_config()
    )
}

#[handler]
pub async fn category_show(req: Request) -> Response {
    let slug = req
        .param("slug")
        .map_err(|_| FrameworkError::param("slug"))?;
    let category = Category::find_by_slug(slug)
        .await?
        .filter(|category| category.is_visible)
        .ok_or_else(|| FrameworkError::not_found(format!("category `{slug}`")))?;

    inertia_response!(
        &req,
        "categories/Show",
        CategoryShowProps {
            category: category_detail(category)
        },
        inertia_config()
    )
}

fn topic_list_item(topic: Topic) -> TaxonomyListItem {
    TaxonomyListItem {
        name: topic.name,
        slug: topic.slug,
        description: topic.description,
        contribution_counts: empty_counts(),
    }
}

fn topic_detail(topic: Topic) -> TaxonomyDetail {
    TaxonomyDetail {
        name: topic.name,
        slug: topic.slug,
        description: topic.description,
        contribution_counts: empty_counts(),
        contributions: empty_contributions(),
    }
}

fn category_detail(category: Category) -> TaxonomyDetail {
    TaxonomyDetail {
        name: category.name,
        slug: category.slug,
        description: category.description,
        contribution_counts: empty_counts(),
        contributions: empty_contributions(),
    }
}

fn tag_detail(tag: Tag) -> TaxonomyDetail {
    TaxonomyDetail {
        name: tag.name,
        slug: tag.slug,
        description: None,
        contribution_counts: empty_counts(),
        contributions: empty_contributions(),
    }
}

fn empty_counts() -> TaxonomyContributionCounts {
    TaxonomyContributionCounts {
        articles: 0,
        resources: 0,
        questions: 0,
    }
}

fn empty_contributions() -> TaxonomyContributions {
    TaxonomyContributions {
        articles: Vec::new(),
        resources: Vec::new(),
        questions: Vec::new(),
    }
}
