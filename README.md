# WaveSpeed MCP for Zed

Zed extension wrapping the official [WaveSpeed MCP server](https://github.com/WaveSpeedAI/mcp-server)
(`@wavespeed/mcp`): run any model on the live [WaveSpeed](https://wavespeed.ai)
catalog — image, video, audio, 3D — from Zed's agent, with catalog search,
per-model schema introspection, local-file upload, and price quotes before
spending.

## Setup

**WaveSpeed MCP** is pending review in the Zed extension registry. Until it is
listed, install it as a dev extension: install Rust via
[rustup](https://www.rust-lang.org/tools/install), clone this repository, then
in Zed open the Extensions page, click **Install Dev Extension** (or run the
`zed: install dev extension` action) and select the cloned directory.

Then set your API key in the context server configuration (or sign in once with the
[wavespeed CLI](https://github.com/WaveSpeedAI/wavespeed-cli): `wavespeed login`
— the server reuses that login). Keys: [wavespeed.ai/accesskey](https://wavespeed.ai/accesskey).

Zed installs and updates the underlying npm package automatically.

## License

[MIT](LICENSE)

---

**[WaveSpeed AI](https://wavespeed.ai/)** — AI image & video generation platform.
Try it in the browser: **[Image generator](https://wavespeed.ai/image-generator)** · **[Video generator](https://wavespeed.ai/video-generator)**
