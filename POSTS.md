# Community Posts

Prepared posts for distribution. Ready to publish once name resolution is complete.

---

## Show HN (news.ycombinator.com)

**Title:**
Show HN: rsfetch – System fetch tool 57x faster than neofetch, with retro boot mode

**Body:**
I built rsfetch as a faster alternative to neofetch with a unique feature: a retro console-style boot animation mode.

Performance: ~7ms vs neofetch's ~400ms. Uses a boot-cycle cache for static fields (hostname, CPU model, etc.) and direct syscalls instead of subprocess spawning.

The boot mode is what makes it different - procedural starfield with earth/moon, or use your own background image. It converts images to half-block terminal characters with full RGB color, and supports iTerm2's inline image protocol for full-resolution rendering.

Written in Rust, distributed via Homebrew and AUR. Currently working on getting it published to crates.io.

Demo GIFs in the README: https://github.com/gustafeden/rsfetch

Happy to answer questions about the implementation.

---

## r/unixporn

**Title:**
[OC] rsfetch - neofetch alternative with retro boot animation mode

**Body:**
Built a system fetch tool with a unique boot mode that shows a procedural starfield (or custom background image) before displaying system info.

Features:
- ~7ms execution time (57x faster than neofetch)
- Retro console-style boot animation with starfield
- Custom background images with half-block rendering (▄▀█)
- Configurable entrance/exit animations (slow/fast/instant)
- Alignment options (left/center/right)
- iTerm2 inline image support for full-resolution backgrounds

The boot animation is what makes it different - no other fetch tool has this. Perfect for terminal startup splash screens.

Install:
```
brew tap gustafeden/tap && brew install rsfetch   # macOS
yay -S rsfetch-gustafeden-bin                     # Arch Linux
```

Demo: https://github.com/gustafeden/rsfetch

[Image: demo/boot.gif]
[Image: demo/boot-image.gif]
[Image: demo/fetch.gif]

---

## r/rust

**Title:**
rsfetch: Built a neofetch alternative 57x faster, with procedural starfield boot mode

**Body:**
I built rsfetch as a faster alternative to neofetch, with a unique retro boot animation mode.

**Performance:**
- ~7ms vs neofetch's ~400ms (57x faster)
- Boot-cycle cache for static fields (hostname, CPU model, disk info)
- Direct syscalls instead of subprocess spawning
- Only 2 subprocess calls on macOS (both cached)

**Technical highlights:**
- Custom terminal UI with [blaeck](https://github.com/gustafeden/blaeck) rendering framework
- Procedural starfield generation with parallel star animation
- Image-to-terminal conversion: PNG/JPEG → half-block chars (▄▀█) with full RGB
- iTerm2 inline image protocol support for full-resolution backgrounds
- Cross-platform (macOS + Linux, Intel + ARM)

**Unique feature:**
Boot animation mode - no other fetch tool has this. Shows a procedural starfield (or custom background image) with configurable entrance/exit animations. Great for terminal startup splash screens.

Distributed via Homebrew and AUR. Working on crates.io publishing.

Repo: https://github.com/gustafeden/rsfetch

Built this to learn more about terminal rendering and syscall optimization. Happy to answer questions about the implementation.

---

## X/Twitter

**Option 1 (visual focus):**
Built rsfetch - a system fetch tool that's 57x faster than neofetch ⚡

The unique part: retro boot mode with procedural starfield 🌟

Written in Rust. ~7ms execution time.

[GIF: demo/boot.gif]

https://github.com/gustafeden/rsfetch

**Option 2 (performance focus):**
neofetch: ~400ms ❌
rsfetch: ~7ms ✅

57x faster system fetch tool in Rust, with a retro console boot animation mode 🚀

[GIF: demo/boot-image.gif]

https://github.com/gustafeden/rsfetch

**Option 3 (demo focus):**
Built a neofetch alternative with a retro boot mode 🌟

Check out the procedural starfield animation:

[GIF: demo/boot.gif]

57x faster • Rust • Custom backgrounds • ~7ms runtime

https://github.com/gustafeden/rsfetch

---

## Future Posts

### r/commandline
**Title:** rsfetch - Fast system fetch with retro boot animation (neofetch alternative)

**Body:** [Similar to r/rust but less technical, focus on practical usage and comparison with other fetch tools like fastfetch]

### Dev.to
**Title:** Building a System Fetch Tool 57x Faster Than Neofetch

**Topics to cover:**
- Why neofetch is slow (shell script, subprocess spawning)
- Optimization strategies (boot-cycle cache, direct syscalls)
- Building the boot animation system
- Image-to-terminal conversion algorithm
- Terminal rendering with blaeck
- Benchmarking and profiling results

### Hashnode
**Title:** How I Made a Terminal Tool 57x Faster by Switching to Rust

**Focus:** Technical deep-dive into performance optimizations and Rust's advantages for CLI tools.

