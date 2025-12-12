# Chrome DevTools MCP for Zed

This repository is a **Zed extension** that integrates the upstream **Chrome DevTools MCP** server by launching it as a **Zed Context Server**.

## Overview

This is an **MCP-native** integration:

- Zed runs the upstream MCP server over **stdio**.
- All tools and behavior come from the upstream `chrome-devtools-mcp` package.
- This extension is a thin wrapper that installs/updates and launches the upstream server.
- All upstream CLI options are exposed as Zed settings.

## Related Projects

- Upstream MCP server: [`chrome-devtools-mcp`](https://github.com/ChromeDevTools/chrome-devtools-mcp)
- This Zed wrapper: `chrome-devtools-mcp-zed`

## What This Extension Does

### Does

- Ensures the npm package `chrome-devtools-mcp` is installed (and kept updated).
- Launches the MCP server entrypoint from the installed package.
- Exposes upstream MCP tools inside Zed via the context server interface.
- Maps all upstream CLI options to Zed settings.

### Does Not

- Add custom Zed UI beyond the context server integration.
- Provide custom slash commands (tools are provided by upstream MCP server).
- Bundle Chrome or manage your system Chrome installation.

## Prerequisites

- [Zed](https://zed.dev/)
- Node.js available on your system (Zed uses Node to run the MCP server)
- (Optional) Google Chrome / Chromium, depending on how you use the MCP server
- If connecting to an existing Chrome instance, you'll need Chrome started with a remote debugging port

## Installation (Development)

1. Ensure you have this repository locally.
2. In Zed:
   - Open **Extensions**
   - Choose **Install Dev Extension**
   - Select the `chrome-devtools-mcp-zed` directory

After installation, enable the context server in Zed if it isn't enabled automatically.

## Configuration

This extension supports per-project settings via Zed's `context_servers` configuration. All setting names are aligned with upstream CLI options (see upstream `src/cli.ts`).

### Basic Example

```json
{
  "context_servers": {
    "chrome-devtools-mcp-zed": {
      "settings": {
        "browser_url": "http://127.0.0.1:9222"
      }
    }
  }
}
```

### Complete Settings Reference

#### Connection Options

| Setting | CLI Flag | Type | Description |
|---------|----------|------|-------------|
| `auto_connect` | `--autoConnect` | boolean | Auto-connect to running Chrome 145+ (requires remote debugging enabled at `chrome://inspect/#remote-debugging`). Conflicts with: `isolated`, `executable_path`. |
| `browser_url` | `--browserUrl` / `-u` | string | Connect to a running, debuggable Chrome instance over HTTP. Example: `"http://127.0.0.1:9222"`. Conflicts with: `ws_endpoint`. |
| `ws_endpoint` | `--wsEndpoint` / `-w` | string | Connect to a running Chrome instance over WebSocket. Example: `"ws://127.0.0.1:9222/devtools/browser/<id>"`. Conflicts with: `browser_url`. |
| `ws_headers` | `--wsHeaders` | object | Custom headers for WebSocket connection (JSON object). Only applies when `ws_endpoint` is set. Example: `{ "Authorization": "Bearer token" }`. |

#### Chrome Launch Options

| Setting | CLI Flag | Type | Description |
|---------|----------|------|-------------|
| `headless` | `--headless` | boolean | Whether to run Chrome in headless (no UI) mode. Default: `false`. |
| `executable_path` | `--executablePath` / `-e` | string | Path to a custom Chrome executable. Conflicts with: `browser_url`, `ws_endpoint`. |
| `isolated` | `--isolated` | boolean | Creates a temporary user-data-dir that is automatically cleaned up after the browser is closed. Conflicts with: `auto_connect`, `user_data_dir`. |
| `user_data_dir` | `--userDataDir` | string | Path to the user data directory (profile) for Chrome. Default: `$HOME/.cache/chrome-devtools-mcp/chrome-profile$CHANNEL_SUFFIX`. Conflicts with: `browser_url`, `ws_endpoint`, `isolated`. |
| `channel` | `--channel` | string | Chrome channel to use. Choices: `"stable"`, `"canary"`, `"beta"`, `"dev"`. Default: `"stable"`. Conflicts with: `browser_url`, `ws_endpoint`, `executable_path`. |
| `viewport` | `--viewport` | string | Initial viewport size for Chrome instances. Format: `"WIDTHxHEIGHT"` (e.g., `"1280x720"`). In headless mode, max size is 3840x2160px. |
| `chrome_arg` | `--chromeArg` | string[] | Additional arguments for Chrome. Only applies when Chrome is launched by chrome-devtools-mcp. |

#### Network Options

| Setting | CLI Flag | Type | Description |
|---------|----------|------|-------------|
| `proxy_server` | `--proxyServer` | string | Proxy server configuration for Chrome (passed as `--proxy-server`). See [Chromium network settings](https://www.chromium.org/developers/design-documents/network-settings/) for details. |
| `accept_insecure_certs` | `--acceptInsecureCerts` | boolean | If enabled, ignores errors for self-signed and expired certificates. Use with caution. |

#### Logging Options

| Setting | CLI Flag | Type | Description |
|---------|----------|------|-------------|
| `log_file` | `--logFile` | string | Path to a file to write debug logs to. Set the env variable `DEBUG=*` to enable verbose logs. Useful for submitting bug reports. |

#### Tool Category Options

| Setting | CLI Flag | Type | Description |
|---------|----------|------|-------------|
| `category_emulation` | `--categoryEmulation` | boolean | Set to `false` to exclude tools related to emulation. Default: `true`. |
| `category_performance` | `--categoryPerformance` | boolean | Set to `false` to exclude tools related to performance. Default: `true`. |
| `category_network` | `--categoryNetwork` | boolean | Set to `false` to exclude tools related to network. Default: `true`. |

#### Passthrough

| Setting | Type | Description |
|---------|------|-------------|
| `extra_args` | string[] | Extra CLI args passed verbatim to the upstream MCP server process. Use this for experimental flags or any options not modeled above. |

### Example Configurations

#### Connect to Existing Chrome Instance (HTTP)

```json
{
  "context_servers": {
    "chrome-devtools-mcp-zed": {
      "settings": {
        "browser_url": "http://127.0.0.1:9222"
      }
    }
  }
}
```

#### Connect via WebSocket with Headers

```json
{
  "context_servers": {
    "chrome-devtools-mcp-zed": {
      "settings": {
        "ws_endpoint": "ws://127.0.0.1:9222/devtools/browser/abc123",
        "ws_headers": { "Authorization": "Bearer token" }
      }
    }
  }
}
```

#### Launch Headless Chrome with Custom Viewport

```json
{
  "context_servers": {
    "chrome-devtools-mcp-zed": {
      "settings": {
        "headless": true,
        "viewport": "1920x1080",
        "channel": "stable"
      }
    }
  }
}
```

#### Use Chrome Canary with Isolated Profile

```json
{
  "context_servers": {
    "chrome-devtools-mcp-zed": {
      "settings": {
        "channel": "canary",
        "isolated": true
      }
    }
  }
}
```

#### Auto-Connect to Running Chrome 145+

```json
{
  "context_servers": {
    "chrome-devtools-mcp-zed": {
      "settings": {
        "auto_connect": true,
        "channel": "stable"
      }
    }
  }
}
```

#### Disable Sandbox (Linux Containers)

```json
{
  "context_servers": {
    "chrome-devtools-mcp-zed": {
      "settings": {
        "chrome_arg": ["--no-sandbox", "--disable-setuid-sandbox"]
      }
    }
  }
}
```

#### Disable Specific Tool Categories

```json
{
  "context_servers": {
    "chrome-devtools-mcp-zed": {
      "settings": {
        "category_emulation": false,
        "category_performance": false
      }
    }
  }
}
```

#### Debug Logging

```json
{
  "context_servers": {
    "chrome-devtools-mcp-zed": {
      "settings": {
        "log_file": "/tmp/chrome-devtools-mcp.log"
      }
    }
  }
}
```

## Running Chrome for CDP

To connect to an existing Chrome instance, start Chrome with a remote debugging port:

```bash
# Linux
google-chrome --remote-debugging-port=9222

# macOS
/Applications/Google\ Chrome.app/Contents/MacOS/Google\ Chrome --remote-debugging-port=9222

# Windows
"C:\Program Files\Google\Chrome\Application\chrome.exe" --remote-debugging-port=9222
```

Then set `browser_url` to `"http://127.0.0.1:9222"`.

## How It Works (Zed-Specific)

At runtime the extension:

1. Asks Zed for the latest available npm version of `chrome-devtools-mcp`.
2. Installs/updates the package as needed.
3. Builds CLI arguments from your settings.
4. Launches the server with Node using the package entrypoint:
   - `node_modules/chrome-devtools-mcp/build/src/index.js`

The server runs over stdio as required by MCP.

## Troubleshooting

- **Context server fails to start**:
  - Confirm Node.js is installed and accessible.
  - Check Zed logs for messages from this extension.

- **Connection refused** (when using `browser_url`):
  - Confirm Chrome is running with the matching `--remote-debugging-port`.
  - Confirm `browser_url` matches the host/port you started Chrome with.

- **WebSocket connection issues** (when using `ws_endpoint`):
  - Confirm `ws_headers` is a JSON object (not an array/string).
  - Verify the WebSocket URL includes the correct browser ID.

- **Entrypoint missing**:
  - The upstream package layout may have changed.
  - Update the wrapper to match the new entrypoint.

- **Tools not appearing**:
  - Ensure the context server is enabled in Zed settings.
  - Check if tool categories are disabled via `category_*` settings.

- **Debug logging**:
  - Set `log_file` to a path and check the output for detailed information.
  - Set the `DEBUG` environment variable to `*` for verbose logs.

## License

Apache-2.0 (matches the upstream MCP server's license)