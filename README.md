# Paper MCP Server Extension for Zed

This extension integrates [Paper](https://paper.design)'s MCP Server as a context server for [Zed's](https://zed.dev) Agent Panel, enabling AI-powered design-to-code workflows.

## Prerequisites

- **Paper desktop app** must be running (exposes MCP at `http://127.0.0.1:29979/mcp`)
- **Node.js** must be installed (for the `mcp-remote` bridge)

## Installation

1. Install the extension from Zed's extension marketplace
2. Ensure Paper desktop app is running
3. Add the following to your Zed settings (`~/.config/zed/settings.json`):

```json
"context_servers": {
  "mcp-server-paper": {}
}
```

4. Restart Zed or reload the window

## Available Tools

Once connected, the following tools are available to Zed's AI assistant:

| Tool | Description |
|------|-------------|
| `getBasicInfo` | Get essential context about the current design: file name, page name, node count, and list of artboards with their dimensions. Call this first to understand the canvas. |
| `getChildren` | Get the direct children of a node. Returns IDs, names, component types, and child counts. Useful for exploring the design hierarchy. |
| `getComputedStyles` | Get computed CSS styles for one or more nodes. Returns a map of nodeId to CSSProperties. Supports batch requests. |
| `getFillImage` | Extract image data from a node with an image fill. Returns base64-encoded data with MIME type. |
| `getJSX` | Get JSX code representation of a node and its descendants. Supports Tailwind CSS classes (default) or inline styles. |
| `getNodeInfo` | Get detailed info about a specific node: size, visibility, lock state, parent, children IDs, and text content. |
| `getScreenshot` | Capture a PNG screenshot of a specific node. Returns base64-encoded data. |
| `getSelection` | Get info about currently selected nodes: IDs, names, component types, size, and which artboard they belong to. |

## How It Works

This extension uses `mcp-remote` to bridge Paper's HTTP/SSE MCP server to Zed's stdio-based context server interface. The extension automatically runs:

```bash
npx -y mcp-remote http://127.0.0.1:29979/mcp
```

## Development

To develop this extension locally:

1. Clone this repository
2. In Zed, run the command: `zed: install dev extension`
3. Select this directory

For debugging, run Zed with verbose logging:
```bash
zed --foreground
```

## About Paper

Paper is a powerful design tool for creating beautiful art and experiences. Learn more at [paper.design](https://paper.design).

## License

MIT License - see [LICENSE](LICENSE) for details.
