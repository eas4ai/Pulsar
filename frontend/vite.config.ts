import { defineConfig } from 'vite'
import vue from '@vitejs/plugin-vue'
import vuetify from 'vite-plugin-vuetify'

// Keep bare `bun run dev` off Vite's commonly-squatted 5173.
const vitePort = Number(process.env.VITE_PORT) || 5765

export default defineConfig({
  base: './',
  plugins: [vue(), vuetify({ autoImport: true })],
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
    rollupOptions: {
      input: 'src/main.ts',
    },
  },
})
