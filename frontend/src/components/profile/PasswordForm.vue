<script setup lang="ts">
import { useForm } from '@inertiajs/vue3'

const form = useForm({
  current_password: '',
  password: '',
  password_confirmation: '',
})

function submit() {
  form.put('/profile/password', {
    onSuccess: () => form.reset(),
  })
}
</script>

<template>
  <v-card class="profile-panel">
    <v-card-item>
      <v-card-title>Update password</v-card-title>
      <v-card-subtitle class="text-wrap">
        Use a long password and rotate it when your credentials change.
      </v-card-subtitle>
    </v-card-item>

    <v-card-text>
      <v-form @submit.prevent="submit">
        <v-text-field
          v-model="form.current_password"
          label="Current password"
          type="password"
          autocomplete="current-password"
          :error-messages="form.errors.current_password"
          required
        />
        <v-text-field
          v-model="form.password"
          label="New password"
          type="password"
          autocomplete="new-password"
          hint="At least 8 characters."
          :error-messages="form.errors.password"
          required
        />
        <v-text-field
          v-model="form.password_confirmation"
          label="Confirm new password"
          type="password"
          autocomplete="new-password"
          :error-messages="form.errors.password_confirmation"
          required
        />
        <div class="profile-form-actions">
          <v-btn type="submit" color="primary" :loading="form.processing">
            Update password
          </v-btn>
          <span v-if="form.recentlySuccessful" class="text-body-2 text-medium-emphasis">
            Password updated.
          </span>
        </div>
      </v-form>
    </v-card-text>
  </v-card>
</template>
