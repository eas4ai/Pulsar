<script setup lang="ts">
import type { DashboardProps } from '../types/inertia-props'

defineProps<DashboardProps>()
</script>

<template>
  <v-container class="console-page">
    <header class="console-hero">
      <div>
        <div class="console-eyebrow">Console</div>
        <h1>Welcome back, {{ user.name }}.</h1>
        <p>
          Track the starter kit's core surfaces and jump into account or public
          page workflows.
        </p>
      </div>
      <v-chip
        :color="account.email_verified ? 'success' : 'warning'"
        variant="tonal"
        prepend-icon="mdi-email-check-outline"
      >
        {{ account.email_verified ? 'Email verified' : 'Email pending' }}
      </v-chip>
    </header>

    <v-row class="mt-6">
      <v-col v-for="card in cards" :key="card.title" cols="12" md="4">
        <article class="dashboard-card">
          <v-icon :icon="card.icon" size="28" />
          <div>
            <div class="dashboard-card__label">{{ card.title }}</div>
            <strong>{{ card.value }}</strong>
            <p>{{ card.helper }}</p>
          </div>
        </article>
      </v-col>
    </v-row>

    <section class="dashboard-section">
      <div class="dashboard-section__header">
        <div>
          <div class="console-eyebrow">Quick actions</div>
          <h2>Keep the kit moving.</h2>
        </div>
      </div>

      <v-row>
        <v-col v-for="action in actions" :key="action.href" cols="12" md="6">
          <v-card class="dashboard-action" variant="outlined">
            <v-card-item>
              <template #prepend>
                <v-avatar color="primary" variant="tonal">
                  <v-icon :icon="action.icon" />
                </v-avatar>
              </template>
              <v-card-title>{{ action.title }}</v-card-title>
              <v-card-subtitle class="text-wrap">{{ action.body }}</v-card-subtitle>
            </v-card-item>
            <v-card-actions>
              <v-btn :to="action.href" color="primary" variant="text" append-icon="mdi-arrow-right">
                Open
              </v-btn>
            </v-card-actions>
          </v-card>
        </v-col>
      </v-row>
    </section>
  </v-container>
</template>
