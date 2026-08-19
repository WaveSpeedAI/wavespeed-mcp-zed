# WaveSpeed MCP

Run any model on the live [WaveSpeed](https://wavespeed.ai) catalog — image,
video, audio, 3D — from Zed's agent. Seven tools: model search, per-model
schema introspection, generation (with `@path` local-file upload), price
quotes before spending, balance, upload, and prediction recovery.

## Authentication

Set `wavespeed_api_key` below (keys: [wavespeed.ai/accesskey](https://wavespeed.ai/accesskey)),
or leave it empty if the [wavespeed CLI](https://github.com/WaveSpeedAI/wavespeed-cli)
is installed and signed in via `wavespeed login` — the server reuses that login.
