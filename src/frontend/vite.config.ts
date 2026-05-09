import tailwindcss from '@tailwindcss/vite';
import stwui from 'stwui/plugin';
import { sveltekit } from '@sveltejs/kit/vite';
import { defineConfig } from 'vite';

export default defineConfig({ plugins: [tailwindcss(), sveltekit()] });
