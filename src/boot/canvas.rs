use std::io::Write;
use blaeck::{supports_truecolor, rgb_to_256};

#[derive(Clone, Copy, PartialEq)]
pub struct Cell {
    pub ch: char,
    pub fg: (u8, u8, u8),
    pub bg: Option<(u8, u8, u8)>,
}

impl Default for Cell {
    fn default() -> Self {
        Self {
            ch: ' ',
            fg: (255, 255, 255),
            bg: None,
        }
    }
}

pub struct Canvas {
    pub width: u16,
    pub height: u16,
    cells: Vec<Cell>,
    prev_cells: Vec<Cell>,
    first_frame: bool,
    image_mask: Vec<bool>,
    truecolor: bool,
}

impl Canvas {
    pub fn new(width: u16, height: u16) -> Self {
        let size = (width as usize) * (height as usize);
        Self {
            width,
            height,
            cells: vec![Cell::default(); size],
            prev_cells: vec![Cell::default(); size],
            first_frame: true,
            image_mask: vec![false; size],
            truecolor: supports_truecolor(),
        }
    }

    pub fn clear(&mut self) {
        for cell in &mut self.cells {
            *cell = Cell::default();
        }
    }

    /// Force a full redraw on the next render (used after terminal resize).
    pub fn invalidate(&mut self) {
        self.first_frame = true;
    }

    fn idx(&self, x: u16, y: u16) -> usize {
        (y as usize) * (self.width as usize) + (x as usize)
    }

    pub fn set(&mut self, x: u16, y: u16, ch: char, fg: (u8, u8, u8), bg: Option<(u8, u8, u8)>) {
        if x < self.width && y < self.height {
            let i = self.idx(x, y);
            self.cells[i] = Cell { ch, fg, bg };
        }
    }

    pub fn put_str(
        &mut self,
        x: u16,
        y: u16,
        s: &str,
        fg: (u8, u8, u8),
        bg: Option<(u8, u8, u8)>,
    ) {
        for (i, ch) in s.chars().enumerate() {
            let cx = x + i as u16;
            if cx < self.width {
                self.set(cx, y, ch, fg, bg);
            }
        }
    }

    /// Mark a rectangular region as image-backed.
    pub fn set_image_mask(&mut self, x1: u16, y1: u16, x2: u16, y2: u16) {
        for y in y1..y2.min(self.height) {
            for x in x1..x2.min(self.width) {
                let i = self.idx(x, y);
                self.image_mask[i] = true;
            }
        }
    }

    /// Clear all image mask bits.
    pub fn clear_image_mask(&mut self) {
        for m in &mut self.image_mask {
            *m = false;
        }
    }

    /// Clear image mask for a region (used during collapse to allow spaces to overwrite image).
    pub fn unmask_region(&mut self, x1: u16, y1: u16, x2: u16, y2: u16) {
        for y in y1..y2.min(self.height) {
            for x in x1..x2.min(self.width) {
                let i = self.idx(x, y);
                self.image_mask[i] = false;
            }
        }
    }

    /// Render only changed cells to stdout using ANSI escape codes.
    /// origin_row/origin_col are 1-based terminal coordinates.
    pub fn render(&mut self, stdout: &mut impl Write, origin_row: u16, origin_col: u16) {
        let mut buf = String::with_capacity(8192);

        for y in 0..self.height {
            let mut x = 0u16;
            while x < self.width {
                let i = self.idx(x, y);

                // Skip image-masked cells that haven't been explicitly painted
                if self.image_mask[i] && self.cells[i] == Cell::default() {
                    x += 1;
                    continue;
                }

                if self.first_frame || self.cells[i] != self.prev_cells[i] {
                    // Position cursor at this cell
                    buf.push_str(&format!(
                        "\x1b[{};{}H",
                        origin_row + y,
                        origin_col + x
                    ));

                    // Draw consecutive changed cells in one batch
                    while x < self.width {
                        let i = self.idx(x, y);

                        // Stop batch at image-masked default cells
                        if self.image_mask[i] && self.cells[i] == Cell::default() {
                            break;
                        }

                        if !self.first_frame && self.cells[i] == self.prev_cells[i] {
                            break;
                        }
                        let cell = self.cells[i];

                        let (r, g, b) = cell.fg;
                        if self.truecolor {
                            buf.push_str(&format!("\x1b[38;2;{};{};{}m", r, g, b));
                        } else {
                            buf.push_str(&format!("\x1b[38;5;{}m", rgb_to_256(r, g, b)));
                        }

                        if let Some((br, bg, bb)) = cell.bg {
                            if self.truecolor {
                                buf.push_str(&format!("\x1b[48;2;{};{};{}m", br, bg, bb));
                            } else {
                                buf.push_str(&format!("\x1b[48;5;{}m", rgb_to_256(br, bg, bb)));
                            }
                        } else {
                            buf.push_str("\x1b[49m");
                        }

                        buf.push(cell.ch);
                        x += 1;
                    }
                    buf.push_str("\x1b[0m");
                } else {
                    x += 1;
                }
            }
        }

        let _ = stdout.write_all(buf.as_bytes());
        let _ = stdout.flush();

        self.prev_cells.copy_from_slice(&self.cells);
        self.first_frame = false;
    }

    /// Render the entire canvas inline to stdout (no cursor positioning).
    /// Used for VHS and other terminals that don't support raw mode.
    pub fn render_inline(&self, stdout: &mut impl Write) {
        for y in 0..self.height {
            for x in 0..self.width {
                let i = self.idx(x, y);
                let cell = self.cells[i];

                let (r, g, b) = cell.fg;
                if self.truecolor {
                    if let Some((br, bg, bb)) = cell.bg {
                        let _ = write!(
                            stdout,
                            "\x1b[38;2;{};{};{};48;2;{};{};{}m{}",
                            r, g, b, br, bg, bb, cell.ch
                        );
                    } else {
                        let _ = write!(stdout, "\x1b[49;38;2;{};{};{}m{}", r, g, b, cell.ch);
                    }
                } else {
                    let fg_idx = rgb_to_256(r, g, b);
                    if let Some((br, bg, bb)) = cell.bg {
                        let bg_idx = rgb_to_256(br, bg, bb);
                        let _ = write!(stdout, "\x1b[38;5;{};48;5;{}m{}", fg_idx, bg_idx, cell.ch);
                    } else {
                        let _ = write!(stdout, "\x1b[49;38;5;{}m{}", fg_idx, cell.ch);
                    }
                }
            }
            let _ = writeln!(stdout, "\x1b[0m");
        }
        let _ = stdout.flush();
    }
}
