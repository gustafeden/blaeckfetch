use super::canvas::Canvas;
use super::rng::Rng;

pub struct Star {
    pub x: u16,
    pub y: u16,
    pub ch: char,
    pub brightness: u8, // 0=dim, 1=normal, 2=bright
    pub color_shift: i8, // -1=warm, 0=neutral, 1=cool
}

fn pick_star_char(rng: &mut Rng) -> char {
    let r = rng.range(100);
    if r < 70 {
        '.'
    } else if r < 85 {
        '·'
    } else if r < 95 {
        '*'
    } else if r < 98 {
        '✦'
    } else {
        '⋅'
    }
}

fn brightness_rgb(brightness: u8, color_shift: i8) -> (u8, u8, u8) {
    let base = match brightness {
        0 => 90u8,
        1 => 160,
        _ => 255,
    };

    match color_shift {
        -1 => (base.saturating_add(20), base, base.saturating_sub(20)),
        1 => (base.saturating_sub(20), base, base.saturating_add(20)),
        _ => (base, base, base),
    }
}

/// Generate a procedural starfield within the given bounds.
pub fn generate(
    start_x: u16,
    start_y: u16,
    end_x: u16,
    end_y: u16,
    density: f32,
    rng: &mut Rng,
) -> Vec<Star> {
    let mut stars = Vec::new();

    for y in start_y..end_y {
        for x in start_x..end_x {
            if rng.float() < density {
                stars.push(Star {
                    x,
                    y,
                    ch: pick_star_char(rng),
                    brightness: rng.range(3) as u8,
                    color_shift: 0,
                });
            }
        }
    }

    stars
}

/// Mutate 3–5 random stars per frame for twinkling effect.
pub fn twinkle(stars: &mut [Star], rng: &mut Rng) {
    if stars.is_empty() {
        return;
    }

    let count = 3 + rng.range(3) as usize;

    for _ in 0..count {
        let idx = rng.range(stars.len() as u32) as usize;
        let star = &mut stars[idx];

        star.brightness = rng.range(3) as u8;

        // 10% chance to shift color
        if rng.range(10) == 0 {
            star.color_shift = (rng.range(3) as i8) - 1;
        }
    }
}

/// Draw visible stars onto the canvas.
/// `visible_fraction` controls how many stars appear (0.0–1.0) for entrance fade-in.
pub fn draw(canvas: &mut Canvas, stars: &[Star], visible_fraction: f32) {
    let count = ((stars.len() as f32) * visible_fraction) as usize;

    for star in stars.iter().take(count) {
        let color = brightness_rgb(star.brightness, star.color_shift);
        canvas.set(star.x, star.y, star.ch, color, None);
    }
}

/// Flash all stars bright (used during freeze).
pub fn flash_all(stars: &mut [Star]) {
    for star in stars.iter_mut() {
        star.brightness = 2;
    }
}
