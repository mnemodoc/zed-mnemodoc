# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Instructions

- Comments above the code (never inline).
- Code, comments, and test descriptions in English.
- Named arguments on complex calls.
- After every change, always run `mise dev:check`.

## Session start

1. Call the MCP tool `mnemodoc_query_documents` to load repo context (query: "zed-mnemodoc extension architecture workflow").
2. Read the files in `.claude/plans/` if needed to complete the context of work in progress.

> **`get_project_context` (not available yet):** once the tool ships in the server, call it first with `query: "zed-mnemodoc extension Rust WASM MCP"` and adopt the returned role. Role files live in `.claude/roles/`.

## What this is

A Rust/WASM Zed extension that launches `mnemodoc-server` as a context server.
It resolves the binary and runs `serve --stdio`; the server reads `.mnemodoc.yml`
from the project root on its own (Zed spawns it with that CWD).

## Development commands

```sh
mise dev:check     # clippy + tests (run this after every change)
mise dev:test      # cargo test (runs natively, not WASM)
mise dev:clippy    # cargo clippy -- -D warnings
mise dev:build     # cargo build --target wasm32-wasip2 --release
mise dev:format    # cargo fmt
mise dev:doc       # cargo doc --no-deps --open
```

Run a single test module: `cargo test <module_name>` (e.g. `cargo test platform`).

## Architecture

Pure logic lives in two modules with no dependency on `zed_extension_api`:

| Module | Responsibility |
|--------|---------------|
| `src/platform.rs` | `asset_name(os, arch)`, `select_asset_url` — GitHub Release asset name and URL selection |
| `src/binary.rs` | `system_path()`, `local_path()` — binary path resolution |
| `src/lib.rs` | `MnemodocExtension` — resolves the binary, runs `serve --stdio`, implements `zed::Extension` |

`lib.rs` is the only file that uses the Zed WASM API. It has no unit tests.

### Binary resolution order

1. System-installed binary (Homebrew paths, `/usr/bin`)
2. Previously downloaded binary in extension work dir
3. Auto-download from `mnemodoc/mcp-server` GitHub Releases

The version is not pinned: `lib.rs` calls `zed::latest_github_release` on
`SERVER_REPO` (`mnemodoc/mcp-server`, defined in `src/platform.rs`) with
`require_assets: true` and `pre_release: false`, then `platform::asset_name`
+ `platform::select_asset_url` pick the asset for the current OS/arch. The
download fails at runtime if the latest release has no asset named
`mnemodoc-server-<os>-<arch>`.

### Config resolution

The extension passes no `--config`. Zed spawns the server with its CWD set to
the project root, so the server resolves its default config path
(`.mnemodoc.yml`) against that CWD — or uses its built-in defaults when the file
is absent. The WASM sandbox cannot reach the project root, so the extension
itself never reads or generates config.

## Testing

`cargo test` compiles the crate as `rlib` (not `cdylib`) — tests run natively without a WASM runner. The two pure modules (`platform`, `binary`) are the only ones with unit tests.

## WASM target

```sh
rustup target add wasm32-wasip2
cargo build --target wasm32-wasip2 --release
# Output: target/wasm32-wasip2/release/zed_mnemodoc.wasm
```

## Documentation conventions

- `///` on every `pub` item.
- `# Errors` section on every function returning `Result`.
- `# Panics` section if a function can panic.
- No comments that restate the function name.

## Workflow rules

### Roles

Adopt the role matching the task domain. Detailed role files live in `.claude/roles/`.

| Domain | Role |
| --- | --- |
| Pure Rust modules, serde/schemars, `wasm32-wasip2`, `zed_extension_api`, clippy/tests | Rust/WASM developer |
| MCP protocol, context server, Zed lifecycle, WASM sandbox | MCP / editor-integration expert |
| `platform.rs`, asset selection, GitHub Releases, CI | Release-engineering / CI expert |
| AI, LLM, MCP, RAG, embeddings, Ollama | AI / LLM engineering expert |
| Docs, README, indexable doc, `///` conventions | Technical writer |

### Communication

- Git commit messages in English (imperative).
- Never persist secrets (passwords, API tokens, keys) — reference the env var name instead.

### Analysis

- **Memory is not a source:** never act or assert from training memory or session context. Anything not read from a file in the **current turn** cannot ground an action — read first, never assume. No source → say so.
- "Can we / is it possible" questions: analyze impact (affected files, side effects, risks) and wait for confirmation before implementing.
- Rhetorical questions ("it's X, right?"): answer directly.
- Uncovered case: stop and ask, never improvise.

### Editing

- Edit files directly, no pre-confirmation — the user reads diffs.
- Change only what's asked; never reformat or restyle silently.
- Related issue spotted mid-task: **flag only**, never fix without an explicit request.
- Every created file ends with a trailing newline.
- Before creating a file or touching several, state the scope and ask for confirmation before proceeding.

### Plan mode

- Plan before multi-file or uncertain changes; skip it for unambiguous targeted fixes.
- Execution plans go in `.claude/plans/` with descriptive kebab-case names.
- Never list "commit" or "push" as a plan step.
- Approval ≠ execution: wait for an explicit "fais le" / "exécute" before implementing.
- Task done with no further instruction: stop and wait — silence is not approval.
- Ambiguity: don't act, ask.

### Git

- Commit/push only when the user writes "commit" / "push" in the current turn. "ok", "parfait", "c'est bon" are not triggers.
- Commit message: English imperative title (≤70 chars, no period); body bullets, one per file or group; trailing `Co-Authored-By: Claude Opus 4.8 <noreply@anthropic.com>`.
