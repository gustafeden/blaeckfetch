# Modes

blaeckfetch supports three display modes, each with a different visual style and purpose.

## Default Mode

The clean, minimal mode featuring the moon logo with essential system information. This is the evolved minimal identity of blaeckfetch.

```bash
blaeckfetch
# or explicitly
blaeckfetch --mode default
```

In your config:

```toml
mode = "default"
```

## Neofetch Mode

A familiar layout for users coming from neofetch. This mode provides a classic fetch-style display.

```bash
blaeckfetch --neofetch
# or
blaeckfetch --mode neofetch
```

In your config:

```toml
mode = "neofetch"
```

## Splash Mode

A retro console-inspired animated splash screen with starfield backgrounds and optional custom images. Perfect for terminal startup sequences.

```bash
blaeckfetch --splash
# or
blaeckfetch --mode splash
```

In your config:

```toml
mode = "splash"
```

For detailed splash mode configuration options, see the dedicated [Splash Mode](splash-mode.md) page.

## Switching Modes

You can switch modes using:

### CLI Flags

- `--mode default` - Default moon logo mode
- `--mode neofetch` - Neofetch compatibility mode
- `--mode splash` - Splash screen mode
- `--neofetch` - Shorthand for neofetch mode
- `--splash` - Shorthand for splash mode

### Config File

```toml
mode = "default"  # or "neofetch" or "splash"
```

CLI flags always override the config file setting.
