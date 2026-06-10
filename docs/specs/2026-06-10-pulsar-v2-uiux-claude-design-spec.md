# Pulsar V2 UI/UX Spec for Claude Design

## Design Goal

Design Pulsar V2 as an unbranded framework ecosystem hub and community starter kit. It should be suitable for a downstream branded site such as `suprnova.app`, but the base design must remain framework-neutral.

The product should feel like a serious developer platform with a welcoming community layer. It must reduce intimidation for people who are new to Rust while still feeling credible to experienced systems developers.

## Design Principles

- Community-first, not corporate SaaS.
- Serious enough for expert developers; warm enough for beginners.
- Contribution-led reputation, not currency-led gamification.
- Dense where operational, spacious where educational.
- Clear role boundaries in dashboards.
- Every empty state should invite a next contribution.
- Avoid generic marketing-site design, excessive gradients, decorative blobs, and novelty UI.
- Do not use CI as a visible product concept; Pulsar is a starter kit.

## Visual Tone

Recommended tone:

- Technical, clean, grounded.
- Neutral base with restrained teal/amber/blue accents.
- Strong typography and high readability.
- Cards only for actual repeated entities or dashboard modules.
- Avoid childish gamification. Badges should feel like contribution credentials, not prizes.

The design should not look like a clone of any existing framework site. It should borrow the idea of a complete framework community hub, not a visual style.

## Page Taxonomy

### Public Marketing

- Home
- Framework overview
- Features
- Getting started
- Community landing
- Showcase landing
- Contribute landing

### Learning

- Docs index
- Docs chapter
- Guides index
- Guide detail
- Tutorials index
- Tutorial detail
- Learning path index
- Learning path detail
- Changelog / releases index
- Release detail
- Upgrade guide

### Community Publishing

- Posts/articles index
- Post/article detail
- Submit post
- Draft editor
- Preview post
- Author profile content tab
- Topic detail
- Tag detail

### Questions and Answers

- Questions index
- Latest questions
- Unanswered questions
- Unsolved questions
- Solved questions
- Ask question
- Question detail
- Edit question
- Answer composer
- Accepted answer state

### Comments and Replies

- Inline comment thread
- Reply composer
- Comment moderation state
- Hidden/removed comment state

### Ecosystem Directory

- Resources index
- Category detail
- Resource type index
- Resource detail
- Submit resource
- Edit submitted resource
- Official packages index
- Package detail
- Community packages index
- Templates index
- Template detail
- Provider integrations index
- Provider detail
- Tools index
- Tool detail
- Example apps index
- Example app detail
- Showcase projects index
- Showcase project detail
- External resources index

### Members and Profiles

- Members index
- Public profile
- Profile posts tab
- Profile answers tab
- Profile resources tab
- Profile showcase tab
- Profile badges/contributions tab

### Authenticated User

- User dashboard
- Profile settings
- Account/security
- Notifications
- Bookmarks
- My questions
- My answers
- My drafts
- My submissions
- Contribution history
- Onboarding checklist

### Contributor / Author

- Contributor dashboard
- Content dashboard
- Drafts
- Submitted posts
- Review feedback
- Revision history
- Scheduled publishing
- Resource submission management
- Featured contribution detail

### Moderator

- Moderator dashboard
- Content review queue
- Resource review queue
- Question/report queue
- Comment report queue
- User report queue
- Moderation decision detail
- Spam review
- Recent decisions

### Admin

- Admin dashboard
- Users
- User detail
- Roles and permissions
- Taxonomy manager
- Category manager
- Topic manager
- Tag manager
- Content manager
- Resource manager
- Question manager
- Release/changelog manager
- Site settings
- Seed/demo tools
- System health / setup state

### Search and Discovery

- Global search overlay
- Search results
- Topic index
- Tag index
- Category index
- RSS/feed discovery

### System States

- Login
- Register
- Verify email
- Forgot password
- Reset password
- 403
- 404
- Empty state
- Loading state
- Error state
- Maintenance/setup state

## Navigation Model

Primary public navigation:

- Docs
- Learn
- Questions
- Resources
- Showcase
- Community

Secondary authenticated navigation:

- Dashboard
- My contributions
- Bookmarks
- Notifications
- Profile

Role-specific navigation should appear only when relevant:

- Contributor tools
- Moderator queues
- Admin

Mobile navigation must collapse predictably and preserve search, auth, and primary sections.

## Dashboard UX

### Member Dashboard

Purpose: make the next helpful action obvious.

Modules:

- Onboarding checklist
- Profile completion
- Ask a question
- Submit a resource
- Bookmarks
- Recent notifications
- Contribution summary
- Suggested beginner-friendly tasks

### Contributor Dashboard

Purpose: manage contributions without needing admin UI.

Modules:

- Drafts
- Submitted posts/resources
- Reviewer feedback
- Accepted answers
- Featured resources
- Suggested next contribution
- Personal contribution timeline

### Author Dashboard

Purpose: editorial workbench.

Modules:

- Content pipeline
- Scheduled posts
- Revisions
- Needs update
- Published content
- Assigned reviews

### Moderator Dashboard

Purpose: fast queue triage.

Modules:

- Reports queue
- Submitted content queue
- Submitted resource queue
- Flagged comments
- Flagged users
- Recent decisions
- Beginner questions needing attention

### Admin Dashboard

Purpose: site operations.

Modules:

- User growth summary
- Pending moderation summary
- Content health
- Resource health
- Taxonomy shortcuts
- Role management
- Site settings
- Demo seed/reset controls

## Key UX Flows

### New Member Onboarding

Register, verify email, complete profile, choose interests/topics, see beginner-safe next steps, bookmark docs, ask first question or submit first resource.

### Ask and Resolve Question

Ask question, assign tags, receive answers, comment for clarification, accept answer, update solved state, reward answer contributor.

### Submit Resource

Choose resource type, enter common fields, fill type-specific metadata, preview detail page, submit for review, track status from dashboard.

### Publish Article

Draft, preview, autosave, submit for review, receive feedback, revise, approve, schedule or publish.

### Moderate Content

Open queue, inspect target, view reporter context and author history, choose decision, add note, notify affected user.

## Component Inventory

Core layout:

- App shell
- Public top navigation
- Mobile navigation drawer
- Dashboard sidebar
- Breadcrumbs
- Global search overlay

Content:

- Article card
- Question card
- Resource card
- Showcase card
- Release note item
- Tag pill
- Topic link
- Author byline
- Markdown renderer
- Code block
- Table of contents

Forms:

- Markdown editor
- Preview pane
- Resource type selector
- Tag/category selector
- Status selector
- Review decision form
- Comment composer
- Answer composer

Dashboards:

- Queue table
- Status board
- Contribution timeline
- Notification list
- Onboarding checklist
- Role badge
- Stats strip

Moderation:

- Report card
- Decision timeline
- Target preview
- Moderator note
- Severity/status chips

States:

- Empty state with next action
- Loading skeletons
- Inline validation errors
- Permission denied state
- Archived/hidden content state

## Responsive Requirements

- Mobile public pages should prioritize search and top-level sections.
- Mobile dashboards should collapse sidebars into a drawer.
- Tables should become stacked rows or filtered lists.
- Editors should use tabbed edit/preview on narrow screens.
- Cards must not resize unpredictably when tags/status labels wrap.
- Buttons must keep text readable at 320px width.

## Accessibility Requirements

- Keyboard navigable menus and queues.
- Visible focus styles.
- Form labels above inputs.
- Error text below inputs.
- ARIA labels for icon-only controls.
- Sufficient contrast in light and dark surfaces.
- No important state communicated by color alone.

## Content Voice

Tone should be direct, helpful, and low-pressure.

Preferred language:

- “Ask a question”
- “Submit a resource”
- “Help someone”
- “Review queue”
- “Good first contribution”
- “Beginner-friendly”

Avoid:

- currency metaphors
- hype language
- shame around beginner questions
- overpromising production readiness

## Claude Design Deliverables

Claude Design should produce:

- Design system tokens
- Public IA and navigation mockups
- Role dashboard layouts
- Resource directory layouts
- Q&A layouts
- Publishing/editor layouts
- Moderation/admin layouts
- Responsive mobile variants
- Component inventory with states
- Empty/loading/error state examples

The design system must be reusable for downstream branding. Brand-specific logos, copy, and color names should be easy to replace.
