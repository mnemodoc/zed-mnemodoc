# zed-mnemodoc

[![CI](https://github.com/mnemodoc/zed-mnemodoc/actions/workflows/ci.yml/badge.svg)](https://github.com/mnemodoc/zed-mnemodoc/actions/workflows/ci.yml)
[![Docs](https://github.com/mnemodoc/zed-mnemodoc/actions/workflows/docs.yml/badge.svg)](https://github.com/mnemodoc/zed-mnemodoc/actions/workflows/docs.yml)
[![License: MIT](https://img.shields.io/badge/License-MIT-blue.svg)](LICENSE)

A [Zed](https://zed.dev) extension that connects `mcp-server` as a context server,
giving Zed's AI assistant access to your project's indexed documentation.

## Installation

Open Zed → `zed: install extension` → search for **Mnemodoc**.

## Prerequisites

- [Ollama](https://ollama.com) running locally with `nomic-embed-text` pulled:

  ```sh
  ollama pull nomic-embed-text
  ollama serve
  ```

- `mnemodoc-server` — installed automatically from GitHub Releases, or manually:

  ```sh
  # macOS
  brew install mnemodoc/tap/mnemodoc-server

  # Linux — download from the releases page:
  # https://github.com/mnemodoc/mcp-server/releases
  ```

## How it works

1. The extension resolves the `mnemodoc-server` binary (system install, a
   previously downloaded copy, or a fresh GitHub Release download).
2. It starts `mnemodoc-server serve --stdio`.
3. Zed spawns that process with its working directory set to the project root,
   so the server reads `.mnemodoc.yml` from there — or falls back to its
   built-in defaults when the file is absent.

The extension never reads project files itself: the WASM sandbox cannot reach
the project root, so all configuration lives in `.mnemodoc.yml` and is read by
the server.

## Configuration

Place a `.mnemodoc.yml` at your project root. This is the single source of
configuration — there are no Zed settings for this extension. Without the file,
the server starts on its defaults and will not index your project's docs.

```yaml
paths:
  - doc/

ollama:
  host: http://localhost:11434
  model: nomic-embed-text

search:
  top_k: 5
```

Index your docs once:

```sh
mnemodoc-server index doc/ --config .mnemodoc.yml
```

## Development

Requires: [mise](https://mise.jdx.dev) (installs the Rust toolchain) and the
`wasm32-wasip2` target, which `rustup` must add separately:

```sh
rustup target add wasm32-wasip2
mise dev:check    # clippy + tests (run after every change)
mise dev:build    # WASM build
mise dev:format   # cargo fmt
mise dev:doc      # generate and open API docs
```

## License

MIT
