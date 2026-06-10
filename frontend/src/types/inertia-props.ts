export interface UserInfo {
  id: number
  name: string
  email: string
}

export interface HomeProps {
  headline: string
  subheadline: string
  features: LandingFeature[]
  metrics: LandingMetric[]
  sample: LandingCodeSample
}

export interface LandingFeature {
  icon: string
  title: string
  body: string
}

export interface LandingMetric {
  value: string
  label: string
}

export interface LandingCodeSample {
  language: string
  code: string
}

export interface LoginProps {}

export interface RegisterProps {}

export interface DashboardProps {
  user: UserInfo
  cards: DashboardCard[]
  actions: DashboardAction[]
  account: AccountStatus
}

export interface DashboardCard {
  title: string
  value: string
  helper: string
  icon: string
}

export interface DashboardAction {
  title: string
  body: string
  href: string
  icon: string
}

export interface AccountStatus {
  email_verified: boolean
}

export interface ProfileProps {
  name: string
  email: string
  email_verified: boolean
  profile: ProfileFormState
}

export interface ProfileFormState {
  handle: string
  display_name: string
  bio: string | null
  avatar_url: string | null
  website_url: string | null
  github_url: string | null
  location: string | null
  timezone: string | null
}

export interface ResetPasswordProps {
  token: string
}

export interface VerifyEmailProps {
  status: string | null
}

export interface DocsProps {
  catalog: DocsCatalog
  chapter: DocsChapter
}

export interface DocsCatalog {
  chapters: DocsCatalogEntry[]
  search: DocsSearchEntry[]
}

export interface DocsCatalogEntry {
  slug: string
  title: string
  excerpt: string
  headings: DocsHeading[]
  previous: string | null
  next: string | null
}

export interface DocsSearchEntry {
  slug: string
  title: string
  excerpt: string
  headings: DocsHeading[]
  plain_text: string
}

export interface DocsChapter {
  slug: string
  title: string
  html: string
  excerpt: string
  headings: DocsHeading[]
  previous: string | null
  next: string | null
}

export interface DocsHeading {
  level: number
  id: string
  title: string
}

export interface ArticlesIndexProps {
  articles: ArticleSummary[]
}

export interface ArticleShowProps {
  article: ArticleDetail
}

export interface ArticleSummary {
  title: string
  slug: string
  excerpt: string
  category: string
  tags: string[]
  published_at: string
  has_code: boolean
  has_math: boolean
}

export interface ArticleDetail extends ArticleSummary {
  body_html: string
}

export interface MembersIndexProps {
  members: MemberSummary[]
}

export interface MemberShowProps {
  member: MemberSummary
}

export interface MemberSummary {
  handle: string
  display_name: string
  bio: string | null
  avatar_url: string | null
  links: MemberLinks
  badges: MemberBadge[]
  contribution_counts: ContributionCounts
}

export interface MemberLinks {
  website_url: string | null
  github_url: string | null
  location: string | null
  timezone: string | null
}

export interface MemberBadge {
  key: string
  name: string
  icon: string | null
}

export interface ContributionCounts {
  articles: number
  resources: number
  answers: number
  reputation: number
}

export interface AdminArticlesIndexProps {
  articles: AdminArticleRow[]
}

export interface AdminArticleRow {
  id: number
  title: string
  slug: string
  status: string
  category: string
  tags: string[]
  updated_at: string
  published_at: string | null
}

export interface AdminArticleEditProps {
  article: ArticleFormState
  errors: Record<string, string[]>
}

export interface ArticleFormState {
  id: number | null
  title: string
  slug: string
  category: string
  tags: string
  status: string
  body_markdown: string
  body_html: string
  has_code: boolean
  has_math: boolean
}
