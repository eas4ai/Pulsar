<script setup lang="ts">
import type { MembersIndexProps } from '../../types/inertia-props'

defineProps<MembersIndexProps>()
</script>

<template>
  <v-container class="member-page">
    <header class="console-hero">
      <div>
        <div class="console-eyebrow">Members</div>
        <h1>Community profiles</h1>
        <p>
          Meet the people building, publishing, and reviewing work across Pulsar.
        </p>
      </div>
    </header>

    <div v-if="members.length" class="member-grid">
      <v-card
        v-for="member in members"
        :key="member.handle"
        class="member-card"
        :to="`/members/${member.handle}`"
        variant="flat"
      >
        <v-card-item>
          <template #prepend>
            <v-avatar color="primary" variant="tonal" size="52">
              <v-img
                v-if="member.avatar_url"
                :src="member.avatar_url"
                :alt="member.display_name"
              />
              <v-icon v-else icon="mdi-account-circle-outline" />
            </v-avatar>
          </template>
          <v-card-title>{{ member.display_name }}</v-card-title>
          <v-card-subtitle>@{{ member.handle }}</v-card-subtitle>
        </v-card-item>

        <v-card-text>
          <p class="member-card__bio">
            {{ member.bio || 'No bio yet.' }}
          </p>
          <div class="member-card__stats">
            <span>{{ member.contribution_counts.articles }} articles</span>
            <span>{{ member.contribution_counts.answers }} answers</span>
            <span>{{ member.badges.length }} badges</span>
          </div>
        </v-card-text>
      </v-card>
    </div>

    <v-alert v-else class="mt-6" type="info" variant="tonal">
      No public profiles are available yet.
    </v-alert>
  </v-container>
</template>
