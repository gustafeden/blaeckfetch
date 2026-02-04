# Performance

## Comparison

| Tool | Time |
|------|------|
| **blaeckfetch** | **~7ms** |
| neofetch | ~400ms |

~57x faster than neofetch. Uses a boot-cycle cache for static fields — first run after reboot is ~40ms, every run after that is ~7ms.

## How It's Fast

- **Compiled Rust binary** — no interpreter startup
- **Direct syscalls** for host model, disk stats, display resolution (no subprocess spawning)
- **Boot-cycle cache** (`~/.cache/blaeckfetch/cache`) for fields that don't change between reboots
- **Only two subprocess calls** on macOS (`defaults read` for theme), and those are cached
- **sysinfo crate** for memory/CPU instead of parsing command output

## Benchmarking

You can benchmark blaeckfetch on your system:

```bash
# Using hyperfine (recommended)
hyperfine 'blaeckfetch' 'neofetch'

# Using time
time blaeckfetch
```
