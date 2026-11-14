---
name: onboarding
description: Onboarding guide for new developers (verifies local tool requirements, git hooks, and MCP server setup).
---

# Onboarding

Create a markdown artifact greeting the developer and explaining that you will guide them through local setup. (This artifact may turn into an implementation plan if dependencies are missing.)

## 1. Requirements Check

Run the following command to check for missing requirements:

```bash
echo "git" && \
git --version && \
echo "cargo" && \
cargo --version && \
echo "cargo lambda" && \
cargo lambda --version && \
echo "cargo nextest" && \
cargo nextest --version && \
echo "npm" && \
npm --version && \
echo "docker" && \
docker --version && \
echo "docker compose" && \
docker compose version && \
echo "aws" && \
aws --version && \
echo "protoc" && \
protoc --version && \
echo "done"
```
*(Note: `cargo lambda` should be > 1.9.)*

Your agent terminal might only load `$PATH` from `~/.zshenv` (not `~/.zshrc`) or `~/.bash_profile` (not `~/.bashrc`). If few or no tools are found, run `echo $PATH` and ask the developer whether `$PATH` is configured in the wrong file; after they fix it, re-run the check.

If dependencies are missing:
- Turn the artifact into an implementation plan (`RequestFeedback: true`).
- Add tasks to install missing dependencies, noting which you can install and which require manual steps:

| Tool          | macOS                                | Linux                                | Windows                              |
| :------------ | :----------------------------------- | :----------------------------------- | :----------------------------------- |
| git           | `brew install git`                   | `sudo apt install git`               | `winget install Git.Git`             |
| cargo         | `brew install rustup`                | `curl https://sh.rustup.rs -sSf \| sh` | `winget install Rustlang.Rustup`     |
| cargo lambda  | `cargo install cargo-lambda`         | `cargo install cargo-lambda`         | `cargo install cargo-lambda`         |
| cargo nextest | `cargo install cargo-nextest --locked` | `cargo install cargo-nextest --locked` | `cargo install cargo-nextest --locked` |
| npm           | `brew install node`                  | `sudo apt install nodejs npm`        | `winget install OpenJS.NodeJS`       |
| docker        | `brew install --cask docker`         | `sudo apt install docker.io`         | `winget install Docker.DockerDesktop` |
| aws           | `brew install awscli`                | `sudo apt install awscli`            | `winget install Amazon.AWSCLI`       |
| protoc        | `brew install protobuf`              | `sudo apt install protobuf-compiler` | `winget install protobuf`            |

## 2. Git Repository & Hooks

Check if this directory is a git repository (note it in the artifact if not).
Suggest that the developer activate the project's `pre-commit` and `pre-push` hooks:

```bash
git config core.hooksPath .githooks
```

## 3. MCP Servers

Check which MCP servers are already available by inspecting your available tools. Do **NOT** read the user's global config file directly as it contains secrets.

- Briefly list already-configured suggested MCP servers in the artifact.
- Explain any missing suggested MCP servers from `@./assets/mcp_server_template.json`, including how to obtain each token/credential.
- Explain that the developer should manually merge the JSON into `~/.gemini/config/mcp_config.json` for security reasons.

## 4. Docker Compose Alias

Include this note in the artifact:

> The project relies on `docker compose` (v2), but AI assistants often try to use the legacy `docker-compose` command. To prevent friction and errors, add an alias to your shell configuration (`~/.zshrc`, `~/.bashrc`, etc.):
> ```bash
> alias docker-compose="docker compose"
> ```

## 5. Summary

Summarize the artifact in your response:
- If dependencies are missing, ask to proceed with the implementation plan and clarify what you will run vs. what the developer must do manually.
- If no dependencies are missing, onboarding is complete.
- If there is no GitHub remote yet, suggest continuing with `/initial-setup` (`@../initial-setup/SKILL.md`).
