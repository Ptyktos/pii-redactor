# Security policy

## Report a vulnerability

Email **security@ptyktos.com** to report a suspected vulnerability. Do not
open a public issue for an unpatched vulnerability. Include the affected
version or commit, impact, and steps to reproduce.

Once GitHub private vulnerability reporting is enabled in repository
settings, reports can also be submitted through
[GitHub Security Advisories](https://github.com/Ptyktos/pii-redactor/security/advisories/new).

## Supported versions

Before the first stable release, security fixes are made against the latest
`main` branch.

## Security-relevant behavior

This app's actual security property is **it makes no network call**. All
detection runs on-device (WebAssembly in the browser tab, or a native
webview on desktop/mobile) against the same `pii-core` engine
[`pii-svc`](https://github.com/Ptyktos/pii-svc) runs server-side, minus the
ONNX/NER model pass (no model ships in a browser or phone). No account, no
telemetry, no upload of the text you paste in.

That property is easy to silently break: any future change that adds a
`fetch`/`reqwest` call, a crash reporter, an analytics SDK, or a "check for
updates" ping would change what this app actually does without changing what
the README claims. Treat adding outbound network capability here as a
security-relevant change requiring explicit review and a README update, not
a routine feature addition.

The clipboard/download code path passes user-supplied text through
`serde_json` before handing it to JS (see `src/main.rs`) — that's the one
place untrusted input crosses a serialization boundary in this app.

## Dependency security

Rust dependencies are recorded in `Cargo.lock`. Pull requests and scheduled
workflows run RustSec advisory checks and cargo-deny policy checks. `pii-core`
is pulled in as a git dependency from `Ptyktos/pii-svc` — its own security
policy covers the detection engine itself.
