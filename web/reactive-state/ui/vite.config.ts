import { sveltekit } from '@sveltejs/kit/vite';
import { defineConfig } from 'vite';
import wasm from "vite-plugin-wasm";

export default defineConfig({
	plugins: [sveltekit(), wasm()],

	server: {
		fs: {
			// Allow serving files from one level up to the project root
			allow: ['.'],
		},
	},
});
