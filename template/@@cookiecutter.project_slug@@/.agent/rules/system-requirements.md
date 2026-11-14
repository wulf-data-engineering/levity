---
trigger: glob
globs: "**/Cargo.toml, **/rust-toolchain*, **/package.json, .nvmrc, docker-compose*.yml, .github/workflows/**"
description: Keep the onboarding skill in sync when project tool requirements change.
---

# System Requirement Rules

If the tool or runtime requirements of the software change, you **MUST** update `@../skills/onboarding/SKILL.md` to reflect those changes.
