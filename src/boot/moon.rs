use super::canvas::Canvas;

const MOON_LIGHT: (u8, u8, u8) = (200, 200, 190);
const MOON_CRATER: (u8, u8, u8) = (130, 125, 115);
const MOON_SHADOW: (u8, u8, u8) = (70, 65, 60);

/// Draw a small moon at (cx, cy) with given radius.
pub fn draw(canvas: &mut Canvas, cx: f32, cy: f32, radius: f32) {
    let r2 = radius * radius;

    let y_min = (cy - radius - 1.0).max(0.0) as i16;
    let y_max = ((cy + radius + 1.0) as i16).min(canvas.height as i16 - 1);
    let x_min = (cx - radius * 2.0 - 1.0).max(0.0) as i16;
    let x_max = ((cx + radius * 2.0 + 1.0) as i16).min(canvas.width as i16 - 1);

    for iy in y_min..=y_max {
        for ix in x_min..=x_max {
            let fx = (ix as f32 - cx) / 2.0;
            let fy = iy as f32 - cy;
            let dist2 = fx * fx + fy * fy;

            if dist2 <= r2 {
                // Light from upper-left
                let light = 1.0 - ((fx + fy * 0.3) / radius * 0.4 + 0.1).max(0.0).min(1.0) * 0.7;

                let is_crater = crater_pattern(fx, fy, radius);

                let base = if light < 0.4 {
                    MOON_SHADOW
                } else if is_crater {
                    MOON_CRATER
                } else {
                    MOON_LIGHT
                };

                let color = (
                    (base.0 as f32 * light).min(255.0) as u8,
                    (base.1 as f32 * light).min(255.0) as u8,
                    (base.2 as f32 * light).min(255.0) as u8,
                );

                canvas.set(ix as u16, iy as u16, '█', color, None);
            }
        }
    }
}

fn crater_pattern(fx: f32, fy: f32, radius: f32) -> bool {
    let craters: &[(f32, f32, f32)] = &[
        (0.2, -0.25, 0.18),
        (-0.25, 0.15, 0.12),
        (0.05, 0.3, 0.10),
    ];

    for &(cx, cy, cr) in craters {
        let dx = fx / radius - cx;
        let dy = fy / radius - cy;
        if dx * dx + dy * dy < cr * cr {
            return true;
        }
    }
    false
}
