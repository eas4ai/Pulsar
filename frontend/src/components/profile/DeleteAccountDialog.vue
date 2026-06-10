<script setup lang="ts">
import { ref } from 'vue'
import { useForm } from '@inertiajs/vue3'

const confirming = ref(false)

const form = useForm({
  password: '',
})

function close() {
  confirming.value = false
  form.reset()
  form.clearErrors()
}

function submit() {
  form.delete('/profile', {
    onSuccess: close,
  })
}
</script>

<template>
  <v-card class="profile-panel profile-panel--danger">
    <v-card-item>
      <v-card-title>Delete account</v-card-title>
      <v-card-subtitle class="text-wrap">
        Permanently remove your account after confirming your password.
      </v-card-subtitle>
    </v-card-item>
    <v-card-text>
      <v-btn color="error" variant="tonal" @click.stop="confirming = true">
        Delete account
      </v-btn>
    </v-card-text>
  </v-card>

  <v-dialog v-model="confirming" max-width="480">
    <v-card>
      <v-card-item>
        <v-card-title>Delete account</v-card-title>
        <v-card-subtitle class="text-wrap">
          This action cannot be undone. Enter your password to confirm.
        </v-card-subtitle>
      </v-card-item>
      <v-card-text>
        <v-form id="delete-account-form" @submit.prevent="submit">
          <v-text-field
            v-model="form.password"
            label="Password"
            type="password"
            autocomplete="current-password"
            placeholder="Your current password"
            :error-messages="form.errors.password"
            required
          />
        </v-form>
      </v-card-text>
      <v-card-actions class="justify-end">
        <v-btn variant="text" @click="close">Cancel</v-btn>
        <v-btn
          type="submit"
          form="delete-account-form"
          color="error"
          :loading="form.processing"
        >
          Permanently delete
        </v-btn>
      </v-card-actions>
    </v-card>
  </v-dialog>
</template>
