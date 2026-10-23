import { defineConfig } from 'vite'
import tailwindcss from '@tailwindcss/vite'
import wasmPackPlugin from './plugins/vite-plugin-wasm-pack.js'

export default defineConfig({
    plugins: [tailwindcss(), wasmPackPlugin()],
    build: {
        rolldownOptions: {
            // wasm-pack-build's buildStart is slow by design, don't warn about it
            checks: { pluginTimings: false }
        }
    },
    server: {
        watch: {
            // pkg is rewritten piecemeal by wasm-pack, the plugin reloads once it's done
            ignored: ['**/pkg/**', '**/target/**']
        }
    }
})
