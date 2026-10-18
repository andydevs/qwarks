import { spawn } from 'node:child_process'
import path from 'node:path'
import { defineConfig } from 'vite'
import run from 'vite-plugin-run'

// Rust sources that end up in the wasm package, relative to www/
const rustSources = [
    '../Cargo.toml',
    '../Cargo.lock',
    '../crates/*/Cargo.toml',
    '../crates/*/src/**/*.rs',
    'Cargo.toml',
    'src/**/*.rs',
]

// Directories holding rustSources, which Vite doesn't watch by default
const rustDirs = ['../Cargo.toml', '../Cargo.lock', '../crates', 'Cargo.toml', 'src']

// Builds the wasm package into pkg/, resolving with wasm-pack's exit code
function wasmPack(cwd, { dev }) {
    const args = ['build', '--target', 'web', '--out-dir', 'pkg', '--out-name', 'qwarks_www']
    if (dev) args.splice(1, 0, '--dev')
    return new Promise((resolve) => {
        const child = spawn('wasm-pack', args, { cwd, stdio: 'inherit' })
        child.on('error', () => resolve(null))
        child.on('exit', resolve)
    })
}

// An async iterable of file changes. Changes that arrive while the consumer
// is busy collapse into a single item holding the first of them, so each
// iteration of a `for await` loop sees everything changed since the last.
function changeStream() {
    let notify
    let next = new Promise((resolve) => (notify = resolve))
    return {
        notify: (file) => notify(file),
        async *[Symbol.asyncIterator]() {
            while (true) {
                const file = await next
                next = new Promise((resolve) => (notify = resolve))
                yield file
            }
        },
    }
}

// Rebuilds the wasm package with wasm-pack and reloads the page once the
// new package is ready. Builds run one at a time, and changes made mid-build
// trigger exactly one more build.
function wasmPackBuilder() {
    const changes = changeStream()

    const buildLoop = async (server) => {
        const log = server.config.logger
        for await (const file of changes) {
            log.info(`[wasm-pack] ${path.relative(server.config.root, file)} changed, rebuilding...`, { timestamp: true })
            const code = await wasmPack(server.config.root, { dev: true })
            if (code === 0) {
                log.info('[wasm-pack] rebuilt, reloading page', { timestamp: true })
                server.moduleGraph.invalidateAll()
                server.ws.send({ type: 'full-reload' })
            } else {
                log.error(`[wasm-pack] build failed (exit code ${code})`, { timestamp: true })
            }
        }
    }

    return [
        {
            name: 'wasm-pack-build',
            apply: 'build',
            async buildStart() {
                const code = await wasmPack(this.environment.config.root, { dev: false })
                if (code !== 0) this.error(`wasm-pack build failed (exit code ${code})`)
            },
        },
        {
            name: 'wasm-pack-watch-rust',
            apply: 'serve',
            async configureServer(server) {
                const log = server.config.logger
                log.info('[wasm-pack] initial build...', { timestamp: true })
                const code = await wasmPack(server.config.root, { dev: true })
                if (code !== 0) {
                    // Keep serving, the watcher rebuilds once the error is fixed
                    log.error(`[wasm-pack] initial build failed (exit code ${code})`, { timestamp: true })
                }
                server.watcher.add(rustDirs.map((dir) => path.resolve(server.config.root, dir)))
                buildLoop(server).catch((err) => log.error(`[wasm-pack] watcher stopped: ${err}`))
            },
        },
        run({
            silent: false,
            input: {
                name: 'wasm-pack',
                pattern: rustSources,
                startup: false,
                build: false,
                debounce: 100,
                onFileChanged: ({ file }) => changes.notify(file),
            },
        }),
    ]
}

export default defineConfig({
    plugins: [wasmPackBuilder()],
    build: {
        rolldownOptions: {
            // wasm-pack-build's buildStart is slow by design, don't warn about it
            checks: { pluginTimings: false },
        },
    },
    server: {
        watch: {
            // pkg is rewritten piecemeal by wasm-pack, the plugin reloads once it's done
            ignored: ['**/pkg/**', '**/target/**'],
        },
    },
})
