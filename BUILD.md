# Building Cass Code

This guide will help you build Cass Code from source.

## Prerequisites

### Required Tools

1. **Rust Toolchain** (version 1.90.0)
   - Install via [rustup](https://rustup.rs/):
     ```bash
     curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
     ```
   - Set the correct toolchain version:
     ```bash
     cd cass-rs
     rustup override set 1.90.0
     rustup component add clippy rustfmt rust-src
     ```

2. **Node.js** (version 22+)
   - Already installed ✅ (v22.20.0)

3. **pnpm** (version 9.0.0+)
   - Install globally:
     ```bash
     npm install -g pnpm@10.8.1
     ```

### System Requirements

- **macOS**: Xcode Command Line Tools
  ```bash
  xcode-select --install
  ```

- **Linux**: Build essentials
  ```bash
  # Ubuntu/Debian
  sudo apt-get install build-essential pkg-config libssl-dev

  # Fedora
  sudo dnf install gcc openssl-devel
  ```

- **Windows**: MSVC Build Tools
  - Install [Visual Studio Build Tools](https://visualstudio.microsoft.com/downloads/)

## Building the Rust CLI

### 1. Build the entire workspace

```bash
cd cass-rs
cargo build --release
```

This will compile all Rust crates including:
- `cass-cli` - Main CLI binary
- `cass-core` - Core functionality
- `cass-tui` - Terminal UI
- `cass-mcp-server` - MCP server implementation
- And 40+ other crates

### 2. Build only the CLI

```bash
cd cass-rs/cli
cargo build --release
```

The compiled binary will be at: `cass-rs/target/release/cass`

### 3. Run without installing

```bash
cd cass-rs
cargo run --release --bin cass -- --help
```

## Building the Node.js Wrapper

The Node.js CLI wrapper (`cass-cli/`) wraps the Rust binary for npm distribution.

### 1. Install dependencies

```bash
cd cass-cli
npm install
```

### 2. Place the Rust binary

The wrapper expects the Rust binary at:
```
cass-cli/vendor/<platform-triple>/cass/cass
```

For example, on macOS ARM64:
```bash
mkdir -p cass-cli/vendor/aarch64-apple-darwin/cass
cp cass-rs/target/release/cass cass-cli/vendor/aarch64-apple-darwin/cass/
```

Platform triples:
- macOS ARM64: `aarch64-apple-darwin`
- macOS x64: `x86_64-apple-darwin`
- Linux x64: `x86_64-unknown-linux-musl`
- Linux ARM64: `aarch64-unknown-linux-musl`
- Windows x64: `x86_64-pc-windows-msvc`

### 3. Test the wrapper

```bash
cd cass-cli
node bin/cass.js --help
```

## Running Tests

### Rust tests

```bash
cd cass-rs
cargo test
```

### Run specific crate tests

```bash
cd cass-rs/core
cargo test
```

### Run with verbose output

```bash
cargo test -- --nocapture
```

## Installation

### Install globally (after building)

```bash
# Install the Rust binary
cd cass-rs
cargo install --path cli

# Or install the npm package
cd cass-cli
npm install -g .
```

### Verify installation

```bash
cass --version
cass --help
```

## Development Workflow

### 1. Make changes to Rust code

```bash
cd cass-rs
# Edit files in src/
```

### 2. Format code

```bash
cargo fmt
```

### 3. Check for issues

```bash
cargo clippy
```

### 4. Build and test

```bash
cargo build
cargo test
```

### 5. Commit changes

```bash
git add .
git commit -m "Your commit message"
git push
```

## Common Issues

### Issue: "rustc: command not found"
**Solution**: Install Rust toolchain using rustup (see Prerequisites)

### Issue: "linker 'cc' not found"
**Solution**: Install build tools for your platform (see System Requirements)

### Issue: "failed to compile openssl-sys"
**Solution**:
- macOS: `brew install openssl`
- Linux: `sudo apt-get install libssl-dev`

### Issue: Cargo build fails with "cannot find crate"
**Solution**: Build from workspace root instead of individual crate directories:
```bash
cd cass-rs
cargo build
```

## Building for Multiple Platforms

### Cross-compilation setup

```bash
# Install cross-compilation tool
cargo install cross

# Build for Linux from macOS
cross build --release --target x86_64-unknown-linux-musl

# Build for Windows from macOS
cross build --release --target x86_64-pc-windows-msvc
```

### Using GitHub Actions

The repository includes GitHub Actions workflows for automated builds. Check `.github/workflows/` for CI/CD configurations.

## Performance Optimization

### Release builds with LTO

The workspace is already configured for Link-Time Optimization (LTO):
```toml
[profile.release]
lto = "fat"
strip = "symbols"
codegen-units = 1
```

This produces smaller, faster binaries but increases build time.

### Debug builds (faster compilation)

```bash
cargo build  # Without --release
```

## Additional Resources

- [Rust Documentation](https://doc.rust-lang.org/cargo/)
- [Cass Code Docs](./docs/)
- [Contributing Guide](./docs/contributing.md)
- [GitHub Repository](https://github.com/slibien/cass-code)

## Getting Help

If you encounter issues:
1. Check [FAQ](./docs/faq.md)
2. Search [GitHub Issues](https://github.com/slibien/cass-code/issues)
3. Create a new issue with build logs and system information
