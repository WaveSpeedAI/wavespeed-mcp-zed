# WaveSpeed MCP

Run any model on the live [WaveSpeed](https://wavespeed.ai) catalog — image,
video, audio, 3D — from Zed's agent. 16 tools: model search, per-model
schema introspection, generation (with `@path` local-file upload), image and
video shortcuts (generate, edit, extend), price quotes before spending,
balance, run history, billing and usage, upload, and prediction recovery.

## Authentication

Set `wavespeed_api_key` below (keys: [wavespeed.ai/accesskey](https://wavespeed.ai/accesskey)),
or leave it empty if the [wavespeed CLI](https://github.com/WaveSpeedAI/wavespeed-cli)
is installed and signed in via `wavespeed login` — the server reuses that login.
