# Installation

## macOS (Homebrew)

```bash
brew tap gustafeden/tap
brew install blaeckfetch
```

## Arch Linux

```bash
yay -S blaeckfetch-bin
```

## Linux/macOS (Install Script)

```bash
curl -fsSL https://gustafeden.github.io/blaeckfetch/install.sh | bash
```

The installer script:
- Supports Linux (x86_64, aarch64) and macOS (Intel, Apple Silicon)
- Detects your platform automatically
- Installs to `~/.local/bin/blaeckfetch`
- You can specify a version with `BLAECKFETCH_VERSION` environment variable:

```bash
BLAECKFETCH_VERSION=0.2.0 curl -fsSL https://gustafeden.github.io/blaeckfetch/install.sh | bash
```

## From Source

Clone the repository and build with Cargo:

```bash
git clone https://github.com/gustafeden/blaeckfetch
cd blaeckfetch
cargo build --release
```

The binary will be available at `target/release/blaeckfetch`.

Alternatively, install directly from Git:

```bash
cargo install --git https://github.com/gustafeden/blaeckfetch
```
