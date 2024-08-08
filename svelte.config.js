import adapter from '@sveltejs/adapter-static' // This was changed from adapter-auto
import { vitePreprocess } from '@sveltejs/vite-plugin-svelte'


/** @type {import('@sveltejs/kit').Config} */
const config = {

  kit: {
    adapter: adapter(),
  }
};

export default config;
