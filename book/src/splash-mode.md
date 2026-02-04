# Splash Mode

![splash mode](/blaeckfetch/demo/splash-image.gif)

Splash mode transforms blaeckfetch into a retro console-inspired splash screen with animated starfield backgrounds and optional custom images. Press any key to dismiss, or it auto-closes after the configured timeout. Perfect for shell startup animations.

## Launching Splash Mode

```bash
blaeckfetch --splash
blaeckfetch --splash --center
```

Add it to your shell RC file (`.zshrc`, `.bashrc`) for a startup splash.

## Background Images

Add a background image in your config:

```toml
[splash]
image = "~/.config/blaeckfetch/space.png"
stretch = "fill"        # fill, fit, or crop
```

Supported formats: PNG and JPEG.

Images are converted to half-block characters (`▄▀█`) — every cell packs two vertical pixels using foreground and background colors. The conversion is pure Rust with no external tools.

On truecolor terminals (iTerm2, WezTerm, Kitty, Ghostty, Alacritty), images render with full 24-bit RGB. Terminals without truecolor (e.g. macOS Terminal.app) automatically fall back to 256-color approximation.

### Stretch Modes

- **fill** — stretch to fit the canvas (default)
- **fit** — letterbox to preserve aspect ratio
- **crop** — fill and trim the edges

## Full-Resolution Inline Images

Terminals that support inline images (iTerm2, WezTerm, Ghostty, kitty) can render the background at full pixel resolution instead of half-block ASCII. blaeckfetch auto-detects this — no config needed.

To force a specific mode:

```toml
[splash]
render_mode = "image"   # force inline image (iTerm2 protocol)
render_mode = "ascii"   # force half-block characters
render_mode = "auto"    # auto-detect (default)
```

### Limitations

- Inline image mode uses the iTerm2 image protocol. Terminals that don't support it will show nothing — use `render_mode = "ascii"` as a fallback.
- Image resizes on terminal resize may briefly flicker.
- VHS and screen recorders capture the half-block fallback, not the inline image.

## Alignment

Control horizontal alignment via CLI flags or config:

```bash
blaeckfetch --splash --left
blaeckfetch --splash --center
blaeckfetch --splash --right
```

```toml
[splash]
align = "left"  # left (default), center, or right
```

## Full Splash Configuration Reference

| Key | Type | Default | Description |
|-----|------|---------|-------------|
| `image` | string | — | Path to background image (PNG/JPEG) |
| `stretch` | string | `"fill"` | `fill` (stretch), `fit` (letterbox), `crop` |
| `width` | int | auto/68 | Canvas width in columns |
| `height` | int | auto/23 | Canvas height in rows |
| `timeout` | int | `120` | Seconds before auto-close |
| `star_brightness` | int | `30` | Star detection threshold (0–255) |
| `render_mode` | string | `"auto"` | `auto` (detect terminal), `image` (force iTerm2), `ascii` (force half-block) |
| `align` | string | `"left"` | `left`, `center`, or `right` (CLI flags override) |
| `min_width` | int | `10` | Minimum canvas width in columns |
| `min_height` | int | `5` | Minimum canvas height in rows |
| `max_width` | int | — | Maximum canvas width in columns |
| `max_height` | int | — | Maximum canvas height in rows |
| `entrance` | string | `"slow"` | Entrance animation: `slow` (~1.2s), `fast` (~400ms), `instant` |
| `exit` | string | `"slow"` | Exit animation: `slow` (~400ms), `fast` (~200ms), `instant` |

## Backward Compatibility

The config section name `[boot]` still works as an alias for `[splash]`.
