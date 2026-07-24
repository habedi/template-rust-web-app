import { sveltekit } from '@sveltejs/kit/vite';
import tailwindcss from '@tailwindcss/vite';
// `vitest/config` re-exports Vite's `defineConfig` with the `test` field typed.
import { defineConfig } from 'vitest/config';

export default defineConfig({
	plugins: [tailwindcss(), sveltekit()],
	test: {
		include: ['{src,test}/**/*.{test,spec}.{js,ts}'],
	},
});
