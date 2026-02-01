use super::canvas::Canvas;

const OCEAN: (u8, u8, u8) = (30, 60, 150);
const OCEAN_DEEP: (u8, u8, u8) = (20, 40, 120);
const LAND_GREEN: (u8, u8, u8) = (50, 130, 50);
const LAND_BROWN: (u8, u8, u8) = (150, 120, 70);
const ATMOSPHERE: (u8, u8, u8) = (80, 140, 210);

/// Draw a procedural Earth sphere at (cx, cy) with given radius.
/// Radius is in "character rows" — horizontal span is doubled for aspect ratio.
pub fn draw(canvas: &mut Canvas, cx: f32, cy: f32, radius: f32) {
    let r2 = radius * radius;

    let y_min = (cy - radius - 1.0).max(0.0) as i16;
    let y_max = ((cy + radius + 1.0) as i16).min(canvas.height as i16 - 1);
    let x_min = (cx - radius * 2.0 - 2.0).max(0.0) as i16;
    let x_max = ((cx + radius * 2.0 + 2.0) as i16).min(canvas.width as i16 - 1);

    for iy in y_min..=y_max {
        for ix in x_min..=x_max {
            // Normalize to sphere coordinates (accounting for char aspect ratio ~2:1)
            let fx = (ix as f32 - cx) / 2.0;
            let fy = iy as f32 - cy;
            let dist2 = fx * fx + fy * fy;

            if dist2 <= r2 {
                let norm_dist = (dist2 / r2).sqrt();
                let angle = fy.atan2(fx);

                let is_land = continent_pattern(angle, norm_dist);

                // Shadow: light comes from upper-left, shadow on lower-right
                let shadow = ((fx + fy * 0.5) / radius * 0.5 + 0.3)
                    .max(0.0)
                    .min(1.0);
                let light = 1.0 - shadow * 0.6;

                let base = if is_land {
                    if ((angle * 3.5).sin() * 0.5 + 0.5) > 0.55 {
                        LAND_GREEN
                    } else {
                        LAND_BROWN
                    }
                } else if norm_dist > 0.7 {
                    OCEAN_DEEP
                } else {
                    OCEAN
                };

                let color = scale_color(base, light);
                canvas.set(ix as u16, iy as u16, '█', color, None);
            } else if dist2 <= r2 * 1.12 {
                // Thin atmosphere glow
                let glow = 1.0 - ((dist2 / r2).sqrt() - 1.0) / 0.12;
                let color = scale_color(ATMOSPHERE, glow * 0.6);
                canvas.set(ix as u16, iy as u16, '░', color, None);
            }
        }
    }
}

fn continent_pattern(angle: f32, dist: f32) -> bool {
    // Continent 1: broad landmass (upper-right quadrant)
    if angle > -0.6 && angle < 0.9 && dist > 0.15 && dist < 0.82 {
        return (angle * 2.3 + dist * 3.1).sin() > 0.15;
    }
    // Continent 2: southern landmass
    if angle > 1.4 && angle < 2.6 && dist > 0.25 && dist < 0.70 {
        return true;
    }
    // Small island cluster
    if angle > -2.2 && angle < -1.3 && dist > 0.35 && dist < 0.55 {
        return (angle * 5.0).cos() > 0.3;
    }
    false
}

fn scale_color(c: (u8, u8, u8), factor: f32) -> (u8, u8, u8) {
    (
        (c.0 as f32 * factor).min(255.0) as u8,
        (c.1 as f32 * factor).min(255.0) as u8,
        (c.2 as f32 * factor).min(255.0) as u8,
    )
}
