import { spawn } from 'node:child_process'
import path from 'node:path'

// Paths holding the Rust sources that end up in the wasm package, relative
// to www/. Vite doesn't watch the ones outside www/ by default.
const rustPaths = ['../Cargo.toml', '../Cargo.lock', '../crates', 'Cargo.toml', 'src']

// Whether a changed file can affect the wasm package
const isRustSource = (file) => /\.rs$|Cargo\.(toml|lock)$/.test(file)

// Builds the wasm package into pkg/, rejecting if wasm-pack can't be run or fails
function wasmPack({ cwd, dev, target, outDir, outName }) {
    const devArgs = dev ? ['--dev'] : []
    const args = ['build', ...devArgs, '--target', target, '--out-dir', outDir, '--out-name', outName]
    return new Promise((resolve, reject) => {
        const child = spawn('wasm-pack', args, { cwd, stdio: 'inherit' })
        child.on('error', (err) => reject(new Error(`couldn't run wasm-pack: ${err.message}`)))
        child.on('exit', (code) => (code === 0 ? resolve() : reject(new Error(`wasm-pack exited with code ${code}`))))
    })
}

// Keeps a dev server's wasm package up to date: rebuilds it and reloads the
// page whenever a Rust source changes. Builds run one at a time, and changes
// made mid-build trigger exactly one more build.
class WasmPackWatcher {
    // First changed file not yet built
    #pending = null
    // Whether a build is running
    #building = false
    // Name of logger to show before logs
    #loggerRootName = 'wasm-pack'

    // Constructs new wasm-pack watcher from the given server and the package name to build
    constructor(server, packageName) {
        this.server = server
        this.root = server.config.root
        this.logger = server.config.logger
        this.packageName = packageName
    }

    // Builds the package, then watches the Rust sources for changes
    async start() {
        this.#logInfo('Starting up...')
        this.#build()
        let absolutePaths = rustPaths.map((p) => path.resolve(this.root, p))
        this.server.watcher.add(absolutePaths)
        this.server.watcher.on('all', (event, file) => {
            if (event === 'addDir') return
            if (event === 'unlinkDir') return
            if (isRustSource(file)) this.#onChange(file)
        })
    }

    // Queues a rebuild for a changed file, starting one unless a build is running
    async #onChange(file) {
        this.#logInfo(`${file} changed`)
        this.#pending ??= file
        if (this.#building) return
        this.#building = true
        while (this.#pending) {
            this.#pending = null
            let buildSuccessful = await this.#build()
            if (buildSuccessful) this.#reload()
        }
        this.#building = false
    }

    // Runs a dev build, logging the outcome, and reports whether it succeeded
    async #build() {
        this.#logInfo('Starting build...')
        try {
            await wasmPack({ cwd: this.root, dev: true, target: 'web', outDir: 'pkg', outName: this.packageName })
            this.#logInfo('Build successful!')
            return true
        } catch (err) {
            // Keep serving, the next change triggers another build
            this.#logError(`${err.message}`)
            return false
        }
    }

    // Drops cached modules and tells the page to reload with the new package
    #reload() {
        this.#logInfo('Reload page')
        this.server.moduleGraph.invalidateAll()
        this.server.ws.send({ type: 'full-reload' })
    }

    // Logs a message to the console at level info
    #logInfo(message) {
        this.logger.info(`[${this.#loggerRootName}] ${message}`, { timestamp: true })
    }

    // Logs a message to the console at level error
    #logError(message) {
        this.logger.error(`[${this.#loggerRootName}] ${message}`, { timestamp: true })
    }
}

const wasmPackageName = 'qwarks_www'

// Builds the wasm package with wasm-pack before `vite build` bundles it
const buildPlugin = {
    name: 'wasm-pack-build',
    apply: 'build',
    async buildStart() {
        await wasmPack({ cwd: this.environment.config.root, dev: false, target: 'web', outDir: 'pkg', outName: wasmPackageName })
    }
}

// Builds the wasm package when the dev server starts and rebuilds it on changes
const watchPlugin = {
    name: 'wasm-pack-watch',
    apply: 'serve',
    async configureServer(server) {
        await new WasmPackWatcher(server, 'qwarks_www').start()
    }
}

export default function wasmPackPlugin() {
    return [buildPlugin, watchPlugin]
}
