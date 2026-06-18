//! The app's stylesheet, injected once as a trusted constant (`dangerous_inner_html`
//! so the `>` child-combinators etc. aren't HTML-escaped). One file, no build-time
//! CSS pipeline — keeps the offline/wasm build simple and works on every platform.

pub const CSS: &str = r#"
:root{
  --bg:#0b1020; --panel:#121a2f; --panel-2:#0e1526; --ink:#e7ecf6; --muted:#9aa6c0;
  --line:#23304f; --accent:#5b8cff; --accent-2:#36d399; --danger:#ff6b6b;
  --mark:#3a2a12; --mark-ink:#ffd591; --radius:14px;
}
*{box-sizing:border-box}
html,body{margin:0;padding:0}
body{
  background:radial-gradient(1200px 600px at 70% -10%, #16213f 0%, var(--bg) 55%);
  color:var(--ink); font:15px/1.55 ui-sans-serif,system-ui,-apple-system,Segoe UI,Roboto,Inter,sans-serif;
  min-height:100vh;
}
a{color:var(--accent);text-decoration:none}
a:hover{text-decoration:underline}
.wrap{max-width:1100px;margin:0 auto;padding:28px 20px 56px}
header.top{display:flex;flex-wrap:wrap;align-items:center;gap:12px;margin-bottom:6px}
.logo{font-size:24px;font-weight:800;letter-spacing:-.02em}
.logo .dot{color:var(--accent-2)}
.badge{display:inline-flex;align-items:center;gap:7px;font-size:12.5px;font-weight:600;
  color:var(--accent-2);background:rgba(54,211,153,.10);border:1px solid rgba(54,211,153,.35);
  padding:5px 11px;border-radius:999px}
.badge .pulse{width:8px;height:8px;border-radius:50%;background:var(--accent-2);box-shadow:0 0 0 0 rgba(54,211,153,.6);animation:pulse 2s infinite}
@keyframes pulse{0%{box-shadow:0 0 0 0 rgba(54,211,153,.5)}70%{box-shadow:0 0 0 7px rgba(54,211,153,0)}100%{box-shadow:0 0 0 0 rgba(54,211,153,0)}}
.sub{color:var(--muted);margin:2px 0 20px;max-width:64ch}
.controls{display:flex;flex-wrap:wrap;gap:10px;align-items:center;margin-bottom:16px}
.toggle{cursor:pointer;user-select:none;border:1px solid var(--line);background:var(--panel-2);
  color:var(--muted);padding:8px 13px;border-radius:999px;font-weight:600;font-size:13.5px;
  display:inline-flex;align-items:center;gap:8px;transition:.15s}
.toggle:hover{border-color:var(--accent)}
.toggle .box{width:15px;height:15px;border-radius:5px;border:1.5px solid var(--muted);display:inline-block}
.toggle.on{color:var(--ink);border-color:var(--accent);background:rgba(91,140,255,.12)}
.toggle.on .box{background:var(--accent);border-color:var(--accent)}
.spacer{flex:1}
.btn{cursor:pointer;border:1px solid var(--line);background:var(--panel);color:var(--ink);
  padding:8px 14px;border-radius:10px;font-weight:600;font-size:13.5px;transition:.15s}
.btn:hover{border-color:var(--accent);background:var(--panel-2)}
.btn.primary{background:var(--accent);border-color:var(--accent);color:#06122e}
.btn.primary:hover{filter:brightness(1.07)}
.btn:disabled{opacity:.45;cursor:not-allowed}
.grid{display:grid;grid-template-columns:1fr 1fr;gap:16px}
@media(max-width:860px){.grid{grid-template-columns:1fr}}
.panel{background:var(--panel);border:1px solid var(--line);border-radius:var(--radius);overflow:hidden;display:flex;flex-direction:column;min-height:340px}
.panel .head{display:flex;align-items:center;justify-content:space-between;gap:8px;
  padding:11px 14px;border-bottom:1px solid var(--line);background:var(--panel-2)}
.panel .head .t{font-weight:700;font-size:13.5px;letter-spacing:.02em}
.panel .head .meta{color:var(--muted);font-size:12px}
textarea.input{flex:1;width:100%;resize:vertical;min-height:300px;border:0;outline:0;
  background:transparent;color:var(--ink);padding:14px;font:14px/1.6 ui-monospace,SFMono-Regular,Menlo,Consolas,monospace}
.out{flex:1;margin:0;padding:14px;white-space:pre-wrap;word-break:break-word;
  font:14px/1.6 ui-monospace,SFMono-Regular,Menlo,Consolas,monospace;overflow:auto}
.out .tok{background:var(--accent);color:#06122e;border-radius:5px;padding:0 5px;font-weight:700}
mark.hit{background:var(--mark);color:var(--mark-ink);border-radius:4px;padding:0 3px;
  border:1px solid rgba(255,213,145,.35);cursor:help}
.tabs{display:inline-flex;gap:4px;background:var(--bg);border:1px solid var(--line);border-radius:9px;padding:3px}
.tabs button{border:0;background:transparent;color:var(--muted);font-weight:600;font-size:12.5px;
  padding:4px 10px;border-radius:6px;cursor:pointer}
.tabs button.on{background:var(--accent);color:#06122e}
.findings{display:flex;flex-wrap:wrap;gap:8px;margin:16px 0 4px}
.chip{display:inline-flex;align-items:center;gap:7px;background:var(--panel);border:1px solid var(--line);
  border-radius:999px;padding:6px 12px;font-size:12.5px;font-weight:600}
.chip .n{background:var(--accent);color:#06122e;border-radius:999px;padding:0 8px;font-size:11.5px}
.empty{color:var(--muted);font-size:13.5px;padding:6px 0}
.advanced{margin-top:16px;border:1px solid var(--line);border-radius:12px;background:var(--panel-2)}
.advanced summary{cursor:pointer;padding:12px 14px;font-weight:700;font-size:13.5px;color:var(--muted)}
.advanced[open] summary{color:var(--ink);border-bottom:1px solid var(--line)}
.advanced .body{padding:14px}
.advanced label{display:block;color:var(--muted);font-size:12.5px;margin-bottom:7px}
textarea.terms{width:100%;min-height:84px;resize:vertical;background:var(--bg);color:var(--ink);
  border:1px solid var(--line);border-radius:9px;padding:10px;
  font:13px/1.5 ui-monospace,SFMono-Regular,Menlo,Consolas,monospace}
footer.bot{margin-top:28px;color:var(--muted);font-size:12.5px;border-top:1px solid var(--line);
  padding-top:16px;display:flex;flex-wrap:wrap;gap:8px 18px;align-items:center}
footer.bot code{background:var(--panel);border:1px solid var(--line);border-radius:6px;padding:2px 6px;color:var(--ink)}
"#;
