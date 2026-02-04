use super::canvas::Canvas;
use super::rng::Rng;
use image::DynamicImage;

pub struct BgCell {
    pub ch: char,
    pub fg: (u8, u8, u8),
    pub bg: Option<(u8, u8, u8)>,
    /// Original fg color (for stars, used to restore after twinkle)
    pub base_fg: (u8, u8, u8),
    /// Whether this cell is a star (isolated bright pixel against dark bg)
    pub is_star: bool,
}

/// Get image pixel dimensions and convert to cell dimensions (w, h).
/// Scales down to fit terminal if needed, preserving aspect ratio.
pub fn image_dimensions(path: &str) -> Option<(u16, u16)> {
    let (cell_w, cell_h) = image_cell_size(path)?;
    let (term_w, term_h) = super::terminal_size();
    Some(fit_to_terminal(cell_w, cell_h, term_w, term_h))
}

/// Get the natural cell dimensions of an image (before clamping to terminal).
/// For half-block ASCII mode, each cell covers 2 vertical pixels.
pub fn image_cell_size(path: &str) -> Option<(f32, f32)> {
    let expanded = expand_path(path);
    let img = match image::open(&expanded) {
        Ok(img) => img,
        Err(e) => {
            eprintln!("warning: could not load boot image '{}': {}", path, e);
            return None;
        }
    };
    let cell_w = img.width() as f32;
    let cell_h = img.height() as f32 / 2.0;
    Some((cell_w, cell_h))
}

/// Get cell dimensions for inline image mode, accounting for cell pixel aspect ratio.
/// Each cell is cell_px_w x cell_px_h pixels, so we need to divide image pixels
/// by those to get the number of cells needed.
pub fn image_cell_size_for_proto(path: &str, cell_px_w: u16, cell_px_h: u16) -> Option<(f32, f32)> {
    let expanded = expand_path(path);
    let img = match image::open(&expanded) {
        Ok(img) => img,
        Err(e) => {
            eprintln!("warning: could not load boot image '{}': {}", path, e);
            return None;
        }
    };
    let cell_w = img.width() as f32 / cell_px_w as f32;
    let cell_h = img.height() as f32 / cell_px_h as f32;
    Some((cell_w, cell_h))
}

/// Fit cell dimensions to a terminal size, preserving aspect ratio.
/// `border_cols` and `border_rows` account for the box chrome (borders, separators, footer)
/// so the *interior* image area has the correct aspect ratio.
pub fn fit_to_terminal(cell_w: f32, cell_h: f32, term_w: u16, term_h: u16) -> (u16, u16) {
    fit_to_terminal_with_chrome(cell_w, cell_h, term_w, term_h, 2, 1)
}

pub fn fit_to_terminal_with_chrome(
    cell_w: f32,
    cell_h: f32,
    term_w: u16,
    term_h: u16,
    border_cols: u16,
    border_rows: u16,
) -> (u16, u16) {
    // Maximum interior dimensions available
    let max_interior_w = term_w.saturating_sub(2).saturating_sub(border_cols) as f32;
    let max_interior_h = term_h.saturating_sub(1).saturating_sub(border_rows) as f32;

    let scale_w = max_interior_w / cell_w;
    let scale_h = max_interior_h / cell_h;
    let scale = scale_w.min(scale_h).min(1.0);

    // Interior dimensions that preserve aspect ratio
    let interior_w = (cell_w * scale).floor() as u16;
    let interior_h = (cell_h * scale).floor() as u16;

    // Add back the chrome to get the full box size
    let w = (interior_w + border_cols).max(10);
    let h = (interior_h + border_rows).max(5);

    (w, h)
}

fn expand_path(path: &str) -> String {
    if path.starts_with("~/") {
        if let Ok(home) = std::env::var("HOME") {
            return format!("{}/{}", home, &path[2..]);
        }
    }
    path.to_string()
}

const DEFAULT_STAR_BRIGHTNESS: u8 = 30;

/// Resize an image to the given pixel dimensions using the specified stretch mode.
fn resize_image(img: &DynamicImage, pixel_w: u32, pixel_h: u32, stretch: &str) -> DynamicImage {
    match stretch {
        "fit" => {
            let img_ratio = img.width() as f32 / img.height() as f32;
            let target_ratio = pixel_w as f32 / pixel_h as f32;
            let (rw, rh) = if img_ratio > target_ratio {
                (pixel_w, (pixel_w as f32 / img_ratio) as u32)
            } else {
                ((pixel_h as f32 * img_ratio) as u32, pixel_h)
            };
            let small = img.resize_exact(rw, rh, image::imageops::FilterType::Nearest);
            let mut canvas_img = image::RgbaImage::new(pixel_w, pixel_h);
            let ox = (pixel_w.saturating_sub(rw)) / 2;
            let oy = (pixel_h.saturating_sub(rh)) / 2;
            image::imageops::overlay(&mut canvas_img, &small.to_rgba8(), ox as i64, oy as i64);
            DynamicImage::ImageRgba8(canvas_img)
        }
        "crop" => {
            let img_ratio = img.width() as f32 / img.height() as f32;
            let target_ratio = pixel_w as f32 / pixel_h as f32;
            let (rw, rh) = if img_ratio < target_ratio {
                (pixel_w, (pixel_w as f32 / img_ratio) as u32)
            } else {
                ((pixel_h as f32 * img_ratio) as u32, pixel_h)
            };
            let big = img.resize_exact(rw, rh, image::imageops::FilterType::Nearest);
            let ox = (rw.saturating_sub(pixel_w)) / 2;
            let oy = (rh.saturating_sub(pixel_h)) / 2;
            big.crop_imm(ox, oy, pixel_w, pixel_h)
        }
        _ => {
            img.resize_exact(pixel_w, pixel_h, image::imageops::FilterType::Nearest)
        }
    }
}

/// Load an image and re-encode as PNG bytes (for iTerm2 inline image protocol).
/// `pixel_w` and `pixel_h` are the exact target pixel dimensions.
pub fn load_raw(path: &str, pixel_w: u32, pixel_h: u32, stretch: &str) -> Option<Vec<u8>> {
    let expanded = expand_path(path);
    let img = match image::open(&expanded) {
        Ok(img) => img,
        Err(e) => {
            eprintln!("warning: could not load boot image '{}': {}", path, e);
            return None;
        }
    };

    let resized = resize_image(&img, pixel_w, pixel_h, stretch);

    let mut buf = std::io::Cursor::new(Vec::new());
    if resized
        .write_to(&mut buf, image::ImageFormat::Png)
        .is_err()
    {
        eprintln!("warning: failed to encode image as PNG");
        return None;
    }

    Some(buf.into_inner())
}

pub fn load(path: &str, w: u16, h: u16, stretch: &str, transparency: u8, star_brightness: Option<u8>) -> Option<Vec<BgCell>> {
    let star_thresh = star_brightness.unwrap_or(DEFAULT_STAR_BRIGHTNESS);
    let expanded = expand_path(path);

    let img = match image::open(&expanded) {
        Ok(img) => img,
        Err(e) => {
            eprintln!("warning: could not load boot image '{}': {}", path, e);
            return None;
        }
    };
    let pixel_w = w as u32;
    let pixel_h = (h as u32) * 2;

    let resized = resize_image(&img, pixel_w, pixel_h, stretch);

    let rgba = resized.to_rgba8();
    let mut cells = Vec::with_capacity((w as usize) * (h as usize));

    for row in 0..h {
        for col in 0..w {
            let top_y = (row as u32) * 2;
            let bot_y = top_y + 1;

            let top_px = if top_y < rgba.height() && (col as u32) < rgba.width() {
                let p = rgba.get_pixel(col as u32, top_y);
                (p[0], p[1], p[2], p[3])
            } else {
                (0, 0, 0, 0)
            };

            let bot_px = if bot_y < rgba.height() && (col as u32) < rgba.width() {
                let p = rgba.get_pixel(col as u32, bot_y);
                (p[0], p[1], p[2], p[3])
            } else {
                (0, 0, 0, 0)
            };

            let top_dark = is_dark(top_px, transparency);
            let bot_dark = is_dark(bot_px, transparency);

            // For star detection: "dark" means all channels below a low threshold
            // (near-black space pixels). Stars are bright pixels where BOTH halves'
            // neighbors are all near-black.
            let top_space = px_near_black(top_px, star_thresh);
            let bot_space = px_near_black(bot_px, star_thresh);

            let dark_bg = if transparency > 0 { None } else { Some((0u8, 0u8, 0u8)) };

            let cell = if top_dark && bot_dark {
                BgCell {
                    ch: ' ',
                    fg: (0, 0, 0),
                    bg: dark_bg,
                    base_fg: (0, 0, 0),
                    is_star: false,
                }
            } else if top_dark {
                // Only bottom pixel is bright
                let bright = (bot_px.0, bot_px.1, bot_px.2);
                BgCell {
                    ch: '▄',
                    fg: bright,
                    bg: dark_bg,
                    base_fg: bright,
                    is_star: top_space, // top is space, bottom is the star
                }
            } else if bot_dark {
                // Only top pixel is bright
                let bright = (top_px.0, top_px.1, top_px.2);
                BgCell {
                    ch: '▀',
                    fg: bright,
                    bg: dark_bg,
                    base_fg: bright,
                    is_star: bot_space, // bottom is space, top is the star
                }
            } else {
                // Both pixels visible
                let fg = (top_px.0, top_px.1, top_px.2);
                let bg_color = (bot_px.0, bot_px.1, bot_px.2);
                // Star if one half is near-black space and other isn't
                let star = (top_space && !bot_space) || (bot_space && !top_space);
                let base = if bot_space { fg } else { bg_color };
                BgCell {
                    ch: '▀',
                    fg,
                    bg: Some(bg_color),
                    base_fg: base,
                    is_star: star,
                }
            };

            cells.push(cell);
        }
    }

    // Post-process: a star must be surrounded by all-dim neighbors
    let w_usize = w as usize;
    let h_usize = h as usize;
    let total = w_usize * h_usize;
    for i in 0..total {
        if !cells[i].is_star {
            continue;
        }
        let cx = i % w_usize;
        let cy = i / w_usize;
        let mut isolated = true;
        'outer: for dy in 0..3usize {
            for dx in 0..3usize {
                if dx == 1 && dy == 1 {
                    continue;
                }
                let nx = (cx + dx).wrapping_sub(1);
                let ny = (cy + dy).wrapping_sub(1);
                if nx >= w_usize || ny >= h_usize {
                    continue;
                }
                let ni = ny * w_usize + nx;
                let n = &cells[ni];
                // Neighbor has any non-near-black content = not isolated
                let (r, g, b) = n.fg;
                if r.max(g).max(b) > star_thresh {
                    isolated = false;
                    break 'outer;
                }
                if let Some((br, bg_g, bb)) = n.bg {
                    if br.max(bg_g).max(bb) > star_thresh {
                        isolated = false;
                        break 'outer;
                    }
                }
            }
        }
        if !isolated {
            cells[i].is_star = false;
        }
    }

    Some(cells)
}

fn is_dark(px: (u8, u8, u8, u8), threshold: u8) -> bool {
    threshold > 0 && px.0 <= threshold && px.1 <= threshold && px.2 <= threshold
}

/// Check if a pixel is near-black (space background).
fn px_near_black(px: (u8, u8, u8, u8), threshold: u8) -> bool {
    px.0 <= threshold && px.1 <= threshold && px.2 <= threshold
}

/// 80s-style star blinking: discrete states with sharp transitions.
/// Stars randomly snap between: off, dim, normal, bright white flash.
pub fn twinkle(cells: &mut [BgCell], rng: &mut Rng) {
    for cell in cells.iter_mut() {
        if !cell.is_star {
            continue;
        }
        // ~5% chance per star per frame to change state
        if rng.range(100) >= 5 {
            continue;
        }
        // Pick a random state
        match rng.range(8) {
            0 => {
                // Off — star disappears
                cell.fg = (0, 0, 0);
            }
            1 => {
                // Dim — barely visible
                cell.fg = dim_color(cell.base_fg);
            }
            2..=4 => {
                // Normal — restore original
                cell.fg = cell.base_fg;
            }
            5..=6 => {
                // Bright — boosted version of original color
                cell.fg = bright_color(cell.base_fg);
            }
            _ => {
                // Flash white — full glint
                cell.fg = (255, 255, 255);
            }
        }
    }
}

fn dim_color(c: (u8, u8, u8)) -> (u8, u8, u8) {
    (c.0 / 4, c.1 / 4, c.2 / 4)
}

fn bright_color(c: (u8, u8, u8)) -> (u8, u8, u8) {
    (
        (c.0 as u16 + (255 - c.0 as u16) / 2) as u8,
        (c.1 as u16 + (255 - c.1 as u16) / 2) as u8,
        (c.2 as u16 + (255 - c.2 as u16) / 2) as u8,
    )
}

/// Draw the background image inside the border area only.
pub fn draw(canvas: &mut Canvas, cells: &[BgCell], w: u16) {
    let h = canvas.height;
    for (i, cell) in cells.iter().enumerate() {
        if cell.ch == ' ' && cell.bg.is_none() {
            continue;
        }
        let col = (i % w as usize) as u16;
        let row = (i / w as usize) as u16;
        if row < 3 || row >= h.saturating_sub(2) || col == 0 || col >= w - 1 {
            continue;
        }
        canvas.set(col, row, cell.ch, cell.fg, cell.bg);
    }
}

/// Render background cells directly to stdout as inline ANSI text.
/// No cursor positioning, no raw mode — just prints lines sequentially.
pub fn render_inline(cells: &[BgCell], w: u16) {
    use std::io::{self, Write};
    let mut stdout = io::stdout();
    let h = cells.len() / w as usize;

    for row in 0..h {
        let start = row * w as usize;
        let end = start + w as usize;
        for cell in &cells[start..end] {
            // Set fg color
            let (fr, fg, fb) = cell.fg;
            if let Some((br, bg, bb)) = cell.bg {
                // Both fg and bg
                let _ = write!(stdout, "\x1b[38;2;{};{};{};48;2;{};{};{}m{}", fr, fg, fb, br, bg, bb, cell.ch);
            } else {
                // fg only
                let _ = write!(stdout, "\x1b[38;2;{};{};{}m{}", fr, fg, fb, cell.ch);
            }
        }
        // Reset and newline
        let _ = writeln!(stdout, "\x1b[0m");
    }
    let _ = stdout.flush();
}
