# Contributing

Thanks for your interest in contributing to kaneo-cli!

## Development Setup

```bash
git clone https://github.com/onreza/kaneo-cli.git
cd kaneo-cli
cargo build
```

### Prerequisites

- Rust 1.85+ (stable)
- Node.js 20+ (for commit linting)

### Install git hooks

```bash
npm install
```

This sets up [lefthook](https://github.com/evilmartians/lefthook) for:
- **pre-commit**: `cargo fmt` + `cargo clippy`
- **commit-msg**: conventional commit validation
- **pre-push**: `cargo test`

## Commit Convention

We use [Conventional Commits](https://www.conventionalcommits.org/):

```
feat(cli): add workspace invite command
fix(auth): handle expired tokens gracefully
docs: update README with new commands
```

**Types:** `feat`, `fix`, `perf`, `docs`, `refactor`, `style`, `chore`, `ci`, `test`

**Scopes:** `cli`, `auth`, `api`, `output`, `deps`, `ci`, `release`, `tests`

## Adding a New Command

1. Add the subcommand to the relevant enum in `src/cli/mod.rs`
2. Implement the handler in `src/cli/<resource>_handler.rs`
3. Route it in `src/main.rs` if it's a new top-level command
4. Add the operationId mapping in `src/cli/api_check_handler.rs`
5. Update `README.md`

## Code Style

- Run `cargo fmt` before committing
- Run `cargo clippy -- -D warnings` — no warnings allowed
- Follow the existing patterns for JSON/human dual output
- Use `anyhow` for error propagation, `thiserror` for custom errors
- Keep handlers simple — logic belongs in the API client layer

## Pull Requests

1. Fork and create a feature branch from `main`
2. Make your changes with clear commit messages
3. Ensure CI passes (`cargo test`, `cargo clippy`, `cargo fmt`)
4. Open a PR with a description of what changed and why
