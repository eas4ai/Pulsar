//! Public member profile controllers.

use serde::Serialize;
use suprnova::eloquent::{Direction, Model};
use suprnova::{FrameworkError, InertiaProps, Request, Response, handler, inertia_response};

use crate::controllers::inertia_config;
use crate::models::profile::Profile;

#[derive(Clone, Debug, Serialize)]
pub struct MemberSummary {
    pub handle: String,
    pub display_name: String,
    pub bio: Option<String>,
    pub avatar_url: Option<String>,
    pub links: MemberLinks,
    pub badges: Vec<MemberBadge>,
    pub contribution_counts: ContributionCounts,
}

#[derive(Clone, Debug, Serialize)]
pub struct MemberLinks {
    pub website_url: Option<String>,
    pub github_url: Option<String>,
    pub location: Option<String>,
    pub timezone: Option<String>,
}

#[derive(Clone, Debug, Serialize)]
pub struct MemberBadge {
    pub key: String,
    pub name: String,
    pub icon: Option<String>,
}

#[derive(Clone, Debug, Serialize)]
pub struct ContributionCounts {
    pub articles: u32,
    pub resources: u32,
    pub answers: u32,
    pub reputation: i32,
}

#[derive(InertiaProps)]
pub struct MembersIndexProps {
    pub members: Vec<MemberSummary>,
}

#[derive(InertiaProps)]
pub struct MemberShowProps {
    pub member: MemberSummary,
}

#[handler]
pub async fn index(req: Request) -> Response {
    let members = <Profile as Model>::query()
        .order_by("display_name", Direction::Asc)
        .order_by("id", Direction::Asc)
        .get()
        .await?
        .into_vec()
        .into_iter()
        .map(summary)
        .collect::<Vec<_>>();

    inertia_response!(
        &req,
        "members/Index",
        MembersIndexProps { members },
        inertia_config()
    )
}

#[handler]
pub async fn show(req: Request) -> Response {
    let handle = req
        .param("handle")
        .map_err(|_| FrameworkError::param("handle"))?;
    let profile = Profile::find_by_handle(handle)
        .await?
        .ok_or_else(|| FrameworkError::not_found(format!("member `{handle}`")))?;

    inertia_response!(
        &req,
        "members/Show",
        MemberShowProps {
            member: summary(profile)
        },
        inertia_config()
    )
}

fn summary(profile: Profile) -> MemberSummary {
    MemberSummary {
        handle: profile.handle,
        display_name: profile.display_name,
        bio: profile.bio,
        avatar_url: profile.avatar_url,
        links: MemberLinks {
            website_url: profile.website_url,
            github_url: profile.github_url,
            location: profile.location,
            timezone: profile.timezone,
        },
        badges: Vec::new(),
        contribution_counts: ContributionCounts {
            articles: 0,
            resources: 0,
            answers: 0,
            reputation: 0,
        },
    }
}
