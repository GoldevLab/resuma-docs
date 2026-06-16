# Resuma documentation site (`resuma-docs`)

Official docs and landing page for [Resuma](https://github.com/GoldevLab/resuma).

**Path:** sibling to the framework repo — `apps/resuma-docs` (i.e. `../resuma-docs` from `apps/resuma`).  
**GitHub:** [resuma-docs](https://github.com/GoldevLab/resuma-docs)  
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

### GitHub Actions (recomendado)

El repo ya incluye `.github/workflows/ci.yml` y `.github/workflows/fly.yml`.  
**CI** corre en cada push/PR; **Fly Deploy** solo corre cuando CI en `main` termina en verde.

1. Desde la raíz de este repo (app `resuma-docs` en `fly.toml`):

```bash
fly tokens create deploy -x 999999h
```

Copia **todo** el token de la salida, incluyendo el prefijo `FlyV1` y el espacio.

2. En GitHub → [resuma-docs](https://github.com/GoldevLab/resuma-docs) → **Settings** → **Secrets and variables** → **Actions** → **New repository secret**
   - Name: `FLY_API_TOKEN`
   - Value: el token del paso 1

3. Haz push a `main`. En **Actions** verás primero **CI** y, si pasa, **Fly Deploy**.

Token alternativo (más permisos, varias apps): `fly auth token`

### Deploy manual

From the repo root (`apps/resuma-docs`):

```bash
docker build -t resuma-docs .   # optional local test
fly deploy
```

Root files: `Dockerfile`, `fly.toml`, `.dockerignore`.
