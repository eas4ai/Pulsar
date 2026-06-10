<script setup lang="ts">
import DeleteAccountDialog from '../components/profile/DeleteAccountDialog.vue'
import PasswordForm from '../components/profile/PasswordForm.vue'
import ProfileInformationForm from '../components/profile/ProfileInformationForm.vue'
import type { ProfileProps } from '../types/inertia-props'

defineProps<ProfileProps>()
</script>

<template>
  <v-container class="console-page profile-page">
    <header class="console-hero console-hero--compact">
      <div>
        <div class="console-eyebrow">Account</div>
        <h1>Profile settings</h1>
        <p>
          Keep identity, password, and account lifecycle controls in one place.
        </p>
      </div>
    </header>

    <div class="profile-grid">
      <ProfileInformationForm
        :name="name"
        :email="email"
        :email-verified="email_verified"
        :profile="profile"
      />
      <PasswordForm />
      <DeleteAccountDialog />
    </div>
  </v-container>
</template>
