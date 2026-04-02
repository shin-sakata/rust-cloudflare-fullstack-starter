# rust-cloudflare-fullstack-starter

Rust fullstack SPA on Cloudflare Workers.
Frontend (Leptos) and Backend (Axum) both compile to WebAssembly and run on the edge.

## Tech Stack

| Layer | Technology |
|---|---|
| Frontend | Leptos 0.8 (CSR) + leptos_router |
| Backend | Axum on Cloudflare Workers |
| Styling | Tailwind CSS v4 |
| Data Fetching | leptos-fetch |
| Shared Types | serde (shared crate) |
| Build Target | `wasm32-unknown-unknown` |
| Dev Tools | Nix + process-compose |

## Quick Start

```bash
# 1. Enter the dev shell (provides all tools)
nix develop

# 2. Start everything
process-compose up
```

Open http://localhost:8787

## Project Structure

```
├── frontend/              # Leptos CSR frontend
│   ├── src/
│   │   ├── main.rs        # App entry + router
│   │   ├── pages/         # Home, About, NotFound
│   │   ├── components/    # NavBar + ui/
│   │   │   └── ui/        # Button, Badge, Spinner, ...
│   │   └── api/           # Data fetching (leptos-fetch)
│   ├── index.html
│   ├── input.css          # Tailwind + design tokens
│   └── Trunk.toml         # Tailwind CSS v4 config
├── worker/                # Axum backend on CF Workers
│   └── src/lib.rs         # /api/* routes
├── shared/                # Shared types between frontend & backend
│   └── src/health.rs      # HealthResponse
├── flake.nix              # Nix dev shell
├── process-compose.yaml   # Dev process orchestration
└── wrangler.jsonc         # Cloudflare Workers config
```

## Architecture

```
Browser
  │
  ├── /api/*  ──→  Cloudflare Worker (Axum)  ──→  JSON response
  │
  └── /*      ──→  Static assets (frontend/dist/)  ──→  Leptos SPA
```

- `wrangler.jsonc` routes `/api/*` to the Worker, everything else serves the SPA
- `not_found_handling: "single-page-application"` enables client-side routing

## Commands

```bash
# Dev (all-in-one)
process-compose up

# Dev (individual)
cd frontend && trunk watch --dist dist    # Frontend watch
wrangler dev                              # Worker + asset serving

# Build
trunk build --release                     # Frontend → frontend/dist/
worker-build --release worker             # Worker → worker/build/

# Check
cargo check --workspace --target wasm32-unknown-unknown
cargo clippy --workspace --target wasm32-unknown-unknown
```

## How It Works

**Frontend** — Leptos components are compiled to WebAssembly via `trunk`. The output (`frontend/dist/`) contains HTML, JS glue code, and `.wasm` binary. Tailwind CSS v4 is integrated through Trunk's built-in support (no npm required).

**Backend** — Axum router handles API requests inside a Cloudflare Worker. `worker-build` compiles Rust to WebAssembly and generates JS bindings.

**Shared** — The `shared` crate defines types (e.g., `HealthResponse`) used by both frontend and backend, ensuring type-safe API communication.

**Dev environment** — `flake.nix` provides all tools declaratively: Rust + wasm target (via fenix), trunk, worker-build, wrangler, and process-compose. `process-compose` orchestrates both processes, with the worker waiting for the frontend's initial build to complete.

## Requirements

- [Nix](https://nixos.org/) with flakes enabled

That's it. Everything else is provided by `nix develop`.

## License

MIT
