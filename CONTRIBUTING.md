# Contributing to PredictIQ

Thank you for your interest in contributing! This guide covers everything you need to get started.

## Table of Contents

- [Development Setup](#development-setup)
- [Branch Naming](#branch-naming)
- [Commit Conventions](#commit-conventions)
- [Pull Request Process](#pull-request-process)
- [Running Tests](#running-tests)
- [Code Style](#code-style)
- [Frontend Styling (CSP-Safe)](#frontend-styling-csp-safe)
- [Security](#security)

---

## Development Setup

### Prerequisites

- [Rust](https://rustup.rs/) (stable toolchain)
- [Node.js](https://nodejs.org/) 18+
- [Docker](https://docs.docker.com/get-docker/) and Docker Compose
- [PostgreSQL](https://www.postgresql.org/) 15+ (or use the provided Docker Compose stack)

### Getting Started

```bash
# Clone the repository
git clone https://github.com/solutions-plug/predictIQ.git
cd predictIQ

# Start backing services (Postgres, Redis, etc.)
docker compose up -d

# API service
cd services/api
cp .env.example .env          # fill in required values
cargo build

# Frontend
cd frontend
cp .env.example .env.local    # fill in required values
npm install
npm run dev

# TTS service
cd services/tts
npm install
npm run dev
```

---

## Branch Naming

Use the following prefixes:

| Prefix | Purpose |
|--------|---------|
| `feat/` | New feature |
| `fix/` | Bug fix |
| `chore/` | Maintenance, dependency updates |
| `docs/` | Documentation only |
| `refactor/` | Code refactoring without behaviour change |
| `perf/` | Performance improvement |
| `ci/` | CI/CD changes |

Examples: `feat/market-resolution`, `fix/rate-limit-header`, `docs/contributing`

---

## Commit Conventions

This project uses **[Conventional Commits](https://www.conventionalcommits.org/)**.  
The CHANGELOG is **auto-generated** from commit messages via [git-cliff](https://git-cliff.org/) — do not edit `CHANGELOG.md` manually.

### Format

```
<type>(<scope>): <short description>

[optional body]

[optional footer(s)]
```

### Types

| Type | When to use |
|------|-------------|
| `feat` | A new feature (triggers a minor version bump) |
| `fix` | A bug fix (triggers a patch version bump) |
| `docs` | Documentation changes only |
| `chore` | Build process, dependency updates, tooling |
| `refactor` | Code change that neither fixes a bug nor adds a feature |
| `perf` | Performance improvement |
| `test` | Adding or updating tests |
| `ci` | CI/CD configuration changes |

Append `!` after the type/scope for **breaking changes** (triggers a major version bump):

```
feat(api)!: remove deprecated /v0 endpoints
```

### Examples

```
feat(markets): add oracle result caching
fix(newsletter): handle duplicate subscription gracefully
docs(api): document rate-limit response headers
chore(deps): bump axum to 0.7.5
```

---

## Pull Request Process

1. Fork the repository and create your branch from `main`.
2. Ensure all tests pass locally (see [Running Tests](#running-tests)).
3. Keep commits focused — one logical change per commit.
4. Open a PR against `main` with a clear title following the commit convention.
5. Fill in the PR description:
   - **What** changed and **why**
   - How to test the change
   - Any breaking changes or migration steps
6. Link related issues using `Closes #<issue>` in the PR description.
7. At least one approval is required before merging.
8. Squash-merge is preferred to keep the history clean.

### PR Checklist

- [ ] Branch is up to date with `main`
- [ ] Commit messages follow Conventional Commits
- [ ] Tests added or updated for the change
- [ ] Documentation updated if behaviour changed
- [ ] No secrets or credentials committed
- [ ] `CHANGELOG.md` **not** manually edited

---

## Running Tests

### API (Rust)

```bash
cd services/api
cargo test
```

### Integration Tests (API — requires backing services)

Integration tests require PostgreSQL, Redis, and a Stellar RPC node.
The easiest way to start them is via the provided Makefile target, which
starts a Docker Compose stack, runs the tests, and tears the stack down:

```bash
make test-integration
```

You can also manage the services manually:

```bash
# Start services
docker compose -f docker-compose.test.yml up -d --wait

# Run tests
cd services/api
TEST_DATABASE_URL=postgres://predictiq_test:predictiq_test@localhost:5433/predictiq_test \
TEST_REDIS_URL=redis://localhost:6380 \
STELLAR_RPC_URL=http://localhost:8080 \
cargo test --test '*' -- --test-threads=1

# Tear down (always run, even on failure)
docker compose -f docker-compose.test.yml down -v
```

If a previous run left the stack running, clean it up first:

```bash
make test-integration-down
```

#### Database fixture — transaction rollback

Each integration test that touches the database should use the
`with_test_transaction` helper from `tests/common/db_fixture.rs`.
It wraps the test body in a database transaction that is **rolled back**
at the end, so no test leaves rows that can affect subsequent tests:

```rust
use common::db_fixture::with_test_transaction;

#[tokio::test]
async fn my_test() {
    let pool = common::db_fixture::test_pool().await;
    with_test_transaction(&pool, |mut conn| async move {
        // use conn for all DB operations in this test
        sqlx::query("INSERT INTO ...").execute(&mut *conn).await.unwrap();
        // transaction is automatically rolled back when this closure returns
    }).await;
}
```

Do **not** commit within the closure — the rollback guarantees a clean slate
for the next test regardless of execution order.

### Frontend (Next.js)

```bash
cd frontend
npm test              # unit tests (Jest)
npm run test:e2e      # end-to-end tests (Playwright)
```

### Visual Regression Tests

Visual regression tests use Playwright snapshots to detect unintended UI changes. Baseline screenshots are stored in Git LFS to keep the repository size manageable.

#### Baseline Screenshot Storage

Baseline screenshot files are configured in `.gitattributes` to be tracked by Git LFS:

```
frontend/e2e/**/__snapshots__/*.png filter=lfs diff=lfs merge=lfs -text
```

Before running visual regression tests locally, ensure Git LFS is installed:

```bash
# Install Git LFS (macOS)
brew install git-lfs
git lfs install

# Or on Linux
sudo apt-get install git-lfs
git lfs install
```

#### Updating Baselines Locally

When UI changes are intentional and tests fail due to new screenshots, update baselines:

```bash
cd frontend
npx playwright test --update-snapshots
```

This command captures new baseline screenshots. Commit the updated baselines via Git LFS:

```bash
git add frontend/e2e/**/__snapshots__/
git commit -m "test: update visual regression baselines"
```

#### Visual Diff Threshold

The visual regression tests use a configurable diff threshold (default: 0.1%) to prevent flakiness from minor pixel differences. Configure the threshold via the `VISUAL_DIFF_THRESHOLD` environment variable:

```bash
# Run with custom threshold (e.g., 0.2%)
VISUAL_DIFF_THRESHOLD=0.2 npm run test:e2e
```

In CI, the threshold is enforced automatically. Tests fail if the pixel diff exceeds the configured threshold.

### TTS Service

```bash
cd services/tts
npm test
```

### Smart Contracts

```bash
cd contracts/predict-iq
make test
```

---

### Property-Based Tests

Validation logic in `src/validation.rs` is covered by property-based tests
using [`proptest`](https://github.com/proptest-rs/proptest).  These run as
part of `cargo test` and are gated in CI with at least **1 000 cases per
property** via `PROPTEST_CASES=1000`.

To run them locally with the same case count:

```bash
cd services/api
PROPTEST_CASES=1000 cargo test prop_
```

When adding a new validation function, add a corresponding `proptest!` block
that at minimum covers:

- Zero-length input
- Input longer than `MAX_LEN + 1`
- All-whitespace strings
- Strings containing null bytes (`\0`)
- Strings that must pass unchanged (valid inputs)

## Minimum Supported Rust Version (MSRV)

The `services/api` crate declares a `rust-version` field in its `Cargo.toml`.
This is the **oldest** Rust

---

## Frontend Styling (CSP-Safe)

### The rule: no inline `style` props

Do **not** use the `style={{ ... }}` prop on React/Next.js components for
anything that affects rendering. Use CSS classes (or CSS modules) instead.

```tsx
// ❌ Don't — CSP strips this at runtime, so the style silently disappears
<div style={{ padding: 16, color: 'red' }}>…</div>

// ✅ Do — put the rules in a stylesheet and reference them by class
<div className="panel panel--error">…</div>
```

### Why (this is not a style preference)

The frontend is served under a strict **Content-Security-Policy** that does not
allow inline styles. When a component sets `style={{ ... }}`, the browser
**blocks or strips** the resulting inline style attribute — the element renders
with no styling at all. This is a real production bug that was fixed in
`5bd5e51` (AppShell chrome) and `e80a15b` (all remaining inline `style` props),
and it is easy to reintroduce by accident because the code still *looks*
correct in the editor and in tests that don't enforce CSP.

Because the failure is silent (no error, just missing styles), the convention
below is mandatory rather than optional.

### Required pattern

- Define styles in CSS (global stylesheet or a CSS module) and apply them via
  `className`.
- For dynamic values, prefer a class variant (e.g. `panel--error`) or a CSS
  custom property set through a class, rather than a `style` prop.
- If a value genuinely cannot be expressed as a class, raise it in the PR
  description so the CSP implications can be reviewed — do not add an inline
  `style` prop silently.

### Enforcement

There is currently **no lint rule or CI check** that blocks inline `style`
props, so this convention relies on review. Adding an ESLint rule such as
[`react/forbid-dom-props`](https://github.com/jsx-eslint/eslint-plugin-react/blob/master/docs/rules/forbid-dom-props.md)
configured to forbid `style` is recommended and tracked separately; until it
lands, reviewers should flag any new `style={{ ... }}` prop.

---

## Security

If you discover a security vulnerability, please **do not** open a public issue.
Instead, report it privately to the maintainers so it can be triaged and fixed
before disclosure.
