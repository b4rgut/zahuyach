# CI/CD Setup Instructions

## What this workflow does

The `release.yml` workflow automatically:
1. Formats code using `cargo fmt`
2. Runs all tests
3. Determines version bump based on commit messages
4. Updates version in `Cargo.toml`
5. Creates a git tag
6. Publishes to crates.io

## Setup

### 1. Add CARGO_REGISTRY_TOKEN secret

1. Go to https://crates.io/settings/tokens
2. Create a new API token with `publish` scope
3. In your GitHub repository, go to Settings → Secrets and variables → Actions
4. Add a new secret named `CARGO_REGISTRY_TOKEN` with your token

### 2. Enable GitHub Actions

The workflow will run automatically when you push to the `main` branch.

## How versioning works

Version bumps are determined by commit message prefixes:
- `feat!:` or any commit with `!` → Major version bump (1.0.0 → 2.0.0)
- `feat:` → Minor version bump (1.0.0 → 1.1.0)
- `fix:` → Patch version bump (1.0.0 → 1.0.1)
- Other commits → No version bump

## Commit message format

Follow the conventional commits format:
```
<type>(<scope>): <description>
```

Example:
- `feat(cli): add new command`
- `fix(parser): handle empty files`
- `feat!: breaking change in API`
