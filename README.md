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
  "paper": {}
}
```

4. Restart Zed or reload the window

## Available Tools

Once connected, the following tools are available to Zed's AI assistant:

| Tool | Description |
|------|-------------|
| `getBasicInfo` | Get essential context about the current design: file name, page name, node count, and list of artboards with their dimensions. Call this first to understand the canvas situation. |
<<<<<<< HEAD
| `getChildren` | Get the direct children of a node. Returns a list of child nodes with their IDs, names, component types, and how many children each has. Useful for exploring the design hierarchy. |
| `getComputedStyles` | Get the computed CSS styles for one or more nodes. Returns a map of nodeld to CSSProperties object. Supports batch requests. |
| `getFillImage` | Extract the image data from a node that has an image fill. Returns the image as base64-encoded data with its MIME type. |
| `getJSX` | Get the JSX code representation of a node and its descendants. Supports two styling formats: Tailwind CSS classes (default) or inline styles. |
| `getNodeInfo` | Get detailed information about a specific node by ID, including its size, visibility, lock state, parent, children IDs, and text content (for text nodes). |
| `getScreenshot` | Capture a PNG screenshot of a specific node by ID. Returns the image as base64-encoded data. |
| `getSelection` | Get detailed information about the currently selected nodes, including IDs, names, component types, size, and which artboard they belong to. |
=======
| `getSelection` | Get detailed information about the currently selected nodes, including IDs, names, component types, size, and which artboard they belong to. |
| `getNodeInfo` | Get detailed information about a specific node by ID, including its size, visibility, lock state, parent, children IDs, and text content (for text nodes). |
| `getChildren` | Get the direct children of a node. Returns a list of child nodes with their IDs, names, component types, and how many children each has. Useful for exploring the design hierarchy. |
| `getScreenshot` | Capture a PNG screenshot of a specific node by ID. Returns the image as base64-encoded data. |
| `getJSX` | Get the JSX code representation of a node and its descendants. Supports two styling formats: Tailwind CSS classes (default) or inline styles. |
| `getTreeSummary` | Get a compact text summary of a node's subtree hierarchy. Returns an indented tree showing each node's component type, name, ID, and dimensions. Much cheaper than getJSX for understanding structure — use this for orientation before diving into specific nodes. |
| `getComputedStyles` | Get the computed CSS styles for one or more nodes. Returns a map of nodeId to CSSProperties object. Supports batch requests. |
| `findPlacement` | Find a blank position on the canvas to place a new artboard. Returns x/y coordinates that avoid overlapping existing artboards. Call this before creating a new artboard with writeHTML. |
| `getFillImage` | Extract the image data from a node that has an image fill. Returns the image as base64-encoded data with its MIME type. |
| `writeHTML` | Write HTML into the design. The HTML is parsed and converted into Paper design nodes. Two modes available: "insert-children" adds nodes as children of the target, "replace" removes the target and inserts the parsed HTML in its place. |
| `createArtboard` | Create a new artboard (top-level frame) on the canvas. Automatically positions it in a blank area that does not overlap existing artboards. Returns the artboard ID which you can then use with writeHTML insert-children to add content. |
| `deleteNodes` | Delete one or more nodes from the design. Also deletes all descendants of the specified nodes. |
| `setTextContent` | Set the text content of one or more Text nodes. Only works on nodes with component type "Text". Use this instead of writeHTML replace when you only need to change text. Supports batch updates in a single call. |
>>>>>>> 4dab32b (Updated README)

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
     "paper": {}
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
