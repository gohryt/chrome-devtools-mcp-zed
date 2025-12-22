# Chrome DevTools MCP — Installation

## Quick Start

1. **Install the extension** in Zed:
   - Open Extensions (Cmd/Ctrl + Shift + X)
   - Search for "Chrome DevTools MCP"
   - Click Install

2. **Start using it** — no configuration required:
   - The extension automatically installs the `chrome-devtools-mcp` npm package
   - Chrome launches automatically when a tool requires it

## Prerequisites

- **Node.js** v20 or newer
- **Google Chrome** (stable channel recommended)

## Configuration (Optional)

For advanced use cases, add settings to your Zed configuration:

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

### Common Settings

| Setting | Type | Description |
|---------|------|-------------|
| `browser_url` | string | Connect to existing Chrome (e.g., `"http://127.0.0.1:9222"`) |
| `channel` | string | Chrome channel: `"stable"`, `"beta"`, `"dev"`, `"canary"` |
| `headless` | boolean | Run Chrome without UI |
| `isolated` | boolean | Use temporary profile (auto-cleaned) |
| `viewport` | string | Initial size (e.g., `"1920x1080"`) |
| `chrome_arg` | string[] | Additional Chrome arguments |
| `log_file` | string | Path for debug logs |

### Example: Connect to Existing Chrome

```bash
# Start Chrome with remote debugging
google-chrome --remote-debugging-port=9222 --user-data-dir=/tmp/chrome-debug
```

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

### Example: Headless Mode

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

## User Data Directory

Chrome profile is stored at `~/.cache/chrome-devtools-mcp/chrome-profile` (persistent across sessions).

Set `"isolated": true` for a temporary profile that's cleaned up when Chrome closes.

## Troubleshooting

### Chrome doesn't launch

- Verify Chrome is installed and accessible
- Try setting `executable_path` to your Chrome binary

### Linux: "Could not find DevToolsActivePort"

Add to `~/.bashrc`:

```bash
export XDG_CONFIG_HOME="$HOME/.config"
```

Or use `browser_url` to connect to a manually-started Chrome instance.

### Debug logging

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

## More Information

- [Upstream documentation](https://github.com/ChromeDevTools/chrome-devtools-mcp)
- [Tool reference](https://github.com/ChromeDevTools/chrome-devtools-mcp/blob/main/docs/tool-reference.md)
