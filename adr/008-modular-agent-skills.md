# ADR‑008 — Modular Agent Skills & Skill Derivation

**Status**: Proposed  
**Date**: 2026‑10‑28

---

## 1. Context & Problem Statement

As the tool‑set evolves, applications frequently require sophisticated, cross‑cutting capabilities such as real‑time WebSockets with SQS background workers, client‑side Rust WebAssembly (WASM) modules, or autonomous CI pipelines for Dependabot pull requests.

Baking every advanced architectural capability directly into the default Cookiecutter blueprint would bloat generated projects, increase baseline AWS infrastructure and CI costs, and burden simple prototypes with unused components. Conversely, leaving developers to wire these integrations from scratch sacrifices the speed and production‑readiness of the tool‑set.

---

## 2. Design & Technology Decision

We keep the base Cookiecutter blueprint intentionally lean and package domain expertise and optional cross‑cutting architectures as **Antigravity Agent Skills** (`SKILL.md` + `assets/`):

| Skill Category | Purpose | Structure |
| -------------- | ------- | --------- |
| **Core Domain Skills** | Guide day‑to‑day development across the default layers (frontend, backend, database, protocols) alongside workspace rules and operational workflows. | `.agent/skills/<layer>/SKILL.md` |
| **Opt‑In Feature Skills** | On‑demand architectural extensions (e.g., `websockets-integration`, `wasm-integration`, `antigravity-dependabot`) applied by the agent only when requested by the developer. | `.agent/skills/<feature>/SKILL.md` + parameterized `assets/lib/` and `assets/examples/` |
| **Template Evolution Skills & Meta‑Workflow** | Guide maintenance of the template repository itself (`template_development`, `dependabot_changes`) and extract new opt‑in skills from working test instances via `/derive-skill`. | `/.agent/skills/` & `/.agent/workflows/derive-skill.md` |

### Skill Derivation Meta‑Workflow (`/derive-skill`)

Instead of hand‑crafting skills or using brittle raw git diffs, new opt‑in skills are systematically derived from a verified template instance (`../levity-instances/<project_slug>`):
1. **Develop & Verify in Instance**: The feature is implemented, formatted, linted, and tested end‑to‑end inside a fresh template instance.
2. **Extract New Files as Parameterized Assets**: Entirely new library or example files are copied into `template/@@cookiecutter.project_slug@@/.agent/skills/<skill-name>/assets/` and re‑parameterized with custom Cookiecutter delimiters (`@@ cookiecutter.project_slug @@`).
3. **Encode Modifications as Semantic Instructions**: Changes to existing baseline files (e.g., `Cargo.toml`, `package.json`, CDK stacks, CI workflows) are documented as semantic, step‑by‑step instructions in `SKILL.md` using inline `/* NEW */` markers.
4. **Round‑Trip Verification**: A second fresh template instance is generated and an agent applies the newly derived `SKILL.md` from scratch to prove end‑to‑end reproducibility before committing.

---

## 3. Tool‑Set Support

- `/.agent/workflows/derive-skill.md`: Step‑by‑step meta‑workflow for extracting verified instance changes into reusable template skills.
- `template/@@cookiecutter.project_slug@@/.agent/skills/`:
  - Core domain skills for the default application stack.
  - Opt‑in feature skills (`wasm-integration`, `websockets-integration`, `antigravity-dependabot`) with ready‑to‑copy `assets/`.
- `scripts/check_defaults.py`: CI guard ensuring skill assets and template files properly substitute Cookiecutter placeholders.

---

## 4. Consequences

- **Pros**: Base template remains minimal and fast to deploy; complex architectures can be added reliably in minutes by an agent; semantic `SKILL.md` instructions survive surrounding code changes far better than static patches or Cookiecutter boolean flags.
- **Cons**: Opt‑in skills require periodic verification when major underlying framework conventions change.

### Alternatives Considered

- **Cookiecutter boolean feature flags (`use_websockets`, `use_wasm`, …)** – Leads to combinatorial explosion of `@= if ... =@` Jinja blocks across dozens of files and cannot be enabled easily *after* a project has already been generated.
- **Git branches per feature** – Quickly diverges from `main` and cannot be composed when an application needs multiple features together.
