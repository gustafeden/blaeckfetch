# blaeckfetch

A fast, minimalist system fetch for your terminal. Written in Rust. Powered by [blaeck](https://github.com/gustafeden/blaeck) for rendering.

## Default mode

![blaeckfetch default](https://gustafeden.github.io/blaeckfetch/demo/default.gif)

Clean moon logo with essential system info. Everything is customizable:

```toml
logo = "arch"
fields = ["OS", "CPU", "GPU", "Memory", "Disk (/)", "Local IP"]
color = "cyan"
palette = true

[colors]
title = [255, 165, 0]
label = "cyan"
logo = "magenta"
```

## Splash mode

![splash — background image](https://gustafeden.github.io/blaeckfetch/demo/splash-image.gif)

Animated starfield background with optional custom images. Press any key to dismiss. Add `blaeckfetch --splash` to your shell RC for a startup splash.

## Performance

| Tool | Time |
|------|------|
| **blaeckfetch** | **~7ms** |
| neofetch | ~400ms |

~57x faster than neofetch. Boot-cycle cache keeps static fields instant after the first run.

## Install

### macOS

```sh
brew tap gustafeden/tap && brew install blaeckfetch
```

### Arch Linux

```sh
yay -S blaeckfetch-bin
```

### Linux / macOS (installer script)

```sh
curl -fsSL https://gustafeden.github.io/blaeckfetch/install.sh | bash
```

### From source

```sh
cargo install --git https://github.com/gustafeden/blaeckfetch
```

## Usage

```sh
blaeckfetch                  # default mode
blaeckfetch -c cyan          # color theme
blaeckfetch --logo arch      # override logo
blaeckfetch --splash         # splash screen
blaeckfetch --neofetch       # neofetch layout
blaeckfetch --json           # JSON output
```

Generate a config file:

```sh
blaeckfetch --print-config > ~/.config/blaeckfetch/config.toml
```

## Documentation

Full docs — configuration reference, splash mode options, field list, and more:

**[gustafeden.github.io/blaeckfetch](https://gustafeden.github.io/blaeckfetch/)**

## License

MIT
