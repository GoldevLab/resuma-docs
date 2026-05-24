# Resuma documentation site

Official docs and landing page for [Resuma](https://github.com/GolfredoPerezFernandez/resuma).

**Live:** https://resuma-docs.fly.dev

## Local dev

```bash
# Resuma Client (TypeScript widgets) — required before cargo build:
npm install
npm run build:client

# Patch local Resuma (ClientComponent + client_asset API):
# cp .cargo/config.toml.example .cargo/config.toml

cargo run
```

Open http://127.0.0.1:3000

## Client components

TypeScript sources live in `client/components/`. Esbuild writes bundles to `static/client/`; Rust embeds them via `FlowApp::client_asset()`.

Example: `client/components/hero-particles.ts` powers the home hero (Three.js WebGPU).

## Deploy (Fly.io)

```bash
fly deploy
```

GitHub Actions: `CI` on push/PR, `Fly Deploy` after green CI on `main` (requires `FLY_API_TOKEN` secret).
