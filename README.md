# Tauri SvelteKit Template

A pnpm-based desktop app template using:

- Tauri v2
- SvelteKit
- TailwindCSS
- TypeScript
- Vite

## Template Generation Commands

This template was generated with the official Svelte and Tauri CLIs, using `pnpm`.

Create the SvelteKit, TypeScript, and TailwindCSS project:

```sh
pnpm dlx sv create . --template minimal --types ts --add tailwindcss="plugins:none" --install pnpm --no-dir-check
```

Install the Tauri CLI, the SvelteKit static adapter used by Tauri, and Node.js type definitions:

```sh
pnpm add -D @tauri-apps/cli @sveltejs/adapter-static @types/node
```

Initialize Tauri in the existing SvelteKit project:

```sh
pnpm tauri init --ci --app-name tauri-sveletkit-template --window-title tauri-sveletkit-template --frontend-dist ../build --dev-url http://localhost:1420 --before-dev-command "pnpm dev" --before-build-command "pnpm build"
```

Install the frontend Tauri API package:

```sh
pnpm add @tauri-apps/api
```

After generation, `svelte.config.js` was switched from `adapter-auto` to `adapter-static`, and `vite.config.ts` was configured to use port `1420`, matching Tauri's `devUrl`.

## Scripts

```sh
pnpm dev
pnpm check
pnpm build
pnpm tauri:dev
pnpm tauri:build
```

SvelteKit builds static files to `build/`, and Tauri loads that directory through `frontendDist`.

## Linux WebKitGTK Runtime Environment

On some Fedora GNOME Wayland systems, WebKitGTK can hit rendering issues on the DMA-BUF renderer path, depending on the GPU driver, Mesa version, and compositor behavior. Symptoms can include a blank window, flickering, or startup crashes. This template disables that WebKitGTK renderer path by default:

```sh
WEBKIT_DISABLE_DMABUF_RENDERER=${WEBKIT_DISABLE_DMABUF_RENDERER:-1}
```

It is implemented in `src-tauri/src/main.rs` so packaged desktop launches also get the setting. If the variable is already set by the user or system, the app preserves that existing value.

## Verification Commands

```sh
pnpm check
pnpm build
cargo check --manifest-path src-tauri/Cargo.toml
pnpm tauri dev
```
