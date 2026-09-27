# Contributing

Thanks for considering a contribution. Issues and pull requests are welcome for
bug fixes, new detection-control UI, additional build targets, and
documentation.

## Before opening a pull request

- Search existing issues and pull requests; describe the user-visible problem
  and expected behavior.
- Keep changes focused. This app's one real constraint is **no network call**
  — it exists specifically so redaction happens on-device. A change that adds
  a `fetch`/`reqwest` call (telemetry, update checks, anything) changes the
  actual security property this app is built around; call that out explicitly
  in the PR description rather than adding it quietly.
- For detection-behavior changes, state what input you tested against and
  what changed (the shared engine is `pii-core`, from
  [`Ptyktos/pii-svc`](https://github.com/Ptyktos/pii-svc) — behavior changes
  usually belong there, not here).

## Development setup

```sh
rustup target add wasm32-unknown-unknown
cargo install dioxus-cli@0.7   # provides `dx`

dx serve --web                 # dev server, hot reload
dx build --web --release       # production bundle
```

Desktop/mobile builds (`dx serve --desktop`, `dx build --android/--ios`) need
their own SDKs — see the README's Targets table.

## Pull requests

Open a pull request against `main`. Include a concise summary and the
commands you ran. CI builds the web target; desktop/mobile builds aren't
wired into CI (no SDKs there) so mention if you tested them manually.

## Security reports

Do not report vulnerabilities in public issues or pull requests. Follow
[SECURITY.md](SECURITY.md) for private reporting.

## License

By contributing, you agree that your contribution is offered under the
repository's Apache-2.0 license.
