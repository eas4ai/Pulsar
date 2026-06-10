import './app.css'
import { createInertiaApp } from '@inertiajs/vue3'
import { createApp, createSSRApp, h, type DefineComponent } from 'vue'
import VuetifyInertiaLink from 'vuetify-inertia-link'
import vuetify from './plugins/vuetify'
import AppLayout from './layouts/AppLayout.vue'

createInertiaApp({
  resolve: (name) => {
    const pages = import.meta.glob<DefineComponent>('./pages/**/*.vue', {
      eager: true,
    })
    const page = pages[`./pages/${name}.vue`]
    if (!page) {
      throw new Error(`Inertia page not found: ${name}`)
    }
    const component = page.default as DefineComponent & { layout?: unknown }
    component.layout = component.layout ?? AppLayout
    return page
  },
  setup({ el, App, props, plugin }) {
    const app = el.hasAttribute('data-server-rendered')
      ? createSSRApp({ render: () => h(App, props) })
      : createApp({ render: () => h(App, props) })

    app.use(plugin).use(vuetify).use(VuetifyInertiaLink).mount(el)
  },
})
