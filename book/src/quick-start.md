# Quick Start

## First Run

After installing blaeckfetch, simply run:

```bash
blaeckfetch
```

You'll see the default moon logo with your system information.

## Basic CLI Examples

### Color Themes

Change the color scheme:

```bash
blaeckfetch -c cyan
blaeckfetch -c magenta
```

### Logos

Use different built-in logos:

```bash
blaeckfetch --logo arch
blaeckfetch --logo ubuntu
blaeckfetch --no-logo
```

Use a custom ASCII logo from a file:

```bash
blaeckfetch --logo-file ~/my-logo.txt
```

### JSON Output

Get machine-readable JSON output:

```bash
blaeckfetch --json
```

### Modes

Try splash mode:

```bash
blaeckfetch --splash
```

Or neofetch compatibility mode:

```bash
blaeckfetch --neofetch
```

## Configuration File

blaeckfetch looks for a config file at:

```
~/.config/blaeckfetch/config.toml
```

### Generate Default Config

Create a config file with default settings:

```bash
blaeckfetch --print-config > ~/.config/blaeckfetch/config.toml
```

### Quick Config Example

Here's a minimal config to get you started:

```toml
color = "cyan"
logo = "moon"

fields = [
    "OS",
    "Kernel",
    "Shell",
    "Terminal",
    "CPU",
    "Memory",
    "Uptime"
]

labels = { OS = "os", Kernel = "kernel", Shell = "shell" }
```

For more configuration options, see the [Configuration](configuration.md) page.
