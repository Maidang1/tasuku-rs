# tasuku-rs

A lightweight task management and terminal rendering library inspired by [tasuku](https://github.com/privatenumber/tasuku), built with Rust and Tokio. Supports task state updates, hierarchical tasks, progressive rendering in TTY-enabled terminals, and plain text output in CI environments.

## Features

- **Task States**: `Pending`, `Loading`, `Success`, `Warning`, `Error`
- **Terminal Rendering**: Automatic detection of color support and CI environments
- **Hierarchical Structure**: Tasks can contain subtasks and status output
- **Easy-to-use API**: Define task workflows through async functions

## Installation

Add this to your `Cargo.toml`:

```toml
[dependencies]
tasuku-rs = "0.1.0"
```

See [CHANGELOG.md](CHANGELOG.md) for release notes and version history.

## Quick Start

```rust
use tasuku::{Tasuku, TaskState};

#[tokio::main]
async fn main() {
    let t = Tasuku::new();

    let task = t.task("Example Task", |api| async move {
        api.set_status(Some("Processing".to_string()));
        tokio::time::sleep(std::time::Duration::from_millis(500)).await;
        api.set_output("Output content");
    }).await;

    assert_eq!(task.state(), TaskState::Success);
}
```

## Building from Source

```bash
# Clone the repository
git clone https://github.com/Maidang1/tasuku-rs.git
cd tasuku-rs

# Build the project
cargo build

# Run examples
cargo run --example demo
```

## Dependencies

- `tokio` - Async runtime
- `crossterm` - Terminal manipulation
- `console` - Terminal utilities
- `futures` - Async utilities
- `parking_lot` - Synchronization primitives
- `once_cell` - Lazy static initialization

## Contributing

We use [changesets](https://github.com/changesets/changesets) to manage versioning and changelogs.

### Making Changes

1. **Make your code changes**
2. **Create a changeset** describing your changes:

   ```bash
   ./scripts/add-changeset.sh
   ```

   This will interactively prompt you for the type of change (patch/minor/major) and a description.

3. **Commit both** your code changes and the changeset file
4. **Open a pull request**

### Changeset Types

- **patch** - Bug fixes and minor changes (0.1.0 → 0.1.1)
- **minor** - New features (0.1.0 → 0.2.0)
- **major** - Breaking changes (0.1.0 → 1.0.0)

### Release Process

When changesets are merged to `main`:

1. A GitHub Action automatically processes the changesets
2. A version bump PR is created with updated `Cargo.toml` and `CHANGELOG.md`
3. After merging the version PR, a git tag is created
4. The tag triggers automatic publishing to crates.io

For more details, see [.changeset/README.md](.changeset/README.md).

## Publishing

This project uses GitHub Actions for automatic publishing to crates.io. The publish workflow is triggered when version tags are created. See the [publish workflow](.github/workflows/publish.yml) for more information.

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

## Acknowledgments

Inspired by [tasuku](https://github.com/privatenumber/tasuku) - A minimal task runner for Node.js
