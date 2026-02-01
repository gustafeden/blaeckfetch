use super::canvas::Canvas;

/// Draw the rounded box border with entrance animation support.
/// `progress` ranges from 0.0 (nothing) to 1.0 (fully drawn).
pub fn draw_border(canvas: &mut Canvas, color: (u8, u8, u8), progress: f32) {
    if progress <= 0.0 {
        return;
    }

    let w = canvas.width;
    let h = canvas.height;

    // Phase 1 (0.0–0.35): Top line sweeps left→right
    let top_progress = (progress / 0.35).min(1.0);
    let top_chars = ((w as f32 - 2.0) * top_progress) as u16;

    canvas.set(0, 0, '╭', color, None);
    for x in 1..=top_chars.min(w - 2) {
        canvas.set(x, 0, '─', color, None);
    }
    if top_progress >= 1.0 {
        canvas.set(w - 1, 0, '╮', color, None);
    }

    // Phase 2 (0.30–0.75): Sides drop down
    if progress > 0.30 {
        let side_progress = ((progress - 0.30) / 0.45).min(1.0);
        let side_rows = ((h as f32 - 2.0) * side_progress) as u16;
        for y in 1..=side_rows.min(h - 2) {
            canvas.set(0, y, '│', color, None);
            canvas.set(w - 1, y, '│', color, None);
        }
    }

    // Phase 3 (0.70–1.0): Bottom line sweeps left→right
    if progress > 0.70 {
        let bot_progress = ((progress - 0.70) / 0.30).min(1.0);
        let bot_chars = ((w as f32 - 2.0) * bot_progress) as u16;

        canvas.set(0, h - 1, '╰', color, None);
        for x in 1..=bot_chars.min(w - 2) {
            canvas.set(x, h - 1, '─', color, None);
        }
        if bot_progress >= 1.0 {
            canvas.set(w - 1, h - 1, '╯', color, None);
        }
    }

    // Separators (only when fully drawn)
    if progress >= 1.0 {
        // Upper separator — row 2
        canvas.set(0, 2, '├', color, None);
        for x in 1..w - 1 {
            canvas.set(x, 2, '─', color, None);
        }
        canvas.set(w - 1, 2, '┤', color, None);

        // Lower separator — row h-3
        canvas.set(0, h - 3, '├', color, None);
        for x in 1..w - 1 {
            canvas.set(x, h - 3, '─', color, None);
        }
        canvas.set(w - 1, h - 3, '┤', color, None);
    }
}
