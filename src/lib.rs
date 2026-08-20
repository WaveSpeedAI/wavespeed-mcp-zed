use schemars::JsonSchema;
use serde::Deserialize;
use zed::settings::ContextServerSettings;
use zed_extension_api::{
    self as zed, serde_json, Command, ContextServerConfiguration, ContextServerId, Project, Result,
};

const MCP_PACKAGE_NAME: &str = "@wavespeed/mcp";
const MCP_SERVER_ENTRYPOINT: &str = "node_modules/@wavespeed/mcp/dist/index.js";

/// Settings for the WaveSpeed context server.
#[derive(Debug, Deserialize, JsonSchema, Default)]
struct WavespeedMcpSettings {
    /// WaveSpeed API key (https://wavespeed.ai/accesskey). Optional when the
    /// wavespeed CLI is installed and signed in via `wavespeed login` — the
    /// server reuses the CLI's stored login.
    #[serde(default)]
    wavespeed_api_key: Option<String>,
}

struct WavespeedMcpExtension;

impl WavespeedMcpExtension {
    fn load_settings(project: &Project) -> WavespeedMcpSettings {
        // Settings are optional; fall back to defaults rather than blocking
        // startup (the server can also authenticate via the CLI's login).
        let Ok(raw) = ContextServerSettings::for_project("wavespeed", project) else {
            return WavespeedMcpSettings::default();
        };
        let raw = raw.settings.unwrap_or_else(|| serde_json::json!({}));
        serde_json::from_value(raw).unwrap_or_default()
    }
}

impl zed::Extension for WavespeedMcpExtension {
    fn new() -> Self {
        Self
    }

    fn context_server_command(
        &mut self,
        _context_server_id: &ContextServerId,
        project: &Project,
    ) -> Result<Command> {
        let settings = Self::load_settings(project);

        // Zed manages the npm package install/update inside the extension dir.
        let latest_version = zed::npm_package_latest_version(MCP_PACKAGE_NAME)?;
        let installed_version = zed::npm_package_installed_version(MCP_PACKAGE_NAME)?;
        if installed_version.as_deref() != Some(latest_version.as_ref()) {
            zed::npm_install_package(MCP_PACKAGE_NAME, &latest_version)?;
        }

        let node = zed::node_binary_path()?;
        let cwd = std::env::current_dir().map_err(|e| e.to_string())?;
        let entrypoint = cwd.join(MCP_SERVER_ENTRYPOINT);

        let mut env: Vec<(String, String)> = Vec::new();
        // Channel attribution: identify this Zed extension as the client so
        // usage is distinguishable from plain `npx @wavespeed/mcp`.
        env.push((
            "WAVESPEED_CLIENT_NAME".to_string(),
            "wavespeed-mcp-zed".to_string(),
        ));
        if let Some(key) = settings
            .wavespeed_api_key
            .as_ref()
            .map(|k| k.trim())
            .filter(|k| !k.is_empty())
        {
            env.push(("WAVESPEED_API_KEY".to_string(), key.to_string()));
        }

        Ok(Command {
            command: node,
            args: vec![entrypoint.to_string_lossy().to_string()],
            env,
        })
    }

    fn context_server_configuration(
        &mut self,
        _context_server_id: &ContextServerId,
        _project: &Project,
    ) -> Result<Option<ContextServerConfiguration>> {
        let installation_instructions =
            include_str!("../configuration/installation_instructions.md").to_string();
        let default_settings = include_str!("../configuration/default_settings.jsonc").to_string();
        let settings_schema = serde_json::to_string(&schemars::schema_for!(WavespeedMcpSettings))
            .map_err(|e| e.to_string())?;

        Ok(Some(ContextServerConfiguration {
            installation_instructions,
            default_settings,
            settings_schema,
        }))
    }
}

zed::register_extension!(WavespeedMcpExtension);
