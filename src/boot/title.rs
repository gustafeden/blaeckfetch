use super::canvas::Canvas;

/// Box-drawing figlet font: each glyph is 3 rows tall, variable width.
fn glyph(c: char) -> Option<[&'static str; 3]> {
    Some(match c {
        'A' => ["╔═╗", "╠═╣", "╩ ╩"],
        'B' => ["╔═╗", "╠═╗", "╚═╝"],
        'C' => ["╔═╗", "║  ", "╚═╝"],
        'D' => ["╔═╗", "║ ║", "╚═╝"],
        'E' => ["╔══", "╠══", "╚══"],
        'F' => ["╔══", "╠══", "╩  "],
        'G' => ["╔══", "║ ╗", "╚═╝"],
        'H' => ["╔ ╗", "╠═╣", "╩ ╩"],
        'I' => ["╔╗", "║║", "╚╝"],
        'J' => [" ╗", " ║", "╚╝"],
        'K' => ["╔ ╔", "╠═ ", "╩ ╚"],
        'L' => ["╔  ", "║  ", "╚══"],
        'M' => ["╔╦╗", "║║║", "╩ ╩"],
        'N' => ["╔╗ ", "║╚╗", "╩ ╩"],
        'O' => ["╔═╗", "║ ║", "╚═╝"],
        'P' => ["╔═╗", "╠═╝", "╩  "],
        'Q' => ["╔═╗", "║ ║", "╚╦╝"],
        'R' => ["╔═╗", "╠╦╝", "╩╚═"],
        'S' => ["╔═╗", "╚═╗", "╚═╝"],
        'T' => ["╔╦╗", " ║ ", " ╩ "],
        'U' => ["╔ ╗", "║ ║", "╚═╝"],
        'V' => ["╔ ╗", "╚╗║", " ╚╝"],
        'W' => ["╔ ╗", "║║║", "╚╩╝"],
        'X' => ["╗ ╔", " ╬ ", "╝ ╚"],
        'Y' => ["╗ ╔", "╚╦╝", " ╩ "],
        'Z' => ["══╗", "╔═╝", "╚══"],
        '0' => ["╔═╗", "║ ║", "╚═╝"],
        '1' => ["╔╗ ", "║║ ", "╚╝ "],
        '2' => ["╔═╗", " ═╝", "╚══"],
        '3' => ["══╗", " ═╣", "══╝"],
        '4' => ["╔ ╗", "╚═╣", "  ╩"],
        '5' => ["╔══", "╚═╗", "══╝"],
        '6' => ["╔══", "╠═╗", "╚═╝"],
        '7' => ["══╗", "  ║", "  ╩"],
        '8' => ["╔═╗", "╠═╣", "╚═╝"],
        '9' => ["╔═╗", "╚═╣", "══╝"],
        '.' => [" ", " ", "·"],
        ' ' => ["   ", "   ", "   "],
        _ => return None,
    })
}

/// Render text into 3 figlet rows using the box-drawing font.
pub fn render_lines(text: &str) -> [String; 3] {
    let mut rows = [String::new(), String::new(), String::new()];

    for (i, c) in text.chars().enumerate() {
        if i > 0 {
            for row in &mut rows {
                row.push(' ');
            }
        }

        let g = glyph(c.to_ascii_uppercase()).unwrap_or(["?", "?", "?"]);
        for (row_idx, row_str) in g.iter().enumerate() {
            rows[row_idx].push_str(row_str);
        }
    }

    rows
}

/// Scrolling gradient palette (warm amber → cool blue → back).
const GRADIENT: [(u8, u8, u8); 15] = [
    (255, 140, 50),
    (255, 160, 60),
    (255, 180, 80),
    (255, 200, 100),
    (220, 200, 130),
    (180, 200, 180),
    (140, 190, 220),
    (100, 170, 240),
    (80, 150, 255),
    (100, 130, 230),
    (140, 120, 200),
    (180, 110, 170),
    (220, 120, 140),
    (255, 130, 100),
    (255, 140, 60),
];

/// Draw the full title with animated gradient.
pub fn draw(
    canvas: &mut Canvas,
    lines: &[String; 3],
    center_x: u16,
    start_y: u16,
    frame: u32,
    gradient_active: bool,
) {
    for (row_idx, line) in lines.iter().enumerate() {
        let y = start_y + row_idx as u16;
        let char_count = line.chars().count() as u16;
        let x = center_x.saturating_sub(char_count / 2);

        for (col_idx, ch) in line.chars().enumerate() {
            if ch == ' ' {
                continue;
            }

            let color = if gradient_active {
                let palette_idx = ((col_idx as u32 + frame) as usize) % GRADIENT.len();
                GRADIENT[palette_idx]
            } else {
                GRADIENT[0]
            };

            canvas.set(x + col_idx as u16, y, ch, color, None);
        }
    }
}

/// Draw title with materialization effect (characters appear progressively).
/// `progress` ranges from 0.0 (nothing visible) to 1.0 (fully materialized).
pub fn draw_partial(
    canvas: &mut Canvas,
    lines: &[String; 3],
    center_x: u16,
    start_y: u16,
    frame: u32,
    gradient_active: bool,
    progress: f32,
) {
    for (row_idx, line) in lines.iter().enumerate() {
        let y = start_y + row_idx as u16;
        let char_count = line.chars().count() as u16;
        let x = center_x.saturating_sub(char_count / 2);

        for (col_idx, ch) in line.chars().enumerate() {
            if ch == ' ' {
                continue;
            }

            // Deterministic hash to decide when each char appears
            let hash = position_hash(col_idx as u32, row_idx as u32);
            let threshold = (hash % 100) as f32 / 100.0;

            if threshold < progress {
                let color = if gradient_active {
                    let palette_idx = ((col_idx as u32 + frame) as usize) % GRADIENT.len();
                    GRADIENT[palette_idx]
                } else {
                    GRADIENT[0]
                };

                canvas.set(x + col_idx as u16, y, ch, color, None);
            }
        }
    }
}

fn position_hash(x: u32, y: u32) -> u32 {
    let mut h = x
        .wrapping_mul(374761393)
        .wrapping_add(y.wrapping_mul(668265263))
        .wrapping_add(42);
    h = (h ^ (h >> 13)).wrapping_mul(1274126177);
    h ^ (h >> 16)
}
