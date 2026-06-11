<script setup lang="ts">
import { computed, ref } from 'vue'
import { router, useForm } from '@inertiajs/vue3'
import BrandMark from '../../components/BrandMark.vue'
import type { VerifyEmailProps } from '../../types/inertia-props'

const props = defineProps<VerifyEmailProps>()

const statusAlerts: Record<string, { title: string }> = {
  'invalid-or-expired': {
    title:
      'That verification link is invalid or has expired. Request a fresh one below.',
  },
}

const statusAlert = computed(() =>
  props.status ? (statusAlerts[props.status] ?? null) : null,
)
const resendForm = useForm({})
const linkSent = ref(false)

function resend() {
  linkSent.value = false
  resendForm.post('/email/verification-notification', {
    preserveState: true,
    onSuccess: () => {
      linkSent.value = true
    },
  })
}

function logout() {
  router.post('/logout')
}
</script>

<template>
  <v-container class="auth-page">
    <v-card class="auth-card">
      <v-card-item class="text-center pt-6">
        <BrandMark />
        <v-card-title class="mt-3">Verify your email address</v-card-title>
        <v-card-subtitle class="text-wrap">
          Before continuing, please check your inbox for a verification link.
          If you didn't receive the email, we'll gladly send you another.
        </v-card-subtitle>
      </v-card-item>

      <v-card-text>
        <v-alert
          v-if="statusAlert"
          type="error"
          variant="tonal"
          icon="mdi-link-variant-off"
          :title="statusAlert.title"
          class="mb-4"
        />
        <v-alert
          v-else-if="linkSent"
          type="success"
          variant="tonal"
          icon="mdi-email-check"
          title="A fresh verification link has been sent to your email address."
          class="mb-4"
        />

        <v-form @submit.prevent="resend">
          <v-btn type="submit" color="primary" block :loading="resendForm.processing">
            Resend verification email
          </v-btn>
        </v-form>
      </v-card-text>

      <v-card-actions class="auth-card__links justify-center pb-4">
        <v-btn variant="text" size="small" @click="logout">Log out</v-btn>
      </v-card-actions>
    </v-card>
  </v-container>
</template>
