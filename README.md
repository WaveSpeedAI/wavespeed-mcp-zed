# WaveSpeed MCP for Zed

Zed extension wrapping the official [WaveSpeed MCP server](https://github.com/WaveSpeedAI/mcp-server)
(`@wavespeed/mcp`): run any model on the live [WaveSpeed](https://wavespeed.ai)
catalog — image, video, audio, 3D — from Zed's agent, with catalog search,
per-model schema introspection, local-file upload, and price quotes before
spending.

## Setup

Install **WaveSpeed MCP** from Zed's extension panel, then set your API key in
the context server configuration (or sign in once with the
[wavespeed CLI](https://github.com/WaveSpeedAI/wavespeed-cli): `wavespeed login`
— the server reuses that login). Keys: [wavespeed.ai/accesskey](https://wavespeed.ai/accesskey).

Zed installs and updates the underlying npm package automatically.

## License

[MIT](LICENSE)

---

**[WaveSpeed AI](https://wavespeed.ai/)** — hosted inference for image, video, audio and 3D models.
Try it in the browser: **[Image generator](https://wavespeed.ai/image-generator)** · **[Video generator](https://wavespeed.ai/video-generator)**
