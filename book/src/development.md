# Development

## Dependencies

- [blaeck](https://github.com/gustafeden/blaeck) — inline terminal UI framework
- [sysinfo](https://crates.io/crates/sysinfo) — cross-platform system information
- [clap](https://crates.io/crates/clap) — CLI argument parsing
- [image](https://crates.io/crates/image) — image loading and processing (splash mode backgrounds)

## Building from Source

```bash
git clone https://github.com/gustafeden/blaeckfetch
cd blaeckfetch
cargo build --release
```

The binary will be at `target/release/blaeckfetch`.

## Releasing

1. Bump `version` in `Cargo.toml` and `installer/Cargo.toml`
2. Commit: `git commit -am "v0.X.0"`
3. Tag and push: `git tag v0.X.0 && git push && git push origin v0.X.0`

GitHub Actions builds binaries for macOS/Linux (Intel + ARM) and creates the release.
