import { createInertiaApp, router } from '@inertiajs/vue3'
import { createApp, createSSRApp, h } from 'vue'
import VuetifyInertiaLink from 'vuetify-inertia-link'
import vuetify from './plugins/vuetify'
import { installPageTools } from './lib/agents'
import { staticHttpClient } from './lib/static'
import AppLayout from './layouts/AppLayout.vue'

const nonce = document.querySelector<HTMLMetaElement>('meta[property="csp-nonce"]')?.nonce

installPageTools(router)

createInertiaApp({
  nonce,
  http: staticHttpClient(),
  pages: './pages',
  layout: () => AppLayout,
  setup({ el, App, props, plugin }) {
    if (!el) return
    const app = el.hasAttribute('data-server-rendered')
      ? createSSRApp({ render: () => h(App, props) })
      : createApp({ render: () => h(App, props) })

    app.use(plugin).use(vuetify).use(VuetifyInertiaLink).mount(el)
  },
})
