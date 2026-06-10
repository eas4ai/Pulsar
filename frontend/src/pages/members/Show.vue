<script setup lang="ts">
import { computed } from 'vue'
import type { MemberShowProps } from '../../types/inertia-props'

const props = defineProps<MemberShowProps>()

const profileLinks = computed(() =>
  [
    {
      label: 'Website',
      href: props.member.links.website_url,
      icon: 'mdi-web',
    },
    {
      label: 'GitHub',
      href: props.member.links.github_url,
      icon: 'mdi-github',
    },
  ].filter((link): link is { label: string; href: string; icon: string } =>
    Boolean(link.href),
  ),
)

const profileDetails = computed(() =>
  [
    {
      label: 'Location',
      value: props.member.links.location,
      icon: 'mdi-map-marker-outline',
    },
    {
      label: 'Timezone',
      value: props.member.links.timezone,
      icon: 'mdi-clock-outline',
    },
  ].filter((detail): detail is { label: string; value: string; icon: string } =>
    Boolean(detail.value),
  ),
)

const counts = computed(() => [
  {
    label: 'Articles',
    value: props.member.contribution_counts.articles,
    icon: 'mdi-file-document-outline',
  },
  {
    label: 'Resources',
    value: props.member.contribution_counts.resources,
    icon: 'mdi-library-outline',
  },
  {
    label: 'Answers',
    value: props.member.contribution_counts.answers,
    icon: 'mdi-comment-check-outline',
  },
  {
    label: 'Reputation',
    value: props.member.contribution_counts.reputation,
    icon: 'mdi-star-outline',
  },
])
</script>

<template>
  <v-container class="member-page member-page--show">
    <v-btn to="/members" variant="text" prepend-icon="mdi-arrow-left" class="mb-4">
      Members
    </v-btn>

    <article class="member-profile">
      <header class="member-profile__header">
        <v-avatar color="primary" variant="tonal" size="96">
          <v-img
            v-if="member.avatar_url"
            :src="member.avatar_url"
            :alt="member.display_name"
          />
          <v-icon v-else icon="mdi-account-circle-outline" size="52" />
        </v-avatar>
        <div>
          <div class="console-eyebrow">Member profile</div>
          <h1>{{ member.display_name }}</h1>
          <p class="member-profile__handle">@{{ member.handle }}</p>
        </div>
      </header>

      <p class="member-profile__bio">
        {{ member.bio || 'No bio yet.' }}
      </p>

      <div v-if="profileLinks.length" class="member-profile__links">
        <v-btn
          v-for="link in profileLinks"
          :key="link.href"
          :href="link.href"
          target="_blank"
          rel="noopener noreferrer"
          variant="outlined"
          :prepend-icon="link.icon"
        >
          {{ link.label }}
        </v-btn>
      </div>

      <div v-if="profileDetails.length" class="member-profile__details">
        <span v-for="detail in profileDetails" :key="detail.label">
          <v-icon :icon="detail.icon" size="18" />
          {{ detail.value }}
        </span>
      </div>

      <div class="member-counts">
        <div v-for="count in counts" :key="count.label" class="member-counts__item">
          <v-icon :icon="count.icon" />
          <strong>{{ count.value }}</strong>
          <span>{{ count.label }}</span>
        </div>
      </div>

      <section class="member-badges">
        <div class="member-badges__header">
          <div class="console-eyebrow">Badges</div>
          <span>{{ member.badges.length }}</span>
        </div>
        <div v-if="member.badges.length" class="member-badges__list">
          <v-chip
            v-for="badge in member.badges"
            :key="badge.key"
            color="primary"
            variant="tonal"
            :prepend-icon="badge.icon || 'mdi-medal-outline'"
          >
            {{ badge.name }}
          </v-chip>
        </div>
        <p v-else>No badges yet.</p>
      </section>
    </article>
  </v-container>
</template>
