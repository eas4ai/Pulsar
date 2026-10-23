<script setup lang="ts">
import { Form, Head, usePage } from '@inertiajs/vue3'
import type { ClaimProps } from '../../types/inertia-props'

defineProps<ClaimProps>()
const page = usePage()
</script>

<template>
  <Head title="Let an agent act for you?" />
  <v-container class="py-8" style="max-width: 720px">
    <v-card class="pa-6">
      <h1 class="text-h5 mb-4">Let an agent act for you?</h1>
      <p v-if="claim.provider" class="mb-4">An agent signed in through <bdi>{{ claim.provider }}</bdi> asks to act as you.</p>
      <p class="mb-4">You are signed in as <bdi>{{ claim.signed_in_as }}</bdi>.<template v-if="claim.email"> The agent named <bdi>{{ claim.email }}</bdi>.</template></p>
      <v-alert v-if="claim.first_time_account || claim.first_time_provider" type="warning" variant="tonal" class="mb-4">This is a new agent connection. Allow it only if you asked an agent to connect.</v-alert>
      <p class="mb-4">If you allow it, the agent can use the permissions below until you revoke it.</p>
      <p v-if="claim.scopes.length === 0" class="mb-4">It asks for no permissions: it can use only what needs none.</p>
      <ul v-else class="pl-6 mb-4">
        <li v-for="scope in claim.scopes" :key="scope.name">{{ scope.description }} (<code>{{ scope.name }}</code>)</li>
      </ul>
      <Form :action="`${page.props.root}/agent/claim`" method="post" :reset-on-error="['user_code']" v-slot="{ errors, processing }">
        <input type="hidden" name="claim_attempt_token" :value="claim_attempt_token" />
        <p class="mb-4">Enter the six-digit code the agent gave you. It works for about {{ Math.ceil(claim.expires_in_seconds / 60) }} more minutes, {{ claim.attempts_remaining }} wrong entries allowed.</p>
        <v-text-field name="user_code" label="Code" inputmode="numeric" autocomplete="one-time-code" maxlength="9" required :error-messages="errors.user_code" />
        <v-btn type="submit" color="primary" :disabled="processing">Allow</v-btn>
      </Form>
      <p class="mt-4">If you did not ask an agent to connect, close this page.</p>
    </v-card>
  </v-container>
</template>
