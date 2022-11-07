import { defineConfig } from "vite"
import pugPlugin from "vite-plugin-pug"
import viteCompression from "vite-plugin-compression"
import importMetaEnv from '@import-meta-env/unplugin'

export default defineConfig(async () => {
	/** @type {import('vite').UserConfig} */
	return {
		plugins: [importMetaEnv.vite({ example: '.env.example' }), pugPlugin(), viteCompression()],
		minify: "esbuild",
	}
})
