# Chrome DevTools MCP for Zed

A Zed extension that integrates the [Chrome DevTools MCP](https://github.com/ChromeDevTools/chrome-devtools-mcp) server, giving your AI assistant access to Chrome's automation, debugging, and performance analysis capabilities.

## Features

- **Zero configuration** — Chrome launches automatically when needed
- **Full DevTools access** — Network inspection, console logs, screenshots, performance traces
- **Browser automation** — Navigate, click, fill forms, handle dialogs
- **All upstream tools** — Direct access to 26+ MCP tools from chrome-devtools-mcp

## Prerequisites

- [Zed](https://zed.dev/) editor
- [Node.js](https://nodejs.org/) v20 or newer
- [Google Chrome](https://www.google.com/chrome/) (stable channel)

## Installation

### From Zed Extensions (when published)

1. Open Zed → Extensions
2. Search for "Chrome DevTools MCP"
3. Click Install

### Development Installation

1. Clone this repository
2. In Zed: Extensions → Install Dev Extension
3. Select the `chrome-devtools-mcp-zed` directory

## Usage

No configuration required. The extension automatically:

1. Installs/updates the `chrome-devtools-mcp` npm package
2. Launches Chrome when a tool requires it
3. Manages a persistent browser profile at `~/.cache/chrome-devtools-mcp/chrome-profile`

### Example Prompts

```
Take a screenshot of https://example.com
```

```
Check the performance of https://zed.dev
```

```
List all network requests on the current page
```

## Configuration (Optional)

Add settings to your Zed configuration for advanced use cases:

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

### Connection Options

| Setting | Type | Description |
|---------|------|-------------|
| `browser_url` | string | Connect to running Chrome via HTTP (e.g., `"http://127.0.0.1:9222"`) |
| `ws_endpoint` | string | Connect via WebSocket |
| `ws_headers` | object | Custom headers for WebSocket connection |
| `auto_connect` | boolean | Auto-connect to Chrome 145+ with remote debugging enabled |

### Chrome Launch Options

| Setting | Type | Description |
|---------|------|-------------|
| `channel` | string | Chrome channel: `"stable"`, `"beta"`, `"dev"`, `"canary"` |
| `headless` | boolean | Run Chrome without UI |
| `isolated` | boolean | Use temporary profile (cleaned up on close) |
| `user_data_dir` | string | Custom profile directory path |
| `viewport` | string | Initial viewport size (e.g., `"1920x1080"`) |
| `executable_path` | string | Path to custom Chrome executable |
| `chrome_arg` | string[] | Additional Chrome launch arguments |

### Other Options

| Setting | Type | Description |
|---------|------|-------------|
| `proxy_server` | string | Proxy server configuration |
| `accept_insecure_certs` | boolean | Ignore certificate errors |
| `log_file` | string | Path for debug logs |
| `category_emulation` | boolean | Enable/disable emulation tools |
| `category_performance` | boolean | Enable/disable performance tools |
| `category_network` | boolean | Enable/disable network tools |
| `extra_args` | string[] | Additional CLI arguments for upstream |

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

**Headless Chrome with custom viewport:**

```json
{
  "context_servers": {
    "chrome-devtools-mcp-zed": {
      "settings": {
        "headless": true,
        "viewport": "1920x1080"
      }
    }
  }
}
```

**Temporary isolated profile:**

```json
{
  "context_servers": {
    "chrome-devtools-mcp-zed": {
      "settings": {
        "isolated": true
      }
    }
  }
}
```

## User Data Directory

By default, Chrome uses a persistent profile at:

| Platform | Path |
|----------|------|
| Linux/macOS | `~/.cache/chrome-devtools-mcp/chrome-profile` |
| Windows | `%USERPROFILE%\.cache\chrome-devtools-mcp\chrome-profile` |

For non-stable channels, the directory includes the channel name (e.g., `chrome-profile-canary`).

Set `isolated: true` to use a temporary profile that's automatically cleaned up.

## Connecting to a Running Chrome Instance

To connect to an existing Chrome instance instead of launching a new one:

**1. Start Chrome with remote debugging:**

```bash
# Linux
google-chrome --remote-debugging-port=9222 --user-data-dir=/tmp/chrome-debug

# macOS
/Applications/Google\ Chrome.app/Contents/MacOS/Google\ Chrome --remote-debugging-port=9222 --user-data-dir=/tmp/chrome-debug

# Windows
chrome.exe --remote-debugging-port=9222 --user-data-dir=%TEMP%\chrome-debug
```

**2. Configure the extension:**

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

## Troubleshooting

### Chrome doesn't launch

- Ensure Google Chrome is installed
- Check that Node.js v20+ is available
- Try setting `executable_path` to your Chrome binary location

### Connection refused

- Verify Chrome is running with `--remote-debugging-port=9222`
- Ensure no firewall is blocking localhost connections

### Linux: "Could not find DevToolsActivePort"

This occurs when using `auto_connect` due to a puppeteer bug with `XDG_CONFIG_HOME`. Solutions:

1. **Set XDG_CONFIG_HOME** (add to `~/.bashrc`):
   ```bash
   export XDG_CONFIG_HOME="$HOME/.config"
   ```

2. **Or use `browser_url`** instead of `auto_connect`

### Debug logging

Enable detailed logs:

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

## License

Apache-2.0 (same as upstream chrome-devtools-mcp)
