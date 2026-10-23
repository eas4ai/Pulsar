<script setup lang="ts">
import { Form, Head, usePage } from '@inertiajs/vue3'
import type { ConnectedAgentsProps } from '../../types/inertia-props'

defineProps<ConnectedAgentsProps>()
const page = usePage()
</script>

<template>
  <Head title="Connected agents" />
  <v-container class="py-8" style="max-width: 900px">
    <h1 class="text-h4 mb-6">Connected agents</h1>
    <v-card class="pa-6 mb-4">
      <h2 class="text-h5 mb-4">Apps</h2>
      <p v-if="clients.length === 0">No apps can act for you.</p>
      <div v-for="client in clients" :key="client.id" class="mb-6">
        <h3><bdi>{{ client.client_name }}</bdi></h3>
        <p>From <bdi>{{ client.client_host }}</bdi>. Allowed {{ client.created_at.slice(0, 10) }}, last used {{ client.last_used_at?.slice(0, 10) ?? 'never' }}.</p>
        <ul class="pl-6 mb-3"><li v-for="scope in client.scopes" :key="scope.name">{{ scope.description }} (<code>{{ scope.name }}</code>)</li></ul>
        <Form :action="`${page.props.root}/connected-agents/clients/${encodeURIComponent(client.id)}/revoke`" method="post" v-slot="{ processing }">
          <v-btn type="submit" color="error" variant="outlined" :disabled="processing">Revoke app</v-btn>
        </Form>
      </div>
    </v-card>
    <v-card class="pa-6">
      <h2 class="text-h5 mb-4">Agents</h2>
      <p v-if="agents.length === 0">No agents can act for you.</p>
      <div v-for="agent in agents" :key="agent.id" class="mb-6">
        <h3><bdi>{{ agent.provider ?? 'Agent' }}</bdi></h3>
        <p>Registered {{ agent.created_at.slice(0, 10) }}<template v-if="agent.claimed_at">, claimed {{ agent.claimed_at.slice(0, 10) }}</template>.</p>
        <ul class="pl-6 mb-3"><li v-for="scope in agent.scopes" :key="scope.name">{{ scope.description }} (<code>{{ scope.name }}</code>)</li></ul>
        <Form :action="`${page.props.root}/connected-agents/agents/${encodeURIComponent(agent.id)}/revoke`" method="post" v-slot="{ processing }">
          <v-btn type="submit" color="error" variant="outlined" :disabled="processing">Revoke agent</v-btn>
        </Form>
      </div>
    </v-card>
  </v-container>
</template>
