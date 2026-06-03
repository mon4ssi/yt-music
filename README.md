# yt-music

Desktop client scaffold for YouTube Music using Tauri v2 and React 19.

## Stack

- Tauri v2 (Rust)
- React 19 + TypeScript
- Vite
- pnpm

## Prerequisites

- Node.js 24+
- pnpm 11+
- Rust stable toolchain
- Xcode Command Line Tools

## Development

Install dependencies:

```bash
pnpm install
```

Run web app only:

```bash
pnpm dev
```

Run desktop app (Tauri + web dev server):

```bash
pnpm tauri:dev
```

## Build

Build web assets:

```bash
pnpm build
```

Build desktop bundles:

```bash
pnpm tauri:build
```

## Quality Checks

```bash
pnpm lint
pnpm typecheck
pnpm test
pnpm test:e2e
```

CI runs the same lint, typecheck, unit test, e2e smoke, and web build checks on pull requests.

## Current Status

- Issue #25 baseline scaffold complete
- Issue #26 tooling baseline complete
- React frontend and Tauri shell wired
- Local AI planning docs are intentionally ignored in git (`docs/`)
