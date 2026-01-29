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

### Available Tools

Once connected, the following tools are available to Zed's AI assistant:

| Tool | Description |
|------|-------------|
| `getBasicInfo` | Get essential context about the current design: file name, page name, node count, and list of artboards with their dimensions |
| `getChildren` | Get the direct children of a node with IDs, names, component types, and child counts |
| `getComputedStyles` | Get computed CSS styles for one or more nodes (supports batch requests) |
| `getFillImage` | Extract image data from a node with an image fill (returns base64-encoded data) |
| `getJSX` | Get JSX code representation of a node with Tailwind CSS or inline styles |
| `getNodeInfo` | Get detailed node info: size, visibility, lock state, parent, children, and text content |
| `getScreenshot` | Capture a PNG screenshot of a specific node (returns base64-encoded data) |
| `getSelection` | Get info about currently selected nodes: IDs, names, types, size, and artboard |
