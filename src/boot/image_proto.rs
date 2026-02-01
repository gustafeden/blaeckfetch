use base64::Engine;
use std::io::Write;

#[derive(Clone, Copy, PartialEq)]
pub enum RenderMode {
    Ascii,
    Image,
}

/// Detect whether the terminal supports the iTerm2 inline image protocol.
/// `config_mode` overrides auto-detection: "image" forces Image, "ascii" forces Ascii.
pub fn detect(config_mode: Option<&str>) -> RenderMode {
    match config_mode {
        Some("image") => return RenderMode::Image,
        Some("ascii") => return RenderMode::Ascii,
        _ => {}
    }

    if let Ok(prog) = std::env::var("TERM_PROGRAM") {
        match prog.as_str() {
            "WezTerm" | "iTerm.app" => RenderMode::Image,
            _ => RenderMode::Ascii,
        }
    } else {
        RenderMode::Ascii
    }
}

/// Query terminal cell size in pixels via ioctl.
pub fn cell_pixel_size() -> Option<(u16, u16)> {
    unsafe {
        let mut ws: libc::winsize = std::mem::zeroed();
        if libc::ioctl(1, libc::TIOCGWINSZ, &mut ws) == 0
            && ws.ws_xpixel > 0
            && ws.ws_ypixel > 0
            && ws.ws_col > 0
            && ws.ws_row > 0
        {
            let cw = ws.ws_xpixel / ws.ws_col;
            let ch = ws.ws_ypixel / ws.ws_row;
            Some((cw, ch))
        } else {
            None
        }
    }
}

/// Emit an inline image using the iTerm2 protocol.
///
/// Positions the cursor at `(origin_row, origin_col)` (1-based terminal coords)
/// and writes the escape sequence with base64-encoded image data.
/// `w` and `h` are the image dimensions in terminal cells.
pub fn emit(
    stdout: &mut impl Write,
    image_bytes: &[u8],
    w: u16,
    h: u16,
    origin_row: u16,
    origin_col: u16,
) {
    let b64 = base64::engine::general_purpose::STANDARD.encode(image_bytes);

    // Position cursor at origin
    let _ = write!(stdout, "\x1b[{};{}H", origin_row, origin_col);

    // Use pixel dimensions if available (avoids cell-rounding issues in WezTerm)
    if let Some((cw, ch)) = cell_pixel_size() {
        let px_w = (w as u32) * (cw as u32);
        let px_h = (h as u32) * (ch as u32);
        let _ = write!(
            stdout,
            "\x1b]1337;File=inline=1;width={}px;height={}px;preserveAspectRatio=0;doNotMoveCursor=1:{}\x07",
            px_w, px_h, b64
        );
    } else {
        let _ = write!(
            stdout,
            "\x1b]1337;File=inline=1;width={};height={};preserveAspectRatio=0;doNotMoveCursor=1:{}\x07",
            w, h, b64
        );
    }

    let _ = stdout.flush();
}
