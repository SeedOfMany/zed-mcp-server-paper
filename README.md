# Paper MCP Server Extension for Zed

This extension integrates [Paper](https://paper.design)'s MCP Server as a context server for [Zed's](https://zed.dev) Agent Panel.

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
