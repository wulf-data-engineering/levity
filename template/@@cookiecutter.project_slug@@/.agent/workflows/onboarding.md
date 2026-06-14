---
description: Onboarding guide for new developers (requirements, MCP server setup)
---

# Onboarding

Create a markdown artifact with greeting to developer that you will guide through setup.
That artifact might turn into an implementation plan later.

# Requirements

Run command exactly to check for missing requirements:

// turbo-all

```bash
echo "git" && \
git --version && \
echo "cargo" && \
cargo --version && \
echo "cargo lambda" && \
cargo lambda --version && \ # Should be above 1.9
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

Your agent terminal might just see paths from `~/.zshenv` (not `~/.zshrc`) or `~/.bash_profile` (not `~/.bashrc`).
If users specify their `$PATH` in the wrong file, you might not see installed tools.
I you see few or no dependencies use `echo $PATH` to check the path and ask the developer wether `$PATH` is in the wrong file.
If the developer fixed path issues, run `source ~/.zshenv` (or your respective file) and run the requirements check again.

If there are missing dependencies:

- Turn artifact into implementation plan
- Add tasks to install missing dependencies to the plan
- State which you can and will install
- State in detail which need to be installed manually and how

| Tool          | mac                                  | linux                                | win                                  |
| ------------- | ------------------------------------ | ------------------------------------ | ------------------------------------ |
| git           | brew install git                     | sudo apt install git                 | winget install Git.Git               |
| cargo         | brew install rustup                  | curl https://sh.rustup.rs -sSf \| sh | winget install Rustlang.Rustup       |
| cargo lambda  | cargo install cargo-lambda           | cargo install cargo-lambda           | cargo install cargo-lambda           |
| cargo nextest | cargo install cargo-nextest --locked | cargo install cargo-nextest --locked | cargo install cargo-nextest --locked |
| npm           | brew install node                    | sudo apt install nodejs npm          | winget install OpenJS.NodeJS         |
| docker        | brew install --cask docker           | sudo apt install docker.io           | winget install Docker.DockerDesktop  |
| aws           | brew install awscli                  | sudo apt install awscli              | winget install Amazon.AWSCLI         |
| protoc        | brew install protobuf                | sudo apt install protobuf-compiler   | winget install protobuf              |

# Git Repo

Check if this repository is a git repository.
If not mention that in the artifact.

Suggest the developer to run the following command to activate the project's pre-commit and pre-push hooks:

```bash
git config core.hooksPath .githooks
```

# MCP servers

Check which MCP servers are already available by inspecting your available tools.
Do NOT try to read the config file directly as it contains secrets.

Mention briefly the existing suggested MCP servers in artifact.
Explain missing suggested MCP servers from @mcp_server_template.json in artifact.

For each credentials/token explain separately where and how to obtain.

Explain developer has to merge json manually with ~/.gemini/antigravity/mcp_config.json for security reasons.

# Docker Compose Alias

Add following information exactly to artifact:

The project relies on `docker compose` (v2), but AI assistants often try to use the legacy `docker-compose` command. To prevent friction and errors, please add an alias to your shell configuration (`~/.zshrc`, `~/.bashrc`, etc.):

```bash
alias docker-compose="docker compose"
```

# Summary

Summarize artifact in final output.
If there are missing dependencies ask to proceed with implementation plan.
Clarify what you do at "proceed" and what dev has to do.
If there are none there is no need to proceed: Workflow done.

If there is no github repository, this is a new project.
Suggest to continue with @initial-setup.md workflow.
