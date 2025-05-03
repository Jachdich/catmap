import { defineConfig } from 'vite'
import { svelte } from '@sveltejs/vite-plugin-svelte'

// https://vite.dev/config/
export default defineConfig({
  plugins: [svelte()],
  base: "/catmap/",
  server: {
    allowedHosts: ["cospox.com", ".cospox.com"],
  }
})
