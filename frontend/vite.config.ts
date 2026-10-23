import { defineConfig } from 'vite'
import inertia from '@inertiajs/vite'
import vue from '@vitejs/plugin-vue'
import vuetify from 'vite-plugin-vuetify'

// Keep bare `bun run dev` off Vite's commonly-squatted 5173.
const vitePort = Number(process.env.VITE_PORT) || 5765

export default defineConfig({
  base: './',
  plugins: [inertia(), vue(), vuetify({ autoImport: true, styles: { configFile: 'src/settings.scss' } })],
  server: {
    port: vitePort,
    origin: `http://127.0.0.1:${vitePort}`,
    strictPort: true,
    cors: true,
  },
  build: {
    outDir: '../public/assets',
    emptyOutDir: true,
    manifest: true,
    rolldownOptions: {
      input: ['src/main.ts', 'src/app.css', 'src/vuetify.scss'],
      output: {
        codeSplitting: {
          includeDependenciesRecursively: false,
          groups: [
            { name: 'vue', test: /\/(?:@vue|vue)\//, priority: 30 },
            { name: 'toolkit', test: /\/es-toolkit\//, priority: 20 },
            { name: 'inertia', test: /\/@inertiajs\//, priority: 10 },
          ],
        },
      },
    },
  },
})
