## Paper MCP Server

This extension connects Zed to [Paper](https://paper.design)'s MCP server, enabling AI-powered design-to-code workflows.

### Requirements

1. **Paper desktop app** must be running (exposes MCP at `http://127.0.0.1:29979/mcp`)
2. **Node.js** must be installed (for the `mcp-remote` bridge)

### Configuration

Add the following to your Zed settings (`~/.config/zed/settings.json`):

```json
"context_servers": {
  "mcp-server-paper": {}
}
```
