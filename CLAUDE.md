# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with
code in this repository.

`AGENTS.md` holds the full agent contract for this repository. Read it before
making changes; this file summarizes the parts needed most often.

## Commands

Every task runs through a [`just`](https://just.systems) recipe. Do not bypass a
recipe with an ad hoc `cargo` or tool command. If a recurring task has no
recipe, add one under `just/` before using it. Run `just` to list all recipes.

```sh
just build                 # cargo build --all-targets
just release               # release build
just test                  # cargo test --all-targets, then --doc
just coverage              # cargo llvm-cov, writes lcov.info
just fmt                   # dprint fmt (Markdown, Rust, TOML, YAML)
just fmt_check             # dprint check
just lint "-- -D warnings" # alias of `just clippy`
just doc                   # cargo doc with RUSTDOCFLAGS="-D warnings"
just deny                  # cargo deny check
just scan_secrets          # trufflehog filesystem
just check                 # the full local quality gate
just setup_githooks        # point core.hooksPath at .githooks
just changelog_preview 0.1.0
just changelog 0.1.0
just publish "--dry-run --allow-dirty"
```

`just check` is the required gate before declaring work done. It chains
`fmt_check`, Clippy with warnings denied, `doc`, `deny`, and `test`.

To run one test, pass the filter through the recipe's `args`:

```sh
just test "greeting_identifies_template"
just test "-- --nocapture"
```

Never request build or test parallelism above eight from the CLI. That is an
invocation constraint only; do not write the cap into tracked files.

If a required tool is missing, say so. Never claim a check passed or silently
swap in a weaker command.

## Architecture

This is a template repository, not an application. The Rust source is a
deliberate two-line placeholder; the substance lives in the tooling layers,
which are designed to stay identical across every project generated from it.

- **Package shape.** One Cargo package with both a `[lib]` (`rust_template`,
  `src/lib.rs`) and a `[[bin]]` (`rust-template`, `src/main.rs`). The binary
  calls the library. Downstream projects delete one target; keep both here.
- **Command layer.** `Justfile` is a thin importer. Each recipe group lives in
  its own file under `just/` (`build`, `test`, `code_check`, `changelog`,
  `publish`) and carries a `[group(...)]` attribute so `just --list` stays
  organized. Recipes take an `args=""` passthrough rather than hard-coding
  flags.
- **Three gates run the same recipes.** `just check` locally, `.githooks/`
  `pre-commit` on commit, and `.github/workflows/ci.yml` in CI all shell out to
  the same `just` targets. Changing a recipe changes all three at once, so a
  recipe edit is never local-only.
- **Lint policy lives in `Cargo.toml`.** The `[lints.rust]` and
  `[lints.clippy]` tables set the warning surface (Clippy `pedantic`, `cargo`,
  `perf`, and a long list of individual restriction lints). Clippy is then run
  with `-D warnings`, so the tables are the real policy and the flag only
  escalates it.
- **Formatting is dprint, not cargo fmt.** `dprint.json` owns Markdown, TOML,
  and YAML, and delegates `.rs` files to nightly rustfmt through its exec
  plugin. `rustfmt.toml` uses nightly-only options
  (`imports_granularity`, `group_imports`), which is why nightly is required.
  Always format with `just fmt`.
- **Release path.** Commits follow Conventional Commits, `cliff.toml` turns
  them into `CHANGELOG.md`, and `.github/workflows/publish.yml` is
  `workflow_dispatch`-only. It validates that the branch is default, the
  `Cargo.toml` version matches the input, `CHANGELOG.md` has a non-empty
  section for it, and no `v<version>` tag exists. Publishing uses crates.io
  trusted publishing (OIDC, no stored token), and the tag is created only after
  crates.io accepts the package.
- **Supply-chain policy.** `deny.toml` is strict: license allowlist,
  `yanked = "deny"`, `unmaintained = "all"`, wildcard versions denied, and
  crates.io as the only allowed source. There are currently no third-party
  dependencies; adding one must satisfy this policy.

## Conventions

- Toolchain is pinned to Rust 1.98.0, edition 2024. Keep
  `rust-toolchain.toml` and `package.rust-version` in sync.
- Use `module_name.rs`; never `mod.rs`.
- Public library items need canonical rustdoc, including a runnable example.
  `just test` runs doctests, and `just doc` denies warnings.
- Named format placeholders, not positional `{}`.
- Prefer `#[expect]` with a reason over `#[allow]`.
- Keep `Cargo.toml` dependency and feature entries alphabetically sorted, with
  bare minimal versions.
- Conventional Commits, imperative and lower-case. No agent attribution,
  session links, or agent `Co-Authored-By` lines.
- Do not stage planning state. `docs/superpowers/`, `.superpowers/`, and
  `.claude/plans/` are gitignored and dprint-excluded.
- After editing a Markdown file that contains a table, run
  `fmt-md-tables -i <file>`.
- After any change under `.github/workflows/`, run `zizmor .github/workflows`
  until it exits clean. Pin actions to a full commit SHA with the matching tag
  in a trailing comment, declare least-privilege permissions, and set
  `persist-credentials: false` on checkout.
