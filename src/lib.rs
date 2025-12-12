use schemars::JsonSchema;
use serde::Deserialize;
use zed::settings::ContextServerSettings;
use zed_extension_api::{
    self as zed, serde_json, Command, ContextServerConfiguration, ContextServerId, Project, Result,
};

const MCP_PACKAGE_NAME: &str = "chrome-devtools-mcp";
const MCP_SERVER_ENTRYPOINT: &str = "node_modules/chrome-devtools-mcp/build/src/index.js";

#[derive(Debug, Clone, Deserialize, JsonSchema, Default, PartialEq)]
#[serde(rename_all = "lowercase")]
enum ChromeChannel {
    #[default]
    Stable,
    Canary,
    Beta,
    Dev,
}

impl ChromeChannel {
    fn as_str(&self) -> &'static str {
        match self {
            ChromeChannel::Stable => "stable",
            ChromeChannel::Canary => "canary",
            ChromeChannel::Beta => "beta",
            ChromeChannel::Dev => "dev",
        }
    }
}

/// Settings mapped to upstream chrome-devtools-mcp CLI options.
/// All settings are optional - Chrome launches automatically with sensible defaults.
#[derive(Debug, Deserialize, JsonSchema, Default)]
struct ChromeDevToolsMcpSettings {
    // Connection options
    #[serde(default)]
    auto_connect: Option<bool>,
    #[serde(default)]
    browser_url: Option<String>,
    #[serde(default)]
    ws_endpoint: Option<String>,
    #[serde(default)]
    ws_headers: Option<serde_json::Value>,

    // Chrome launch options
    #[serde(default)]
    headless: Option<bool>,
    #[serde(default)]
    executable_path: Option<String>,
    #[serde(default)]
    isolated: Option<bool>,
    #[serde(default)]
    user_data_dir: Option<String>,
    #[serde(default)]
    channel: Option<ChromeChannel>,
    #[serde(default)]
    viewport: Option<String>,
    #[serde(default)]
    chrome_arg: Vec<String>,

    // Network options
    #[serde(default)]
    proxy_server: Option<String>,
    #[serde(default)]
    accept_insecure_certs: Option<bool>,

    // Tool categories (set to false to disable)
    #[serde(default)]
    category_emulation: Option<bool>,
    #[serde(default)]
    category_performance: Option<bool>,
    #[serde(default)]
    category_network: Option<bool>,

    // Debugging
    #[serde(default)]
    log_file: Option<String>,
    #[serde(default)]
    extra_args: Vec<String>,
}

struct ChromeDevToolsMcpExtension;

impl ChromeDevToolsMcpExtension {
    fn load_settings(project: &Project) -> ChromeDevToolsMcpSettings {
        // Per-project settings are optional. If parsing fails, fall back to defaults rather than
        // preventing startup (keeps the wrapper resilient to schema changes).
        let Ok(raw) = ContextServerSettings::for_project("chrome-devtools-mcp-zed", project) else {
            return ChromeDevToolsMcpSettings::default();
        };
        let raw = raw.settings.unwrap_or_else(|| serde_json::json!({}));

        serde_json::from_value(raw).unwrap_or_default()
    }

    fn build_upstream_args(settings: &ChromeDevToolsMcpSettings) -> Vec<String> {
        let mut args: Vec<String> = Vec::new();

        if let Some(true) = settings.auto_connect {
            args.push("--autoConnect".to_string());
        }

        if let Some(url) = settings
            .browser_url
            .as_ref()
            .map(|s| s.trim())
            .filter(|s| !s.is_empty())
        {
            args.push("--browserUrl".to_string());
            args.push(url.to_string());
        }

        if let Some(url) = settings
            .ws_endpoint
            .as_ref()
            .map(|s| s.trim())
            .filter(|s| !s.is_empty())
        {
            args.push("--wsEndpoint".to_string());
            args.push(url.to_string());
        }

        if let Some(headers) = settings.ws_headers.as_ref() {
            // Upstream expects a JSON string for --wsHeaders.
            // We only pass it if wsEndpoint is configured to match upstream "implies".
            if settings
                .ws_endpoint
                .as_ref()
                .map(|s| !s.trim().is_empty())
                .unwrap_or(false)
            {
                if let Ok(json) = serde_json::to_string(headers) {
                    args.push("--wsHeaders".to_string());
                    args.push(json);
                }
            }
        }

        if let Some(true) = settings.headless {
            args.push("--headless".to_string());
        }

        if let Some(path) = settings
            .executable_path
            .as_ref()
            .map(|s| s.trim())
            .filter(|s| !s.is_empty())
        {
            args.push("--executablePath".to_string());
            args.push(path.to_string());
        }

        if let Some(true) = settings.isolated {
            args.push("--isolated".to_string());
        }

        if let Some(dir) = settings
            .user_data_dir
            .as_ref()
            .map(|s| s.trim())
            .filter(|s| !s.is_empty())
        {
            args.push("--userDataDir".to_string());
            args.push(dir.to_string());
        }

        if let Some(channel) = settings.channel.as_ref() {
            args.push("--channel".to_string());
            args.push(channel.as_str().to_string());
        }

        if let Some(viewport) = settings
            .viewport
            .as_ref()
            .map(|s| s.trim())
            .filter(|s| !s.is_empty())
        {
            args.push("--viewport".to_string());
            args.push(viewport.to_string());
        }

        for a in &settings.chrome_arg {
            let a = a.trim();
            if !a.is_empty() {
                args.push("--chromeArg".to_string());
                args.push(a.to_string());
            }
        }

        if let Some(proxy) = settings
            .proxy_server
            .as_ref()
            .map(|s| s.trim())
            .filter(|s| !s.is_empty())
        {
            args.push("--proxyServer".to_string());
            args.push(proxy.to_string());
        }

        if let Some(true) = settings.accept_insecure_certs {
            args.push("--acceptInsecureCerts".to_string());
        }

        if let Some(log_file) = settings
            .log_file
            .as_ref()
            .map(|s| s.trim())
            .filter(|s| !s.is_empty())
        {
            args.push("--logFile".to_string());
            args.push(log_file.to_string());
        }

        if let Some(false) = settings.category_emulation {
            args.push("--no-category-emulation".to_string());
        }

        if let Some(false) = settings.category_performance {
            args.push("--no-category-performance".to_string());
        }

        if let Some(false) = settings.category_network {
            args.push("--no-category-network".to_string());
        }

        args.extend(settings.extra_args.iter().cloned());

        args
    }
}

impl zed::Extension for ChromeDevToolsMcpExtension {
    fn new() -> Self {
        Self
    }

    fn context_server_command(
        &mut self,
        _context_server_id: &ContextServerId,
        project: &Project,
    ) -> Result<Command> {
        let settings = Self::load_settings(project);

        // Ensure the upstream npm package is installed/updated (Zed manages installation).
        let latest_version = zed::npm_package_latest_version(MCP_PACKAGE_NAME)?;
        let installed_version = zed::npm_package_installed_version(MCP_PACKAGE_NAME)?;

        eprintln!(
            "[chrome-devtools-mcp-zed] npm package: {} installed={:?} latest={}",
            MCP_PACKAGE_NAME, installed_version, latest_version
        );

        if installed_version.as_deref() != Some(latest_version.as_ref()) {
            eprintln!(
                "[chrome-devtools-mcp-zed] installing/updating npm package {}@{}",
                MCP_PACKAGE_NAME, latest_version
            );
            zed::npm_install_package(MCP_PACKAGE_NAME, &latest_version)?;
        }

        // Launch the MCP server over stdio.
        let node = zed::node_binary_path()?;

        let cwd = std::env::current_dir().map_err(|e| e.to_string())?;
        let entrypoint = cwd.join(MCP_SERVER_ENTRYPOINT);

        if !entrypoint.exists() {
            eprintln!(
                "[chrome-devtools-mcp-zed] expected MCP server entrypoint missing: {}",
                entrypoint.to_string_lossy()
            );
            eprintln!(
                "[chrome-devtools-mcp-zed] package layout/version may differ; expected {}",
                MCP_SERVER_ENTRYPOINT
            );
        } else {
            eprintln!(
                "[chrome-devtools-mcp-zed] launching MCP server entrypoint: {}",
                entrypoint.to_string_lossy()
            );
        }

        let mut args = vec![entrypoint.to_string_lossy().to_string()];
        args.extend(Self::build_upstream_args(&settings));

        Ok(Command {
            command: node,
            args,
            env: vec![],
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
        let settings_schema =
            serde_json::to_string(&schemars::schema_for!(ChromeDevToolsMcpSettings))
                .map_err(|e| e.to_string())?;

        Ok(Some(ContextServerConfiguration {
            installation_instructions,
            default_settings,
            settings_schema,
        }))
    }
}

zed::register_extension!(ChromeDevToolsMcpExtension);
