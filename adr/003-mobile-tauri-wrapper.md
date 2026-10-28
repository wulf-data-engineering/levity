# ADR-003 — Mobile Distribution via Tauri Wrapping the Web Codebase

**Status**: Proposed
**Date**: 2025‑07‑11
**Deciders**: Tool‑set maintainers

---

## 1. Design & Technology Decision

All first‑party mobile apps **SHALL be produced by wrapping the Svelte web bundle inside a Tauri‑Mobile shell**. The Tauri layer provides the WebView container, minimal Rust sidecar for native bridges, and platform packaging for **iOS (UIKit WebKit) and Android (WebView)**.

| Aspect            | Choice                                                                                                     |
| ----------------- | ---------------------------------------------------------------------------------------------------------- |
| Wrapper framework | **Tauri‑Mobile β** (Rust core + Swift/Kotlin stubs)                                                        |
| Web assets        | Same `build/` folder from ADR‑001 SSG process                                                              |
| Native bridges    | Rust commands exposed via Tauri IPC for: secure storage, file picker, push notifications (when supported)  |
| State persistence | IndexedDB + optional encrypted store via Tauri plugin                                                      |
| Updates           | Full‑app updates through App Store / Play Store; hot‑reload disabled for production per store policy       |
| CI packaging      | Manual Fastlane lanes (`fastlane ios beta`, `fastlane android beta`) triggered by GitHub workflow dispatch |
| Minimum OS        | Modern iOS and Android WebView targets                                                                     |

**Why not React‑Native / Capacitor?**
Keeping one Svelte codebase avoids divergence between web and mobile UIs, and Rust aligns with the rest of the stack for native extensions.

---

## 2. Intended Usage in Applications

1. **Generate** a mobile project with `/templates/mobile` (prompts for bundle ID, display name, icons).
2. **Embed web build**: `cp ../web/build/* mobile/src-tauri/dist/` during the mobile packaging task.
3. **Bridge pattern**: define a Rust command in `src-tauri/src/bridge.rs` and call it from Svelte with `window.__TAURI__.invoke('command', { ... })`.
4. **Local dev**: `deno task dev:mobile` runs `vite dev` and launches Tauri in hot‑reload mode.
5. **Versioning**: mobile semver follows backend major.minor; patch reserved for store resubmissions.
6. **Testing**: Playwright E2E matrix runs against `tauri dev --target ios-sim` and `android‑emulator`.
7. **Release**:

   * Tag `vX.Y.Z-mobile` → GitHub Action builds signed IPA/AAB → uploads to TestFlight / Internal App Sharing.
   * Manual approval in App Store / Play Console completes rollout.

Guidelines:

| Concern            | Guideline                                                                                |
| ------------------ | ---------------------------------------------------------------------------------------- |
| Beta maturity      | Track Tauri‑Mobile releases; update template when a stable 1.0 ships.                    |
| Push notifications | Use Firebase (FCM) on Android; APNs with Tauri plugin on iOS (when GA).                  |
| Deep links         | Configure `tauri.conf.json` → `app.protocols` and register URL scheme in store listings. |
| Offline mode       | Serve cached SSG pages; queue API writes in IndexedDB and flush when online.             |

---

## 3. Tool‑Set Support

| Component                                        | How it supports the decision                                                                                                                   |
| ------------------------------------------------ | ---------------------------------------------------------------------------------------------------------------------------------------------- |
| **`/templates/mobile`**                          | Cookiecutter scaffold: Tauri project, pre‑wired Rust bridges, icons, Fastlane lanes, GH Action stub.                                           |
| **`/libraries/mobile/bridge-common`**            | Shared Rust crate with helpers: secure storage, encrypted prefs, protobuf encoder/decoder.                                                     |
| **`/iac/cdk/push-topics.ts`**                    | CDK construct to create SNS/APNs keys and FCM tokens, outputs to Secrets Manager for mobile build.                                             |
| **CI workflow** (`.github/workflows/mobile.yml`) | Matrix of iOS/Android: installs Tauri CLI, Rust, Xcode / Android SDK; builds, signs, runs Playwright E2E in emulator, stores artifact IPA/AAB. |
| **Docs** (`/docs/tauri-mobile-guidelines.md`)    | Store checklist, icon sizes, push setup, debugging tips.                                                                                       |
| **Antigravity agent guidance**                   | Hints: “Prefer Tauri command bridge for native; avoid DOM‑breaking plugins; keep packaging size <40 MB.”                                       |

---

### Consequences

* **Pros**: single UI codebase, small binary size (<15 MB), Rust native modules, consistent offline behavior.
* **Cons**: dependent on Tauri‑Mobile beta stability; limited native plugin ecosystem compared to Capacitor; some OS APIs may require custom Rust work.

### Alternatives Considered

* **Capacitor** – mature plugin ecosystem but heavier JS runtime, diverges from Rust tooling.
* **React‑Native** – performant UI but duplicates logic across frameworks and increases bundle size.
