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
| `getBasicInfo` | Get essential context about the current design: file name, page name, node count, and list of artboards with their dimensions. Call this first to understand the canvas situation. |
| `getChildren` | Get the direct children of a node. Returns a list of child nodes with their IDs, names, component types, and how many children each has. Useful for exploring the design hierarchy. |
| `getComputedStyles` | Get the computed CSS styles for one or more nodes. Returns a map of nodeld to CSSProperties object. Supports batch requests. |
| `getFillImage` | Extract the image data from a node that has an image fill. Returns the image as base64-encoded data with its MIME type. |
| `getJSX` | Get the JSX code representation of a node and its descendants. Supports two styling formats: Tailwind CSS classes (default) or inline styles. |
| `getNodeInfo` | Get detailed information about a specific node by ID, including its size, visibility, lock state, parent, children IDs, and text content (for text nodes). |
| `getScreenshot` | Capture a PNG screenshot of a specific node by ID. Returns the image as base64-encoded data. |
| `getSelection` | Get detailed information about the currently selected nodes, including IDs, names, component types, size, and which artboard they belong to. |

## How It Works

This extension uses `mcp-remote` to bridge Paper's HTTP/SSE MCP server to Zed's stdio-based context server interface. The extension automatically runs:

```bash
npx -y mcp-remote http://127.0.0.1:29979/mcp
```

## Local Use

To use this extension locally:

1. Clone this repository:
   ```bash
   git clone https://github.com/SeedOfMany/zed-mcp-server-paper.git
   cd zed-mcp-server-paper
   ```

2. Ensure Paper desktop app is running

3. In Zed, open the command palette (`Cmd+Shift+P`) and run:
   ```
   zed: install dev extension
   ```

4. Select the cloned directory when prompted

5. Add to your Zed settings (`~/.config/zed/settings.json`):
   ```json
   "context_servers": {
     "mcp-server-paper": {}
   }
   ```

6. Reload Zed to apply changes

### Debugging

Run Zed from the terminal with verbose logging:
```bash
zed --foreground
```

## About Paper

Paper is a powerful design tool for creating beautiful art and experiences. Learn more at [paper.design](https://paper.design).

## License

MIT License - see [LICENSE](LICENSE) for details.
