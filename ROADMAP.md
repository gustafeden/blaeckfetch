# rsfetch roadmap

## v0.2 — Boot Sequence Mode

A Sega CD / retro console inspired boot screen for your terminal.
Inline (not fullscreen), animated, interactive. Feels like powering on
a console from the early 90s.

### What you see

```
╭──────────────────────────────────────────────────────────────────╮
│  macOS 15.5 · M3 Pro · 29/36 GiB · 33d up                      │
├──────────────────────────────────────────────────────────────────┤
│                                                                  │
│  ·          ✦                    ·            .         ·        │
│        .          ·    .                  ✦         .            │
│    ·  ╭───╮                                                     │
│   ╭───│   │  .        ·               .        ▄▄▄▄▄▄▄▄▄       │
│   │   ╰───╯       .              ·         ▄████████████████    │
│     ·       .                           ▄██████████████████████ │
│  .      ·             ╔╦╗╔═╗╔═╗╔═╗╔═╗  ████████████████████████│
│              ·        ║║║╠═╣║  ║ ║╚═╗  ████████████████████████│
│   .     ·        .    ╩ ╩╩ ╩╚═╝╚═╝╚═╝  ▀██████████████████████│
│       .                  ╔╗ ╔═╗ ╔═╗        ▀████████████████    │
│  ·          .     ·      ║║ ╠╣  ╠╣            ▀▀▀▀▀▀▀▀▀        │
│         ·            .   ╚╝ ╚   ╚          ·        .           │
│    .          ·   .            .       ·           ·             │
│         .              ·           .          .         ·        │
│                                                                  │
├──────────────────────────────────────────────────────────────────┤
│                     press any key to start                       │
╰──────────────────────────────────────────────────────────────────╯
```

### Layers (back to front)

1. **Border** — rounded box (`╭╮╰╯│─`), static, dim white or theme color
2. **Status bar** — top row inside border, compact system info, dim
3. **Starfield** — procedurally scattered stars (`.` `·` `*` `✦`),
   twinkling (random brightness/color shifts each frame)
4. **Earth** — bottom-right corner, static ASCII art using RGB block
   characters (`▄█▀`), earthy palette (browns, blues, greens)
5. **Moon** — upper-left area, small static ASCII circle, pale grey/white
6. **OS title** — centered, large figlet-style double-line text
   ("MacOS 15.5" or hostname), scrolling gradient animation
7. **Footer** — "press any key to start", blinking (visible/hidden toggle
   every 30 frames)

### Animation timeline

**State machine:** `Entrance → Alive → Freeze → Done`

#### Entrance (~600ms)

1. Border draws itself — top line sweeps left→right, sides drop down,
   bottom sweeps left→right (like a CRT turning on)
2. Stars fade in — random stars appear one by one over ~200ms
3. Earth and moon fade in (dim → full brightness over ~150ms)
4. OS title materializes — characters appear randomly, scattered, then
   snap to their correct positions
5. Status bar types itself in, character by character
6. Footer fades in last, starts blinking

#### Alive (indefinite, until keypress)

Running simultaneously at ~30fps:
- **Stars twinkle** — each frame, ~3-5 random stars change:
  pick new brightness (dim/normal/bright), occasionally shift color
  (white → pale blue → pale yellow → white)
- **OS title gradient scrolls** — smooth RGB gradient mapped to each
  character column, offset shifts each frame. One full cycle ~4s.
- **Footer blinks** — "press any key to start" toggles visible/hidden
  every ~1s (30 frames on, 30 frames off)
- Everything else static (earth, moon, border, status bar)

#### Freeze (~200ms, triggered by keypress)

1. All stars flash bright for 2 frames
2. OS title gradient locks to static theme color
3. Footer stops blinking, text changes to "ready" for a beat
4. Quick dissolve — all content fades: stars disappear first,
   then earth/moon, then title, border last
5. Or simpler: everything vanishes instantly, clean exit

#### Done

- Restore terminal (show cursor, cooked mode)
- Print newline
- Exit — shell prompt appears on fresh line below

### Canvas system

The entire boot screen is a 2D grid of cells:

```rust
struct Cell {
    ch: char,
    fg: (u8, u8, u8),  // RGB foreground
    bg: Option<(u8, u8, u8)>,  // RGB background, None = transparent/default
}

struct Canvas {
    width: u16,
    height: u16,
    cells: Vec<Cell>,
    prev_cells: Vec<Cell>,  // for frame diffing
}
```

Each frame:
1. Clear canvas to default (space, no color)
2. Draw layers back-to-front (starfield → earth → moon → title → border
   → status → footer)
3. Diff `cells` vs `prev_cells`
4. Emit ANSI escape codes only for changed cells
5. Flush stdout
6. Swap `cells` into `prev_cells`

Frame diffing keeps bandwidth low — most frames only a few stars change
plus the gradient offset on the title. Maybe 50-100 cells change per
frame out of ~1500 total.

**Or** use blaeck's built-in rendering which already does diffing via
LogUpdate. Build the frame as a single string with embedded ANSI color
codes, pass it to `blaeck.render()`. Blaeck handles the cursor
positioning and diff. This is simpler and stays inline.

### Procedural starfield

```rust
fn generate_starfield(width: u16, height: u16, density: f32) -> Vec<Star> {
    // Seeded RNG for consistency (same stars each boot, different twinkle)
    // density ~0.02 = ~2% of cells have a star
    // Each star: position (x, y), base brightness, current brightness
    // Star characters: weighted random from ['.', '·', '*', '✦', '⋅']
    //   '.' most common (70%), '·' (15%), '*' (10%), '✦' (5%)
}
```

Twinkle: each frame, pick 3-5 random stars. For each:
- Shift brightness: cycle through dim(90,90,90) → normal(160,160,160)
  → bright(255,255,255) → normal → dim
- 10% chance: shift color slightly (add ±20 to blue channel for a
  cool-star effect, or ±20 to red for a warm-star effect)

### Earth ASCII art

Bottom-right corner, ~12 lines tall, ~25 chars wide. Built from half-block
characters (`▄▀█░▒▓`) with RGB colors:

- Ocean: deep blue (30, 60, 120)
- Land: tan/brown (160, 130, 80), green (60, 120, 60)
- Atmosphere edge: light blue glow (100, 150, 200)
- Shadow side: very dark (20, 20, 30)

Static — no animation needed. The curve of the earth is the visual anchor.

### Moon ASCII art

Upper-left area, ~5 lines tall, ~8 chars wide. Simple circle with
crater shading:

- Light side: bright grey (200, 200, 190)
- Craters: darker grey (140, 130, 120)
- Shadow: dark (80, 75, 70)

Static.

### OS title — figlet-style text

Use double-line box-drawing characters to render "MacOS 15.5" (or
detected OS + version) in large centered text:

```
╔╦╗╔═╗╔═╗╔═╗╔═╗
║║║╠═╣║  ║ ║╚═╗
╩ ╩╩ ╩╚═╝╚═╝╚═╝
    ╔╗ ╔═╗ ╔═╗
    ║║ ╠╣  ╠╣
    ╚╝ ╚   ╚
```

Gradient: define a palette of ~12 RGB colors forming a smooth arc
(e.g., warm amber → theme color → cool blue). Map each character column
to a position in the palette based on `(column + frame_offset) % len`.
Shift `frame_offset` by 1 each frame for scrolling effect.

For other OSes: render "Ubuntu 24.04", "Arch Linux", etc. Use the same
box-drawing font renderer.

### Compact status bar

Single line, top of the frame:

```
macOS 15.5 · M3 Pro · 29/36 GiB · 33d up
```

Assembled from SystemInfo fields. Separator: ` · ` (middle dot).
Color: dim white. Static after entrance typing animation.

### Footer

Centered, below the scene:

```
press any key to start
```

Blinking: alternate between visible (dim white) and hidden (spaces)
every ~1 second. Classic retro game prompt.

### File structure

```
src/
  boot/
    mod.rs          — public API: run_boot_sequence()
    canvas.rs       — Canvas struct, cell diffing, ANSI rendering
    starfield.rs    — star generation, twinkle animation
    earth.rs        — earth ASCII art data + RGB colors
    moon.rs         — moon ASCII art data + RGB colors
    title.rs        — figlet text renderer, gradient animation
    border.rs       — box border drawing
    status.rs       — compact status line builder
    timeline.rs     — state machine (Entrance/Alive/Freeze/Done)
                      entrance sub-animations, frame timing
  main.rs           — add --boot flag, call boot::run_boot_sequence()
  animate.rs        — keep existing color-cycle mode as --animate
  ...existing files...
```

### CLI

```
rsfetch              # normal static fetch (unchanged)
rsfetch --animate    # existing color-cycle animation (unchanged)
rsfetch --boot       # new boot sequence mode
```

Config:
```toml
[boot]
enabled = true       # use boot mode by default (for .zshrc)
timeout = 120        # auto-exit after N seconds
# theme = "sega"     # future: different boot themes
```

### Dependencies

No new crates needed:
- `libc` — already added, for raw mode + keypress detection
- All rendering is raw ANSI escape codes or via blaeck's renderer
- Figlet text: hand-coded box-drawing font, no external crate
- RNG for starfield: use simple LCG or xorshift, no rand crate

### Verification

1. `cargo build` — compiles
2. `rsfetch --boot` — border draws, stars appear, earth/moon render,
   title materializes with gradient, footer blinks
3. Press any key — freeze animation, clean exit, prompt appears
4. `rsfetch` — normal mode still works, no regression
5. `rsfetch --animate` — color cycle mode still works
6. Test in multiple terminals: WezTerm, iTerm2, Terminal.app
7. Test with different terminal widths (the canvas should adapt or
   use a fixed width and center itself)
