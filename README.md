# rsfetch

A fast system fetch display for your terminal, written in Rust. Powered by [blaeck](https://github.com/gustafeden/blaeck) for rendering.

## Performance

| Tool | Time |
|------|------|
| **rsfetch** | **~7ms** |
| neofetch | ~400ms |

~57x faster than neofetch. Uses a boot-cycle cache for static fields — first run after reboot is ~40ms, every run after that is ~7ms.

## Output

```
                    'c.          gustafeden@hostname
                 ,xNMM.          --------------------------
               .OMMMMo           OS: MacOS 15.5 arm64
               OMMM0,            Host: Mac15,6
     .;loddo:' loolloddol;.      Kernel: 24.5.0
   cKMMMMMMMMMMNWMMMMMMMMMM0:    Uptime: 32 days, 17 hours
 .KMMMMMMMMMMMMMMMMMMMMMMMWd.    Packages: 214 (brew)
 XMMMMMMMMMMMMMMMMMMMMMMMX.      Shell: zsh
;MMMMMMMMMMMMMMMMMMMMMMMM:       Resolution: 1800x1169, 2560x1440
:MMMMMMMMMMMMMMMMMMMMMMMM:       DE: Aqua
.MMMMMMMMMMMMMMMMMMMMMMMMX.      WM: Quartz Compositor
 kMMMMMMMMMMMMMMMMMMMMMMMMWd.    WM Theme: Blue (Dark)
 .XMMMMMMMMMMMMMMMMMMMMMMMMMMk   Terminal: WezTerm
  .XMMMMMMMMMMMMMMMMMMMMMMMMK.   CPU: Apple M3 Pro (11)
    kMMMMMMMMMMMMMMMMMMMMMMd     GPU: Apple M3 Pro
     ;KMMMMMMMWXXWMMMMMMMk.      Memory: 7083MiB / 36864MiB
       .cooc,.    .,coo:.        Disk (/): 753GiB / 926GiB (81%)
                                 Local IP: 192.168.1.100
```

## Install

```sh
cargo install --git https://github.com/gustafeden/rsfetch
```

Or build from source:

```sh
git clone https://github.com/gustafeden/rsfetch
cd rsfetch
cargo build --release
./target/release/rsfetch
```

## Usage

```
rsfetch [OPTIONS]

Options:
  -c, --color <COLOR>  Color theme [default: green]
  -l, --logo <LOGO>    Logo to display
      --no-logo        Hide the logo
      --json           Output as JSON
      --clear-cache    Clear the cache and re-gather all info
  -h, --help           Print help
  -V, --version        Print version
```

### Color themes

`green` (default), `cyan`, `red`, `magenta`, `yellow`, `blue`, `mono`

```sh
rsfetch -c cyan
rsfetch -c red
```

### Logos

Auto-detected by OS. Override with `--logo`:

`apple`, `linux`, `ubuntu`, `arch`, `debian`, `fedora`, `none`

```sh
rsfetch --logo arch
rsfetch --no-logo
```

### JSON output

```sh
rsfetch --json
```

## Fields

| Field | macOS | Linux |
|-------|-------|-------|
| OS | version + arch | distro + arch |
| Host | hardware model | DMI product name |
| Kernel | Darwin version | Linux version |
| Uptime | days, hours, mins | days, hours, mins |
| Packages | brew | dpkg, pacman, rpm, flatpak, snap |
| Shell | name + version | name + version |
| Resolution | CoreGraphics API | - |
| DE | Aqua | XDG_CURRENT_DESKTOP |
| WM | Quartz Compositor | XDG_SESSION_TYPE |
| WM Theme | accent color + dark/light | GTK theme |
| Terminal | TERM_PROGRAM | TERM_PROGRAM |
| CPU | model + core count | model + core count |
| GPU | SoC name | lspci VGA |
| Memory | used / total MiB | used / total MiB |
| Disk | root filesystem GiB | root filesystem GiB |
| Local IP | first non-loopback IPv4 | first non-loopback IPv4 |

## How it's fast

- Compiled Rust binary — no interpreter startup
- Direct syscalls for host model, disk stats, display resolution (no subprocess spawning)
- Boot-cycle cache (`~/.cache/rsfetch/cache`) for fields that don't change between reboots
- Only two subprocess calls on macOS (`defaults read` for theme), and those are cached
- sysinfo crate for memory/CPU instead of parsing command output

## Dependencies

- [blaeck](https://github.com/gustafeden/blaeck) — inline terminal UI framework
- [sysinfo](https://crates.io/crates/sysinfo) — cross-platform system information
- [clap](https://crates.io/crates/clap) — CLI argument parsing

## License

MIT
