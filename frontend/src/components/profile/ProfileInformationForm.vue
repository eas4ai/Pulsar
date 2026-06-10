<script setup lang="ts">
import { useForm } from '@inertiajs/vue3'

const props = defineProps<{
  name: string
  email: string
  emailVerified: boolean
}>()

const form = useForm({
  name: props.name,
  email: props.email,
})

function submit() {
  form.patch('/profile')
}
</script>

<template>
  <v-card class="profile-panel">
    <v-card-item>
      <div class="profile-panel__title-row">
        <div>
          <v-card-title>Profile information</v-card-title>
          <v-card-subtitle class="text-wrap">
            Update your name and the email address used for sign-in.
          </v-card-subtitle>
        </div>
        <v-chip
          :color="emailVerified ? 'success' : 'warning'"
          variant="tonal"
          size="small"
        >
          {{ emailVerified ? 'Email verified' : 'Email not verified' }}
        </v-chip>
      </div>
    </v-card-item>

    <v-card-text>
      <v-form @submit.prevent="submit">
        <v-text-field
          v-model="form.name"
          label="Name"
          type="text"
          autocomplete="name"
          :error-messages="form.errors.name"
          required
        />
        <v-text-field
          v-model="form.email"
          label="Email address"
          type="email"
          autocomplete="email"
          hint="Changing your email requires verifying the new address."
          :error-messages="form.errors.email"
          required
        />
        <div class="profile-form-actions">
          <v-btn type="submit" color="primary" :loading="form.processing">
            Save
          </v-btn>
          <span v-if="form.recentlySuccessful" class="text-body-2 text-medium-emphasis">
            Saved.
          </span>
        </div>
      </v-form>
    </v-card-text>
  </v-card>
</template>
