# Configuration

## Config File Location

blaeckfetch reads configuration from:

```
~/.config/blaeckfetch/config.toml
```

## Generating Default Config

To see all available options with their defaults:

```bash
blaeckfetch --print-config
```

To create a config file:

```bash
blaeckfetch --print-config > ~/.config/blaeckfetch/config.toml
```

## Configuration Options

| Option | Type | Description |
|--------|------|-------------|
| `mode` | string | Display mode: `"default"`, `"neofetch"`, or `"splash"` |
| `color` | string | Color theme name |
| `logo` | string | Logo name or `"auto"` |
| `logo_file` | string | Path to custom ASCII art file |
| `palette` | bool | Show color palette (default: `true`) |
| `separator` | string | Separator character (default: `"-"`) |
| `fields` | list | Fields to show, in order |
| `labels` | table | Rename any field label |
| `colors.title` | string/rgb | Title color |
| `colors.label` | string/rgb | Label color |
| `colors.separator` | string/rgb | Separator color |
| `colors.logo` | string/rgb | Logo color |

## Color Values

Colors can be specified as:

### Named Colors

`green` (default), `cyan`, `red`, `magenta`, `yellow`, `blue`, `mono`

### RGB Arrays

```toml
color = [255, 165, 0]  # Orange
```

## Full Example Config

```toml
color = "cyan"
logo = "arch"
palette = false
separator = "="

# Show only these fields, in this order
fields = [
    "OS",
    "CPU",
    "Memory",
    "Disk (/)",
]

# Rename fields
[labels]
"Disk (/)" = "Disk"
"Local IP" = "IP"

# Custom colors (named or RGB)
[colors]
title = [255, 165, 0]
label = "cyan"
separator = "dark_gray"
logo = "magenta"
```

## Fields Reference

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
| Disk (/) | root filesystem GiB | root filesystem GiB |
| Local IP | first non-loopback IPv4 | first non-loopback IPv4 |

## CLI Flags Override Config

Command-line flags always take precedence over config file settings. For example:

```bash
blaeckfetch -c magenta --logo arch
```

This will use magenta color and arch logo, even if your config specifies different values.
