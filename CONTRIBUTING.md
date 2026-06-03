# Contributing

## Workflow

1. Tasks are tracked via GitHub issues with `type:task` and `status:ready` labels.
2. Create a branch from `main`: `feat/<issue-id>-<short-name>`.
3. Implement with small, reviewable commits.
4. Before opening a PR, run the full verification suite.
5. Open a PR linking the issue (`Closes #<id>`).
6. After merge, update `docs/ROADMAP.local.md` to mark the task complete.

## Issue Templates

- **Task** (`type:task`): Use for implementation work. Body doubles as an agent prompt.
- **Bug** (`type:bug`): Use for bugs and regressions.

## Quality Gates

Every PR must pass:

```bash
pnpm lint
pnpm typecheck
pnpm test
pnpm test:e2e
pnpm build
```

## Stack

- Tauri v2 (Rust)
- React 19 + TypeScript
- Vite
- pnpm

## Local Docs

Planning documents in `docs/` are local-only and intentionally gitignored.
