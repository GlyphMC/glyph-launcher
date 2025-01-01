# Glyph Launcher

The Glyph launcher is a cutting-edge open source Minecraft launcher with instance management, mod downloading, and more coming soon. The launcher is built to be extremely lightweight, utilizing [SvelteKit](https://svelte.dev/docs/kit/introduction) and [Tauri](https://tauri.app/).

Please note that Glyph launcher is still in development and is not yet ready for public use.

## Building

In order to build the launcher from source, follow these steps:

1. Install Node.js
2. Install pnpm
3. Install Rust
4. Clone the repository
5. Run `pnpm install`
6. Run `pnpm run tauri build`
7. The launcher will be built in the `src-tauri/target/release` directory
