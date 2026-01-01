# Contributing to linewise

## Development Setup

```bash
git clone https://github.com/monokrome/linewise
cd linewise
cargo build
```

## Running Tests

```bash
cargo test
```

## Code Style

- Run `cargo fmt` before committing
- Run `cargo clippy` and address warnings
- Use conventional commits: `feat:`, `fix:`, `chore:`, `refactor:`, `docs:`

## Project Structure

```
src/
├── main.rs        # CLI entry point and commands
├── preset.rs      # Preset system (TOML loading, gloss transforms)
├── interactive.rs # TUI mode (ratatui)
├── analysis.rs    # Byte pattern analysis
├── commands.rs    # Subcommand implementations
├── config.rs      # Legacy config (being migrated to preset.rs)
└── records.rs     # Record parsing utilities
```

## Adding a New Command

1. Add variant to `Command` enum in `main.rs`
2. Add match arm in `main()` function
3. Implement command logic (inline or in `commands.rs`)

## Adding a Built-in Transform

Edit `src/preset.rs` in `GlossConfig::apply_builtin()`:

```rust
fn apply_builtin(&self, transform: &str, record: &str) -> Result<String> {
    match transform {
        "my_transform" => {
            // Your transform logic
            Ok(transformed)
        }
        // ...
    }
}
```

## Releasing

Releases are automated via GitHub Actions when a version tag is pushed.

### Release Process

1. **Update version** in `Cargo.toml`:
   ```toml
   [package]
   version = "X.Y.Z"
   ```

2. **Commit the version bump**:
   ```bash
   git add Cargo.toml
   git commit -m "chore: bump version to X.Y.Z"
   ```

3. **Create and push the tag**:
   ```bash
   git tag vX.Y.Z
   git push origin main --tags
   ```

### What the Release Does

The `release.yml` workflow triggers on `v*` tags and:

1. **Builds binaries** for:
   - Linux x86_64
   - Linux aarch64
   - macOS x86_64
   - macOS aarch64
   - Windows x86_64

2. **Publishes to crates.io**

3. **Creates GitHub Release** with all binaries

### Version Scheme

We use semantic versioning:
- **MAJOR**: Breaking changes to CLI or preset format
- **MINOR**: New features, backward compatible
- **PATCH**: Bug fixes

## Pull Requests

1. Fork the repository
2. Create a feature branch: `git checkout -b feat/my-feature`
3. Make changes with conventional commits
4. Run `cargo fmt && cargo clippy && cargo test`
5. Push and open a PR against `main`

## Preset Development

Presets are TOML files in `~/.config/linewise/presets/`. See README.md for format details.

When contributing presets for specific tools/formats:
- Add them to a dedicated repository or package
- Document the preset's purpose and requirements
- Test with real-world data
