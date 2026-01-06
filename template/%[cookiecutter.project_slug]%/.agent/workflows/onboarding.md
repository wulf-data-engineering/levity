---
description: Onboarding guide for new developers (MCP Setup, Shell Aliases)
---

# Developer Onboarding

Welcome! This guide helps you set up your environment for maximum productivity with this template.

## 1. Shell Configuration

### Docker Compose Alias

The project relies on `docker compose` (v2), but AI assistants often try to use the legacy `docker-compose` command. To prevent friction and errors, please add an alias to your shell configuration (`~/.zshrc`, `~/.bashrc`, etc.):

```bash
alias docker-compose="docker compose"
```

## 2. Setup Model Context Protocol (MCP)

To enable your AI assistant to understand Svelte 5 (Runes), AWS resources, and more, you need to configure the **Model Context Protocol (MCP)** servers in your global settings.

### Open Configuration

You can open the configuration in two ways:

#### Option A: Via GUI

1.  Click the **Agent** tab in Antigravity.
2.  Click the **...** (Additional Options) menu.
3.  Select **MCP Servers** > **Manage MCP Servers**.
4.  Click **View Raw Config**.

#### Option B: Direct File Edit

Open the global configuration file at:
`~/.gemini/antigravity/mcp_config.json`

### Add Svelte Server

Add the following JSON snippet to the `mcpServers` object in the configuration file.

```json
"svelte": {
  "command": "npx",
  "args": [
    "-y",
    "@sveltejs/mcp"
  ]
}
```

#### Full Example

If the file is empty, it should look like this:

```json
{
  "mcpServers": {
    "svelte": {
      "command": "npx",
      "args": ["-y", "@sveltejs/mcp"]
    }
  }
}
```

### Restart

Restart your IDE window or reload the agent to apply the changes.
