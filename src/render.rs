use blaeck::prelude::*;
use blaeck::Blaeck;
use std::io;

use crate::animate;
use crate::color::{self, Theme};
use crate::config::Config;
use crate::info::SystemInfo;
use crate::logo;
use crate::mode::Mode;

/// Build the moon as a blaeck Column of Rows with per-cell RGB colors.
fn moon_element() -> Element {
    let grid = logo::moon_grid();
    let mut rows: Vec<Element> = Vec::new();

    for row in &grid {
        let mut cells: Vec<Element> = Vec::new();
        // Group consecutive same-type cells (filled vs space) for efficiency
        let mut i = 0;
        while i < row.len() {
            if let Some((r, g, b)) = row[i] {
                // Collect consecutive filled cells with same color
                cells.push(element! {
                    Text(content: "█", color: Color::Rgb(r, g, b))
                });
                i += 1;
            } else {
                // Collect consecutive spaces
                let mut spaces = String::new();
                while i < row.len() && row[i].is_none() {
                    spaces.push(' ');
                    i += 1;
                }
                cells.push(element! {
                    Text(content: spaces)
                });
            }
        }

        rows.push(Element::node::<Box>(
            BoxProps {
                flex_direction: FlexDirection::Row,
                ..Default::default()
            },
            cells,
        ));
    }

    Element::node::<Box>(
        BoxProps {
            flex_direction: FlexDirection::Column,
            ..Default::default()
        },
        rows,
    )
}

pub fn render(info: &SystemInfo, logo: &str, theme: &Theme, cfg: &Config, mode: Mode) -> io::Result<()> {
    let title = info.title();
    let title_len = title.len();
    let all_fields = info.fields();
    let active = cfg.active_fields_for_mode(mode);

    let mut info_elements: Vec<Element> = Vec::new();

    // Title
    info_elements.push(element! {
        Text(content: title, color: theme.title, bold: true)
    });

    // Separator — use blaeck Divider
    info_elements.push(Element::node::<Divider>(
        DividerProps::new()
            .width(title_len)
            .line_style(DividerStyle::Dashed)
            .color(theme.separator),
        vec![],
    ));

    // Info fields — filtered and ordered by config
    let right_align_labels = cfg.label_align.as_deref() == Some("right");
    let label_on_right = cfg.label_position.as_deref() == Some("right");
    let right_align_values = cfg.value_align.as_deref() == Some("right");
    let field_sep = cfg.field_separator.as_deref().unwrap_or(": ");
    let use_fill = field_sep == "fill";

    // Collect active field data
    let mut field_data: Vec<(String, String)> = Vec::new();
    for key in &active {
        if let Some((_, value)) = all_fields.iter().find(|(k, _)| *k == key.as_str()) {
            field_data.push((cfg.label_for(key), value.to_string()));
        }
    }

    // Calculate max widths for alignment
    let max_label_width = field_data.iter().map(|(l, _)| l.len()).max().unwrap_or(0);
    let max_value_width = if right_align_values {
        field_data.iter().map(|(_, v)| v.len()).max().unwrap_or(0)
    } else {
        0
    };

    // Total field width for fill separator
    let fill_total = max_label_width + max_value_width + 6; // padding for fill line

    for (label, value) in &field_data {
        let formatted_value = if right_align_values {
            format!("{:>width$}", value, width = max_value_width)
        } else {
            value.clone()
        };

        if label_on_right {
            // Value first, then label: "MacOS 15.5  arm64   OS"
            let formatted_label = if right_align_labels {
                format!("{:>width$}", label, width = max_label_width)
            } else {
                label.clone()
            };
            let sep = if use_fill {
                let used = formatted_value.len() + formatted_label.len() + 2;
                let fill_len = fill_total.saturating_sub(used);
                format!(" {} ", "─".repeat(fill_len))
            } else {
                format!(" {}", field_sep)
            };
            info_elements.push(element! {
                Box(flex_direction: FlexDirection::Row) {
                    Text(content: formatted_value)
                    Text(content: sep, color: theme.separator)
                    Text(content: formatted_label, color: theme.label, bold: true)
                }
            });
        } else if use_fill {
            // Label with fill line to value: "OS ──────── MacOS 15.5"
            let formatted_label = if right_align_labels {
                format!("{:>width$}", label, width = max_label_width)
            } else {
                label.clone()
            };
            let used = formatted_label.len() + formatted_value.len() + 2;
            let fill_len = fill_total.saturating_sub(used);
            let fill = format!(" {} ", "─".repeat(fill_len));
            info_elements.push(element! {
                Box(flex_direction: FlexDirection::Row) {
                    Text(content: formatted_label, color: theme.label, bold: true)
                    Text(content: fill, color: theme.separator)
                    Text(content: formatted_value)
                }
            });
        } else {
            // Standard: "OS: MacOS 15.5"
            let formatted_label = if right_align_labels {
                format!("{:>width$}{}", label, field_sep, width = max_label_width)
            } else {
                format!("{}{}", label, field_sep)
            };
            info_elements.push(element! {
                Box(flex_direction: FlexDirection::Row) {
                    Text(content: formatted_label, color: theme.label, bold: true)
                    Text(content: formatted_value)
                }
            });
        }
    }

    // Color palette
    if cfg.show_palette_for_mode(mode) {
        info_elements.push(element! { Text(content: "") });
        info_elements.push(element! {
            Box(flex_direction: FlexDirection::Row) {
                Text(content: "███", color: Color::Black)
                Text(content: "███", color: Color::Red)
                Text(content: "███", color: Color::Green)
                Text(content: "███", color: Color::Yellow)
                Text(content: "███", color: Color::Blue)
                Text(content: "███", color: Color::Magenta)
                Text(content: "███", color: Color::Cyan)
                Text(content: "███", color: Color::White)
            }
        });
        info_elements.push(element! {
            Box(flex_direction: FlexDirection::Row) {
                Text(content: "███", color: Color::DarkGray)
                Text(content: "███", color: Color::LightRed)
                Text(content: "███", color: Color::LightGreen)
                Text(content: "███", color: Color::LightYellow)
                Text(content: "███", color: Color::LightBlue)
                Text(content: "███", color: Color::LightMagenta)
                Text(content: "███", color: Color::LightCyan)
                Text(content: "███", color: Color::White)
            }
        });
    }

    // Logo element — use per-cell colored moon for default mode, otherwise text
    let use_moon = mode == Mode::Default && logo.is_empty();
    let logo_element = if use_moon {
        moon_element()
    } else {
        element! { Text(content: logo, color: theme.logo) }
    };

    let ui = Element::node::<Box>(
        BoxProps {
            flex_direction: FlexDirection::Row,
            align_items: Some(AlignItems::Center),
            ..Default::default()
        },
        vec![
            logo_element,
            element! {
                Text(content: "   ")
            },
            Element::node::<Box>(
                BoxProps {
                    flex_direction: FlexDirection::Column,
                    ..Default::default()
                },
                info_elements,
            ),
        ],
    );

    let mut blaeck = Blaeck::new(io::stdout())?;
    blaeck.render(ui)?;
    blaeck.unmount()?;

    Ok(())
}

pub fn render_animated(info: &SystemInfo, logo: &str, theme: &Theme, cfg: &Config, mode: Mode) -> io::Result<()> {
    // Normal render first
    render(info, logo, theme, cfg, mode)?;

    if logo.is_empty() {
        return Ok(());
    }

    // Calculate geometry
    let logo_lines = logo.lines().count();
    let info_lines = {
        let active = cfg.active_fields_for_mode(mode);
        let mut count = 2; // title + separator
        count += active.len();
        if cfg.show_palette_for_mode(mode) {
            count += 3; // blank + 2 palette rows
        }
        count
    };
    let total_height = std::cmp::max(logo_lines, info_lines) as u16;

    let original_color = color::color_to_ansi(&theme.logo);

    // Animate in foreground — blocks until keypress, then exits
    animate::run_foreground(logo, total_height, original_color);

    Ok(())
}
