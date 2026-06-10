<script setup lang="ts">
defineProps<{
  authenticated: boolean
}>()
</script>

<template>
  <section class="cta-band">
    <div>
      <div class="landing-section__eyebrow">Next step</div>
      <h2>Start from the kit, not another blank app.</h2>
      <p>
        Keep the product surface consistent while the backend keeps owning routes,
        middleware, sessions, and static assets.
      </p>
    </div>
    <div class="cta-band__actions">
      <v-btn
        v-if="authenticated"
        to="/dashboard"
        color="primary"
        size="large"
        prepend-icon="mdi-view-dashboard-outline"
      >
        Go to dashboard
      </v-btn>
      <template v-else>
        <v-btn
          to="/register"
          color="primary"
          size="large"
          prepend-icon="mdi-account-plus-outline"
        >
          Create account
        </v-btn>
        <v-btn
          to="/login"
          variant="outlined"
          size="large"
          prepend-icon="mdi-login"
        >
          Sign in
        </v-btn>
      </template>
    </div>
  </section>
</template>
