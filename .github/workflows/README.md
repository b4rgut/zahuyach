# CI/CD Setup Instructions

## What this workflow does

The `release.yml` workflow automatically:
1. Formats code using `cargo fmt`
2. Runs all tests
3. Checks if package is already published on crates.io
4. Determines version bump based on commit messages
5. Updates version in `Cargo.toml`
6. Creates a git tag
7. Publishes to crates.io

For first-time publishing or manual control, use the `manual-release.yml` workflow.

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
- Other commits → Patch version bump (if package was already published)
- First release → Uses current version from Cargo.toml

## Commit message format

Follow the conventional commits format:
```
<type>(<scope>): <description>
```

Example:
- `feat(cli): add new command`
- `fix(parser): handle empty files`
- `feat!: breaking change in API`

## Manual Release

If automatic release doesn't work or you need more control:

1. Go to Actions → Manual Release
2. Click "Run workflow"
3. Select version bump type or "none" to use current version
4. Optionally check "Force publish" to overwrite existing version

## Troubleshooting

### Package not publishing

1. **First release**: The automatic workflow will detect if your package has never been published and will publish the current version
2. **No version bump**: If no commits match the patterns, the workflow will:
   - Skip publishing if the current version is already on crates.io
   - Apply a patch bump if there are any new commits

### Common issues

- **"Package already exists"**: The version in Cargo.toml is already published. The workflow will auto-bump the version on next push
- **"No token found"**: Make sure CARGO_REGISTRY_TOKEN is set in repository secrets
- **"Invalid manifest"**: Check that Cargo.toml has all required fields for publishing

### Force publishing

Use the manual workflow with "Force publish" option if you need to:
- Republish a yanked version
- Override version detection
- Publish without conventional commits
