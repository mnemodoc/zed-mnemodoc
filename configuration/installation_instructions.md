## Mnemodoc

Indexes your project's Markdown documentation via [Ollama](https://ollama.com) embeddings and exposes search to Zed's AI assistant.

### Prerequisites

1. **Ollama** running locally with the embedding model:

   ```sh
   # macOS native (recommended — uses Metal GPU)
   brew install ollama
   ollama pull nomic-embed-text
   ollama serve

   # or via Docker
   docker run -d --name ollama -p 11434:11434 ollama/ollama
   docker exec ollama ollama pull nomic-embed-text
   ```

2. **mnemodoc-server** binary — installed automatically from GitHub Releases, or manually:

   ```sh
   # macOS
   brew install mnemodoc/tap/mnemodoc-server

   # Linux — download from the releases page:
   # https://github.com/mnemodoc/mcp-server/releases
   ```

### Configuration — required

Create a `.mnemodoc.yml` at your **project root**. The server reads it on its
own (Zed spawns it with the project root as its working directory), so no Zed
settings are involved. Without this file the server starts on its built-in
defaults and will not index your project's docs.

```yaml
paths:
  - doc/

ollama:
  host: http://localhost:11434
  model: nomic-embed-text

search:
  top_k: 5
```

### Index your docs

```sh
mnemodoc-server index doc/ --config .mnemodoc.yml
```
