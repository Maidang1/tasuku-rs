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

## Publishing

This project uses GitHub Actions for automatic publishing to crates.io. See the [setup guide](/.github/workflows/publish.yml) for more information.

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

## Acknowledgments

Inspired by [tasuku](https://github.com/privatenumber/tasuku) - A minimal task runner for Node.js
