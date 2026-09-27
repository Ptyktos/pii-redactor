# pii-redactor — local PII & secret redaction app

A [Dioxus](https://dioxuslabs.com) app that strips PII and secrets from text
**entirely on the device** — web, desktop, Android and iOS from this one Rust
codebase. The detection engine is [`pii-core`](https://github.com/Ptyktos/pii-svc/tree/main/crates/pii-core)
(the same one [`pii-svc`](https://github.com/Ptyktos/pii-svc) runs, pulled in
here as a git dependency), compiled in with `default-features = false` so it's
the pure-Rust passes only — **regex+checksum, secrets, intl national IDs,
identity denylist, and the optional resonance pass** — no ONNX model. On
the web target this runs as WebAssembly in the tab; on desktop/mobile it runs
natively. **Nothing is uploaded — no network call, no account, no telemetry.**

> The model-backed NER pass is intentionally absent here (a GLiNER ONNX model has
> no place in a browser/phone). For that breadth, point at a `pii-svc` instance
> or the [Worker](https://github.com/Ptyktos/pii-svc/tree/main/worker)'s `POST /redact`.

## Prerequisites

- Rust (stable) + the wasm target: `rustup target add wasm32-unknown-unknown`
- The Dioxus CLI: `cargo install dioxus-cli@0.7` (provides `dx`)

`dx` downloads three helper tools on first run (`wasm-bindgen`, `wasm-opt`,
`esbuild`). On a locked-down network, put those on `PATH` and export
`NO_DOWNLOADS=1` so `dx` uses them instead of fetching.

## Run / build

```bash
dx serve --web                 # dev server with hot reload  → http://localhost:8080
dx build --web --release       # production bundle → target/dx/pii-redactor/release/web/public
```

Same codebase, other targets (see notes below):

```bash
dx serve  --desktop            # native window (webview renderer)
dx build  --android --release  # .apk / .aab
dx build  --ios --release      # .app / .ipa
```

## Targets

| target | status | prerequisites |
|---|---|---|
| **web** (wasm) | built & verified in CI | wasm target + `dx` |
| **desktop** | scaffolded (features wired) | Linux: `webkit2gtk-4.1` + `libxdo`; macOS/Windows: system webview |
| **android** | scaffolded (features wired) | Android SDK + NDK (`ANDROID_NDK_HOME`), JDK |
| **ios** | scaffolded (features wired) | macOS + Xcode, an Apple signing identity |

The renderer is HTML/CSS on every target (Dioxus uses a webview on desktop and
mobile), so the UI and the clipboard / download behaviour are identical
everywhere — only the build toolchains differ. Desktop and mobile builds need
SDKs that aren't present in this repo's CI, so they're left as `dx` one-liners
rather than wired into the `apps` workflow.

## What the controls do

- **Secrets & IDs** (on) — the `pii-core` regex+checksum / secrets / intl passes:
  emails, phones, IPs, credit cards (Luhn), AU TFN/ABN/Medicare, SSN/IBAN/national
  IDs, API keys, PEM private keys, JWTs, `keyword=VALUE` secrets.
- **Aggressive (catch unknown)** (off) — the resonance prose-membership pass
  (see [`pii-svc`'s docs](https://github.com/Ptyktos/pii-svc/tree/main/docs)).
  Catches high-entropy / never-before-seen secrets and odd spans the patterns
  miss, at the cost of more false positives. Uses bundled English prose as its
  model (no download).
- **Advanced → custom names** — a literal denylist (one term per line) redacted as
  `[CUSTOM]`, for project names / handles / people the detectors don't know.

## Layout

```
Cargo.toml      standalone workspace; dioxus + pii-core (git dependency, no-default-features)
Dioxus.toml     app name + static <title>
src/main.rs     the App component, redaction wiring, copy/download
src/styles.rs   the stylesheet (one trusted constant)
```

## Provenance

Split out of [`Ptyktos/pii-svc`](https://github.com/Ptyktos/pii-svc)'s `app/`
directory (2026-09-27) so this client app can version and release
independently of the HTTP service. Full history preserved.
