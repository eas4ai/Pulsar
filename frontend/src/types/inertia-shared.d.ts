type Value = Record<string, string[]>

interface UserInfo {
  id: number
  name: string
  email: string
}

interface LandingFeature {
  icon: string
  title: string
  body: string
}

interface LandingCapability {
  icon: string
  title: string
  body: string
}

interface LandingCodeSample {
  language: string
  code: string
}

interface DashboardCard {
  title: string
  value: string
  helper: string
  icon: string
}

interface DashboardAction {
  title: string
  body: string
  href: string
  icon: string
}

interface AccountStatus {
  email_verified: boolean
}

interface ProfileFormProps {
  handle: string
  display_name: string
  bio: string | null
  avatar_url: string | null
  website_url: string | null
  github_url: string | null
  location: string | null
  timezone: string | null
}

type ProfileFormState = ProfileFormProps

interface DocsCatalog {
  chapters: DocsCatalogEntry[]
  search: DocsSearchEntry[]
}

interface DocsCatalogEntry {
  slug: string
  title: string
  excerpt: string
  headings: DocsHeading[]
  previous: string | null
  next: string | null
}

interface DocsSearchEntry {
  slug: string
  title: string
  excerpt: string
  headings: DocsHeading[]
  plain_text: string
}

interface DocsChapter {
  slug: string
  title: string
  html: string
  excerpt: string
  headings: DocsHeading[]
  previous: string | null
  next: string | null
}

interface DocsHeading {
  level: number
  id: string
  title: string
}

interface ArticleSummary {
  title: string
  slug: string
  excerpt: string
  category: string
  tags: string[]
  published_at: string
  has_code: boolean
  has_math: boolean
}

interface ArticleDetail extends ArticleSummary {
  body_html: string
}

interface MemberSummary {
  handle: string
  display_name: string
  bio: string | null
  avatar_url: string | null
  links: MemberLinks
  badges: MemberBadge[]
  contribution_counts: ContributionCounts
}

interface MemberLinks {
  website_url: string | null
  github_url: string | null
  location: string | null
  timezone: string | null
}

interface MemberBadge {
  key: string
  name: string
  icon: string | null
}

interface ContributionCounts {
  articles: number
  resources: number
  answers: number
  reputation: number
}

interface TaxonomyListItem {
  name: string
  slug: string
  description: string | null
  contribution_counts: TaxonomyContributionCounts
}

interface TaxonomyDetail extends TaxonomyListItem {
  contributions: TaxonomyContributions
}

interface TaxonomyContributionCounts {
  articles: number
  resources: number
  questions: number
}

interface TaxonomyContributions {
  articles: string[]
  resources: string[]
  questions: string[]
}

interface AdminArticleRow {
  id: number
  title: string
  slug: string
  status: string
  category: string
  tags: string[]
  updated_at: string
  published_at: string | null
}

interface ArticleFormState {
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

interface AdminTaxonomyTermRow {
  id: number
  name: string
  slug: string
  description: string | null
  sort_order: number
  is_visible: boolean
}

interface AdminTagRow {
  id: number
  name: string
  slug: string
}
