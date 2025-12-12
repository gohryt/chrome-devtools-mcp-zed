# Chrome DevTools MCP (Zed) — Installation

This Zed extension integrates the upstream **Chrome DevTools MCP** server by launching it as a **Zed Context Server**.

## What this extension does

- Zed installs and runs the upstream `chrome-devtools-mcp` MCP server over **stdio**.
- All tools exposed by the MCP server are available through Zed's assistant tooling.
- Settings map directly to upstream CLI options for full configuration flexibility.

This extension does **not** bundle Chrome, does **not** require a browser extension, and does **not** add custom Zed UI beyond the context server integration.

## Prerequisites

- **Zed** editor
- **Node.js** available on your system (Zed uses Node to run the MCP server)
- (Optional) **Google Chrome / Chromium** installed, depending on your workflow

## Install as a Development Extension

1. Clone or open this repository locally.

2. In Zed:
   - Open **Extensions**
   - Choose **Install Dev Extension**
   - Select the `chrome-devtools-mcp-zed` directory

3. The context server should be enabled automatically. If not, enable it in Zed settings under `context_servers`.

## Configuration

Add settings to your Zed configuration under `context_servers`:

```json
{
  "context_servers": {
    "chrome-devtools-mcp-zed": {
      "settings": {
        // Your settings here
      }
    }
  }
}
```

### Settings Reference

#### Connection Options

| Setting | CLI Flag | Description |
|---------|----------|-------------|
| `auto_connect` | `--autoConnect` | Auto-connect to running Chrome 145+ (requires remote debugging enabled at `chrome://inspect/#remote-debugging`) |
| `browser_url` | `--browserUrl` | Connect via HTTP (e.g., `"http://127.0.0.1:9222"`) |
| `ws_endpoint` | `--wsEndpoint` | Connect via WebSocket (e.g., `"ws://127.0.0.1:9222/devtools/browser/<id>"`) |
| `ws_headers` | `--wsHeaders` | Custom headers for WebSocket (JSON object, only with `ws_endpoint`) |

#### Chrome Launch Options

| Setting | CLI Flag | Description |
|---------|----------|-------------|
| `headless` | `--headless` | Run Chrome in headless mode (no UI) |
| `executable_path` | `--executablePath` | Path to custom Chrome executable |
| `isolated` | `--isolated` | Use temporary user-data-dir (auto-cleaned on close) |
| `user_data_dir` | `--userDataDir` | Path to Chrome user data directory |
| `channel` | `--channel` | Chrome channel: `"stable"`, `"canary"`, `"beta"`, `"dev"` |
| `viewport` | `--viewport` | Initial viewport size (e.g., `"1280x720"`) |
| `chrome_arg` | `--chromeArg` | Additional Chrome arguments (array) |

#### Network Options

| Setting | CLI Flag | Description |
|---------|----------|-------------|
| `proxy_server` | `--proxyServer` | Proxy server configuration |
| `accept_insecure_certs` | `--acceptInsecureCerts` | Ignore certificate errors (use with caution) |

#### Logging Options

| Setting | CLI Flag | Description |
|---------|----------|-------------|
| `log_file` | `--logFile` | Path to debug log file (set `DEBUG=*` for verbose logs) |

#### Tool Category Options

| Setting | CLI Flag | Description |
|---------|----------|-------------|
| `category_emulation` | `--no-category-emulation` | Set `false` to disable emulation tools |
| `category_performance` | `--no-category-performance` | Set `false` to disable performance tools |
| `category_network` | `--no-category-network` | Set `false` to disable network tools |

#### Passthrough

| Setting | Description |
|---------|-------------|
| `extra_args` | Array of additional CLI args passed verbatim to upstream |

### Example Configurations

**Connect to existing Chrome instance:**

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

**Launch headless Chrome with custom viewport:**

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

**Use Chrome Canary with isolated profile:**

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

**Disable sandbox (Linux containers):**

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

## Running Chrome with Remote Debugging

To connect to an existing Chrome instance, start Chrome with remote debugging enabled:

```bash
# Linux
google-chrome --remote-debugging-port=9222

# macOS
/Applications/Google\ Chrome.app/Contents/MacOS/Google\ Chrome --remote-debugging-port=9222

# Windows
"C:\Program Files\Google\Chrome\Application\chrome.exe" --remote-debugging-port=9222
```

Then set `browser_url` to `"http://127.0.0.1:9222"`.

## Troubleshooting

- **Context server fails to start**: Confirm Node.js is installed and accessible.
- **Connection refused**: If using `browser_url`, confirm Chrome is running with matching `--remote-debugging-port`.
- **Entrypoint missing**: The upstream package layout may have changed. Check Zed logs for details.
- **Tools not appearing**: Ensure the context server is enabled in Zed settings.
- **Debug logging**: Set `log_file` to a path and check the output for detailed information.

### Linux: "Could not find DevToolsActivePort" Error

On Linux, there is a known issue with puppeteer where it looks for Chrome's user data directory at `$HOME/config` instead of `$HOME/.config` when `XDG_CONFIG_HOME` is not set.

**Solution 1: Set XDG_CONFIG_HOME (Recommended)**

Add to your shell profile (`~/.bashrc`, `~/.zshrc`, etc.):

```bash
export XDG_CONFIG_HOME="$HOME/.config"
```

Then restart Zed or your terminal session.

**Solution 2: Explicitly set user_data_dir**

Configure the extension with the correct Chrome profile path:

```json
{
  "context_servers": {
    "chrome-devtools-mcp-zed": {
      "settings": {
        "user_data_dir": "~/.config/google-chrome"
      }
    }
  }
}
```

Chrome profile paths by channel:
- **Stable**: `~/.config/google-chrome`
- **Beta**: `~/.config/google-chrome-beta`
- **Dev/Unstable**: `~/.config/google-chrome-unstable`
- **Canary**: `~/.config/google-chrome-canary`

**Solution 3: Use browser_url instead**

Connect directly to a running Chrome instance with remote debugging:

```bash
# Start Chrome with remote debugging
google-chrome --remote-debugging-port=9222
```

```json
{
  "context_servers": {
    "chrome-devtools-mcp-zed": {
      "settings": {
        "auto_connect": false,
        "browser_url": "http://127.0.0.1:9222"
      }
    }
  }
}
```