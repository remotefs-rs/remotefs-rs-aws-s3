# AGENTS.md

Guidance for coding agents working in this repository.

## Project context

This repository is a reusable Rust project template. It starts as one Cargo
package with both library and binary targets and no third-party Rust
dependencies.

The toolchain is pinned to Rust 1.98.0 with edition 2024. Keep
`rust-toolchain.toml` and `package.rust-version` in `Cargo.toml` synchronized.

## Working rules

- Read the relevant source, documentation, configuration, and tests before
  making changes.
- Keep changes scoped to the requested task and preserve unrelated user work.
- Use existing project patterns before adding new abstractions.
- Add or update focused tests before changing behavior.
- Update user documentation when commands, public APIs, configuration, or
  workflows change.
- Use `git`; never use `jj` or git worktrees.
- Never open a GitHub issue without prior user approval.
- Do not stage or commit local plans. Planning state belongs under the ignored
  `docs/superpowers/`, `.superpowers/`, or `.claude/plans/` directories.

## Source layout

- `src/lib.rs` contains the public library API and its focused unit tests.
- `src/main.rs` is the binary entry point and may call the library API.
- Keep both targets unless the requested project is explicitly library-only or
  binary-only.
- Use `module_name.rs`; never introduce `mod.rs`.

## Rust conventions

- Follow `rustfmt.toml` and format Rust through `just fmt`.
- Clippy must pass with warnings denied.
- Public library modules and items require canonical rustdoc documentation.
- Avoid `unsafe` unless the task requires it and its safety invariants are
  documented and tested.
- Use named format placeholders instead of positional `{}` arguments.
- Prefer `#[expect]` with a reason over `#[allow]` for local lint overrides.
- Keep dependency entries and feature definitions alphabetically sorted.
- Use bare, minimal dependency versions in `Cargo.toml`.

## Command interface

Use [`just`](https://just.systems) recipes when one exists. Do not bypass a
recipe with an ad hoc `cargo` or tool command. If a recurring task has no
recipe, add a focused recipe before using it.

Run `just` to list every command. The primary recipes are:

```sh
just build
just release
just test
just coverage
just fmt
just fmt_check
just lint "-- -D warnings"
just doc
just deny
just scan_secrets
just check
just setup_githooks
```

`just check` is the required local quality gate. It runs formatting checks,
Clippy with warnings denied, rustdoc with warnings denied, cargo-deny, and the
test suite.

When invoking compilation or test commands from the CLI, never request
parallelism greater than eight. This is an invocation constraint; do not encode
the local cap in tracked project files.

## Required tools

The local recipes expect:

- Rust and rustup.
- just.
- dprint 0.56.1 and nightly rustfmt.
- cargo-deny.
- TruffleHog.
- git-cliff.
- cargo-llvm-cov for coverage.
- zizmor for workflow changes.
- shellcheck for shell hook changes.

If a required tool is unavailable, report it explicitly. Do not claim its check
passed or silently replace the repository command with a weaker check.

## Documentation

- Write task-oriented documentation with runnable examples.
- Use ATX headings and fenced code blocks with language identifiers.
- Keep Markdown lines readable, tables aligned, and files terminated by one
  newline.
- Run `fmt-md-tables -i <file>` after editing a Markdown file that contains a
  table.

## GitHub Actions

- Run `zizmor .github/workflows` after every workflow change until it exits
  successfully with no findings.
- Pin every external action to the full commit SHA of its latest stable release
  and record the exact matching tag in a trailing comment.
- Declare explicit least-privilege permissions.
- Set `persist-credentials: false` on checkout steps unless later authenticated
  Git operations are explicitly required.
- Never interpolate attacker-controlled GitHub expressions directly into a
  shell script. Pass values through `env` and quote the shell variable.

## Git and releases

- Use Conventional Commits with an imperative, lower-case description.
- Do not add agent attribution, session links, or agent `Co-Authored-By` lines.
- Inspect the diff before staging or committing changes.
- Generate release notes with `just changelog_preview <version>` and
  `just changelog <version>`.
- Verify packages locally with `just publish "--dry-run --allow-dirty"`.
- Live publication uses crates.io trusted publishing through
  `.github/workflows/publish.yml`.
