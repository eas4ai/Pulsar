# Community and Admin

Pulsar is meant to become a framework community hub, so it includes public community surfaces and role-gated admin routes.

## Public Community Routes

- `/members` lists public profiles.
- `/members/{handle}` shows a profile.
- `/topics` lists topics.
- `/topics/{slug}` shows one topic.
- `/tags/{slug}` shows one tag.
- `/categories/{slug}` shows one category.

Profiles are created from account data and edited at `/profile`.

## Admin Routes

Admin and moderation routes are protected by authentication, email verification, and permissions.

- `/admin/articles` for article publishing.
- `/admin/taxonomy` for categories, topics, and tags.
- `/admin/users` for user management.
- `/moderation` for review workflows.

Some routes are placeholders for the broader V2 community plan, but the route and permission seams are already in place.

## Permissions

Routes use `PermissionMiddleware::<User>::new("permission.name")`. Current permission examples include:

- `articles.create`
- `articles.update`
- `articles.publish`
- `taxonomy.manage`
- `users.manage`
- `moderation.review`

Use permission names that describe capabilities, not job titles. Roles can then collect those capabilities.

## Profiles

Public profiles include handle, display name, bio, avatar URL, website, GitHub URL, location, and timezone. Contribution counts and badges are wired as display fields so downstream apps can fill them with real activity.

Continue with [Deployment](deployment.md).
