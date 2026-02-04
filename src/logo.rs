#[allow(dead_code)]
pub struct Logo {
    pub art: &'static str,
    pub name: &'static str,
}

pub fn detect() -> Logo {
    #[cfg(target_os = "macos")]
    {
        apple()
    }
    #[cfg(target_os = "linux")]
    {
        detect_linux()
    }
    #[cfg(not(any(target_os = "macos", target_os = "linux")))]
    {
        generic()
    }
}

pub fn from_file(path: &str) -> Result<String, std::io::Error> {
    let expanded = if path.starts_with('~') {
        if let Ok(home) = std::env::var("HOME") {
            format!("{}{}", home, &path[1..])
        } else {
            path.to_string()
        }
    } else {
        path.to_string()
    };
    std::fs::read_to_string(expanded)
}

pub fn by_name(name: &str) -> Logo {
    match name.to_lowercase().as_str() {
        "apple" | "macos" | "mac" => apple(),
        "linux" | "tux" => tux(),
        "ubuntu" => ubuntu(),
        "arch" => arch(),
        "debian" => debian(),
        "fedora" => fedora(),
        "moon" => moon(),
        "none" | "off" => Logo { art: "", name: "none" },
        _ => detect(),
    }
}

#[cfg(target_os = "linux")]
fn detect_linux() -> Logo {
    let id = std::fs::read_to_string("/etc/os-release")
        .ok()
        .and_then(|c| {
            c.lines()
                .find(|l| l.starts_with("ID="))
                .map(|l| l.trim_start_matches("ID=").trim_matches('"').to_lowercase())
        })
        .unwrap_or_default();

    match id.as_str() {
        "ubuntu" => ubuntu(),
        "arch" | "archlinux" => arch(),
        "debian" => debian(),
        "fedora" => fedora(),
        _ => tux(),
    }
}

pub fn apple() -> Logo {
    Logo {
        name: "apple",
        art: "                    'c.
                 ,xNMM.
               .OMMMMo
               OMMM0,
     .;loddo:' loolloddol;.
   cKMMMMMMMMMMNWMMMMMMMMMM0:
 .KMMMMMMMMMMMMMMMMMMMMMMMWd.
 XMMMMMMMMMMMMMMMMMMMMMMMX.
;MMMMMMMMMMMMMMMMMMMMMMMM:
:MMMMMMMMMMMMMMMMMMMMMMMM:
.MMMMMMMMMMMMMMMMMMMMMMMMX.
 kMMMMMMMMMMMMMMMMMMMMMMMMWd.
 .XMMMMMMMMMMMMMMMMMMMMMMMMMMk
  .XMMMMMMMMMMMMMMMMMMMMMMMMK.
    kMMMMMMMMMMMMMMMMMMMMMMd
     ;KMMMMMMMWXXWMMMMMMMk.
       .cooc,.    .,coo:.",
    }
}

pub fn tux() -> Logo {
    Logo {
        name: "linux",
        art: "        a8888b.
       d888888b.
       8P\"YP\"Y88
       8|o||o|88
       8'    .88
       8`._.' Y8.
      d/      `8b.
    .dP   .     Y8b.
   d8:'   \"   `::88b.
  d8\"           `Y88b
 :8P     '       :888
  8a.    :      _a88P
._/\"Yaa_ :    .| 88P|
\\    YP\"      `| 8P  `.
/     \\._____.d|    .'
`--..__)888888P`._.'",
    }
}

pub fn ubuntu() -> Logo {
    Logo {
        name: "ubuntu",
        art: "             .-/+oossssoo+/-.
         `:+ssssssssssssssssss+:`
       -+ssssssssssssssssssyyssss+-
     .ossssssssssssssssssdMMMNysssso.
    /ssssssssssshdmmNNmmyNMMMMhssssss/
   +ssssssssshmydMMMMMMMNddddyssssssss+
  /sssssssshNMMMyhhyyyyhmNMMMNhssssssss/
 .ssssssssdMMMNhsssssssssshNMMMdssssssss.
 +sssshhhyNMMNyssssssssssssyNMMMysssssss+
 ossyNMMMNyMMhsssssssssssssshmmmhssssssso
 ossyNMMMNyMMhsssssssssssssshmmmhssssssso
 +sssshhhyNMMNyssssssssssssyNMMMysssssss+
 .ssssssssdMMMNhsssssssssshNMMMdssssssss.
  /sssssssshNMMMyhhyyyyhdNMMMNhssssssss/
   +sssssssssdmydMMMMMMMMddddyssssssss+
    /ssssssssssshdmNNNNmyNMMMMhssssss/
     .ossssssssssssssssssdMMMNysssso.
       -+sssssssssssssssssyyyssss+-
         `:+ssssssssssssssssss+:`
             .-/+oossssoo+/-.",
    }
}

pub fn arch() -> Logo {
    Logo {
        name: "arch",
        art: "                   -`
                  .o+`
                 `ooo/
                `+oooo:
               `+oooooo:
               -+oooooo+:
             `/:-:++oooo+:
            `/++++/+++++++:
           `/++++++++++++++:
          `/+++ooooooooooooo/`
         ./ooosssso++osssssso+`
        .oossssso-````/ossssss+`
       -osssssso.      :ssssssso.
      :osssssss/        osssso+++.
     /ossssssss/        +ssssooo/-
   `/ossssso+/:-        -:/+osssso+-
  `+sso+:-`                 `.-/+oso:
 `++:.                           `-/+/
 .`                                 `/",
    }
}

pub fn debian() -> Logo {
    Logo {
        name: "debian",
        art: "       _,met$$$$$gg.
    ,g$$$$$$$$$$$$$$$P.
  ,g$$P\"     \"\"\"Y$$.\"$.
 ,$$P'              `$$$.
',$$P       ,ggs.     `$$b:
`d$$'     ,$P\"'   .    $$$
 $$P      d$'     ,    $$P
 $$:      $$.   -    ,d$$'
 $$;      Y$b._   _,d$P'
 Y$$.    `.`\"Y$$$$P\"'
 `$$b      \"-.__
  `Y$$
   `Y$$.
     `$$b.
       `Y$$b.
          `\"Y$b._
              `\"\"\"",
    }
}

pub fn fedora() -> Logo {
    Logo {
        name: "fedora",
        art: "             .',;::::;,'.
         .';:cccccccccccc:;,.
      .;cccccccccccccccccccccc;.
    .:cccccccccccccccccccccccccc:.
   ;ccccccccccccc;.:dddl:.;ccccccc;
  :ccccccccccccc;OWMKOOXMWd;ccccccc:
 :ccccccccccccc;KMMc;cc;xMMc;ccccccc:
 ccccccccccccc;MMM.;cc;;WMW;cccccccc;
 ccccccccccccc;MMM.;cc;;WMW;cccccccc;
 :ccccccccccccc;KMMc;cc;xMMc;ccccccc:
  :ccccccccccccc;OWMKOOXMWd;ccccccc:
   ;ccccccccccccc;.:dddl:.;ccccccc;
    .:cccccccccccccccccccccccccc:.
      .;cccccccccccccccccccccc;.
         .';:cccccccccccc:;,.
             .',;::::;,'.",
    }
}

pub fn moon() -> Logo {
    Logo {
        name: "moon",
        art: "  ██████
█████▒███
██████████
█████████
  ██████",
    }
}

/// Moon as a grid of cells with per-pixel RGB colors — identical to the splash mode moon.
/// Uses the exact same algorithm as boot/moon.rs.
/// Returns rows of (Option<(u8,u8,u8)>, char) — None means empty space.
pub fn moon_grid() -> Vec<Vec<Option<(u8, u8, u8)>>> {
    const MOON_LIGHT: (f32, f32, f32) = (200.0, 200.0, 190.0);
    const MOON_CRATER: (f32, f32, f32) = (130.0, 125.0, 115.0);
    const MOON_SHADOW: (f32, f32, f32) = (70.0, 65.0, 60.0);

    // Exact boot mode parameters: canvas 68x23
    let bw: f32 = 68.0;
    let bh: f32 = 23.0;
    let cx = bw * 0.15;
    let cy = bh * 0.26;
    let radius = bh * 0.11;
    let r2 = radius * radius;

    let y_min = (cy - radius - 1.0).max(0.0) as i16;
    let y_max = ((cy + radius + 1.0) as i16).min(bh as i16 - 1);
    let x_min = (cx - radius * 2.0 - 1.0).max(0.0) as i16;
    let x_max = ((cx + radius * 2.0 + 1.0) as i16).min(bw as i16 - 1);

    // Find the leftmost filled column
    let mut global_min_x = x_max;
    for iy in y_min..=y_max {
        for ix in x_min..=x_max {
            let fx = (ix as f32 - cx) / 2.0;
            let fy = iy as f32 - cy;
            if fx * fx + fy * fy <= r2 && ix < global_min_x {
                global_min_x = ix;
            }
        }
    }

    let mut rows = Vec::new();
    for iy in y_min..=y_max {
        let fy = iy as f32 - cy;
        let mut row = Vec::new();
        let mut has_pixel = false;

        for ix in global_min_x..=x_max {
            let fx = (ix as f32 - cx) / 2.0;
            let dist2 = fx * fx + fy * fy;

            if dist2 <= r2 {
                has_pixel = true;
                let light = 1.0 - ((fx + fy * 0.3) / radius * 0.4 + 0.1).max(0.0).min(1.0) * 0.7;
                let is_crater = moon_crater(fx, fy, radius);

                let base = if light < 0.4 {
                    MOON_SHADOW
                } else if is_crater {
                    MOON_CRATER
                } else {
                    MOON_LIGHT
                };

                let r = (base.0 * light).min(255.0) as u8;
                let g = (base.1 * light).min(255.0) as u8;
                let b = (base.2 * light).min(255.0) as u8;

                row.push(Some((r, g, b)));
            } else {
                row.push(None);
            }
        }

        if has_pixel {
            // Trim trailing empty cells
            while row.last() == Some(&None) {
                row.pop();
            }
            rows.push(row);
        }
    }

    rows
}

fn moon_crater(fx: f32, fy: f32, radius: f32) -> bool {
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

#[allow(dead_code)]
pub fn generic() -> Logo {
    Logo {
        name: "generic",
        art: "   ___  ___
  |   \\/   |
  |        |
  |  /\\/\\  |
  |_/    \\_|",
    }
}

#[allow(dead_code)]
pub fn available() -> &'static [&'static str] {
    &["apple", "linux", "ubuntu", "arch", "debian", "fedora", "moon", "none"]
}
