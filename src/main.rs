//! `pii-redactor` — a Dioxus app that redacts PII & secrets **entirely on the
//! device**. The exact same `pii-core` engine the `pii-svc` server runs is
//! compiled into this binary (regex+checksum, secrets, intl IDs, identity
//! denylist, and the optional resonance pass — everything except the ONNX/model
//! NER pass, which has no place in a browser/phone). Text is never sent anywhere:
//! on the web target the redaction runs in WebAssembly in the tab; on desktop and
//! mobile it runs natively. No network, no account, no telemetry.
//!
//! One codebase, every target: `dx serve --web | --desktop | --android | --ios`.

use dioxus::prelude::*;
use pii_core::{Engine, Finding, IdentityList, PassConfig, RedactResponse};

mod styles;

const EXAMPLE: &str = "Hi, I'm Jane Doe — reach me at jane.doe@example.com or +61 412 345 678.\nMy TFN is 123 456 782 and Medicare 2428 77813 1.\nPay to card 4111 1111 1111 1111. The box is at 10.0.0.42.\nDeploy key: AKIAIOSFODNN7EXAMPLE  token=sk-Abc123Def456Ghi789Jkl012Mno345Pqr678.\n-----BEGIN OPENSSH PRIVATE KEY-----\nb3BlbnNzaC1rZXktdjEAAAAA\n-----END OPENSSH PRIVATE KEY-----";

fn main() {
    dioxus::launch(App);
}

/// Run the configured passes over `input`, building the engine from the current
/// UI state. Cheap enough to call on every keystroke (no model, no I/O).
fn redact(input: &str, regex: bool, aggressive: bool, terms_text: &str) -> RedactResponse {
    let terms: Vec<(String, String)> = terms_text
        .lines()
        .map(str::trim)
        .filter(|l| !l.is_empty())
        .map(|l| (l.to_owned(), "CUSTOM".to_owned()))
        .collect();
    let identity = if terms.is_empty() {
        None
    } else {
        IdentityList::from_terms(&terms)
    };
    let passes = PassConfig {
        regex,
        ner: false, // no model on-device
        identity: identity.is_some(),
        resonance: aggressive, // resonance::Model falls back to bundled prose off-web
    };
    Engine::new(passes, None, identity).redact(input)
}

/// A run of the original text: either plain or a detected (`hit`) span.
struct Seg {
    hit: bool,
    text: String,
    ty: String,
}

/// Split the original into alternating plain / detected segments for the
/// highlighted view. Byte offsets from `findings` are snapped to char
/// boundaries so multi-byte text can never panic the slice.
fn segments(original: &str, findings: &[Finding]) -> Vec<Seg> {
    let snap = |mut i: usize| {
        i = i.min(original.len());
        while i < original.len() && !original.is_char_boundary(i) {
            i += 1;
        }
        i
    };
    let mut out = Vec::new();
    let mut cursor = 0usize;
    for f in findings {
        let (s, e) = (snap(f.start), snap(f.end));
        if s > cursor {
            if let Some(t) = original.get(cursor..s) {
                out.push(Seg {
                    hit: false,
                    text: t.to_owned(),
                    ty: String::new(),
                });
            }
        }
        if let Some(t) = original.get(s..e) {
            out.push(Seg {
                hit: true,
                text: t.to_owned(),
                ty: f.entity_type.clone(),
            });
        }
        cursor = e.max(cursor);
    }
    if let Some(t) = original.get(cursor..) {
        if !t.is_empty() {
            out.push(Seg {
                hit: false,
                text: t.to_owned(),
                ty: String::new(),
            });
        }
    }
    out
}

/// JSON-encode a string so it can be embedded safely in a `<script>`/eval body.
fn js(s: &str) -> String {
    serde_json::to_string(s).unwrap_or_else(|_| "\"\"".to_owned())
}

#[component]
fn App() -> Element {
    let mut input = use_signal(String::new);
    let mut regex_on = use_signal(|| true);
    let mut aggressive = use_signal(|| false);
    let mut terms = use_signal(String::new);
    let mut show_hits = use_signal(|| false);

    // Recomputed whenever any input above changes.
    let result = use_memo(move || redact(&input(), regex_on(), aggressive(), &terms()));

    let findings = use_memo(move || result().findings);
    let redacted = use_memo(move || result().redacted_text);

    // Per-entity-type counts for the summary chips.
    let counts = use_memo(move || {
        let mut m: std::collections::BTreeMap<String, usize> = std::collections::BTreeMap::new();
        for f in &findings() {
            *m.entry(f.entity_type.clone()).or_default() += 1;
        }
        m
    });

    let n_found = findings().len();
    let has_text = !input().is_empty();

    rsx! {
        document::Title { "Redactor — 100% local PII & secret redaction" }
        document::Meta { name: "viewport", content: "width=device-width, initial-scale=1" }
        document::Meta {
            name: "description",
            content: "Redact PII and secrets locally in your browser. Nothing is uploaded — detection runs on your device via the sovereign pii-core engine compiled to WebAssembly.",
        }
        style { dangerous_inner_html: styles::CSS }

        div { class: "wrap",
            header { class: "top",
                div { class: "logo", "Redactor",
                    span { class: "dot", "." }
                }
                div { class: "badge",
                    span { class: "pulse" }
                    "100% local · nothing leaves this device"
                }
            }
            p { class: "sub",
                "Paste anything — notes, logs, a transcript — and strip out names, emails, phone numbers, card / TFN / Medicare numbers, IP addresses, API keys and private keys. Detection runs entirely "
                b { "on your device" }
                "; there is no upload, no account, and no telemetry."
            }

            // ---- controls ------------------------------------------------
            div { class: "controls",
                Toggle {
                    on: regex_on(),
                    label: "Secrets & IDs",
                    onclick: move |_| regex_on.set(!regex_on()),
                }
                Toggle {
                    on: aggressive(),
                    label: "Aggressive (catch unknown)",
                    onclick: move |_| aggressive.set(!aggressive()),
                }
                div { class: "spacer" }
                button {
                    class: "btn",
                    onclick: move |_| input.set(EXAMPLE.to_owned()),
                    "Load example"
                }
                button {
                    class: "btn",
                    disabled: !has_text,
                    onclick: move |_| {
                        input.set(String::new());
                        terms.set(String::new());
                    },
                    "Clear"
                }
            }

            // ---- input / output -----------------------------------------
            div { class: "grid",
                section { class: "panel",
                    div { class: "head",
                        span { class: "t", "YOUR TEXT" }
                        span { class: "meta", "{input().chars().count()} chars" }
                    }
                    textarea {
                        class: "input",
                        placeholder: "Paste or type text to redact…",
                        value: "{input}",
                        spellcheck: false,
                        autocomplete: "off",
                        oninput: move |e| input.set(e.value()),
                    }
                }

                section { class: "panel",
                    div { class: "head",
                        div { class: "tabs",
                            button {
                                class: if show_hits() { "" } else { "on" },
                                onclick: move |_| show_hits.set(false),
                                "Redacted"
                            }
                            button {
                                class: if show_hits() { "on" } else { "" },
                                onclick: move |_| show_hits.set(true),
                                "Highlighted"
                            }
                        }
                        span { class: "meta",
                            if n_found == 1 { "1 finding" } else { "{n_found} findings" }
                        }
                    }
                    if show_hits() {
                        pre { class: "out",
                            if has_text {
                                for seg in segments(&input(), &findings()) {
                                    if seg.hit {
                                        mark { class: "hit", title: "{seg.ty}", "{seg.text}" }
                                    } else {
                                        "{seg.text}"
                                    }
                                }
                            } else {
                                span { class: "empty", "Nothing to show yet." }
                            }
                        }
                    } else {
                        pre { class: "out",
                            if has_text {
                                "{redacted()}"
                            } else {
                                span { class: "empty", "The redacted copy appears here." }
                            }
                        }
                    }
                }
            }

            // ---- findings summary ---------------------------------------
            if n_found > 0 {
                div { class: "findings",
                    for (ty , n) in counts() {
                        span { class: "chip", key: "{ty}",
                            "{ty}"
                            span { class: "n", "{n}" }
                        }
                    }
                }
            } else if has_text {
                p { class: "empty", "No PII or secrets detected with the current settings." }
            }

            // ---- actions ------------------------------------------------
            div { class: "controls", style: "margin-top:14px",
                button {
                    class: "btn primary",
                    disabled: n_found == 0,
                    onclick: move |_| {
                        let script = format!(
                            "navigator.clipboard && navigator.clipboard.writeText({});",
                            js(&redacted()),
                        );
                        spawn(async move {
                            let _ = document::eval(&script).await;
                        });
                    },
                    "Copy redacted"
                }
                button {
                    class: "btn",
                    disabled: !has_text,
                    onclick: move |_| {
                        let script = format!(
                            "(function(){{const b=new Blob([{}],{{type:'text/plain;charset=utf-8'}});const u=URL.createObjectURL(b);const a=document.createElement('a');a.href=u;a.download='redacted.txt';document.body.appendChild(a);a.click();a.remove();URL.revokeObjectURL(u);}})();",
                            js(&redacted()),
                        );
                        spawn(async move {
                            let _ = document::eval(&script).await;
                        });
                    },
                    "Download .txt"
                }
            }

            // ---- advanced: custom denylist ------------------------------
            details { class: "advanced",
                summary { "Advanced — redact specific names / handles" }
                div { class: "body",
                    label {
                        "One term per line (case-insensitive). These are matched literally and replaced with "
                        code { "[CUSTOM]" }
                        " — useful for project names, usernames, or people the detectors don't know."
                    }
                    textarea {
                        class: "terms",
                        placeholder: "Acme Corp\njdoe\nProject Platypus",
                        value: "{terms}",
                        spellcheck: false,
                        oninput: move |e| terms.set(e.value()),
                    }
                }
            }

            footer { class: "bot",
                span {
                    "Powered by "
                    a { href: "https://github.com/sanguinehost/pii-svc", "pii-core" }
                    " — the sovereign Rust detection engine, compiled to WebAssembly."
                }
                span {
                    "Need a hosted API? "
                    code { "POST /redact" }
                    " speaks the same contract."
                }
            }
        }
    }
}

/// A pill toggle (chip with a checkbox-style box). Clicking flips the bound state.
#[component]
fn Toggle(on: bool, label: String, onclick: EventHandler<MouseEvent>) -> Element {
    rsx! {
        div {
            class: if on { "toggle on" } else { "toggle" },
            onclick: move |e| onclick.call(e),
            span { class: "box" }
            "{label}"
        }
    }
}
