use blaeck::prelude::*;
use blaeck::Blaeck;
use std::io;

use crate::color::Theme;
use crate::info::SystemInfo;

pub fn render(info: &SystemInfo, logo: &str, theme: &Theme) -> io::Result<()> {
    let title = info.title();
    let separator = "-".repeat(title.len());
    let fields = info.fields();

    let mut info_elements: Vec<Element> = Vec::new();

    // Title
    info_elements.push(element! {
        Text(content: title, color: theme.title, bold: true)
    });
    info_elements.push(element! {
        Text(content: separator, color: theme.separator)
    });

    // Info fields
    for (label, value) in &fields {
        info_elements.push(element! {
            Box(flex_direction: FlexDirection::Row) {
                Text(content: format!("{}: ", label), color: theme.label, bold: true)
                Text(content: value.to_string())
            }
        });
    }

    // Color palette
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

    let ui = Element::node::<Box>(
        BoxProps {
            flex_direction: FlexDirection::Row,
            ..Default::default()
        },
        vec![
            element! {
                Text(content: logo, color: theme.logo)
            },
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
