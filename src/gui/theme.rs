use iced::font::Family;
use iced::theme::Palette;
use iced::widget::{button, container, pick_list, rule, text, text_input, Text};
use iced::{Background, Border, Color, Font, Theme};

pub const BG: Color = Color { r: 0.200, g: 0.180, b: 0.160, a: 1.0 };
pub const SURFACE: Color = Color { r: 0.250, g: 0.230, b: 0.210, a: 1.0 };
pub const BORDER_COLOR: Color = Color { r: 0.350, g: 0.320, b: 0.290, a: 1.0 };
pub const TEXT_COLOR: Color = Color { r: 0.900, g: 0.880, b: 0.850, a: 1.0 };
pub const TEXT_SECONDARY: Color = Color { r: 0.620, g: 0.580, b: 0.530, a: 1.0 };
pub const ACCENT: Color = Color { r: 0.780, g: 0.530, b: 0.280, a: 1.0 };
pub const ACCENT_LIGHT: Color = Color { r: 0.280, g: 0.255, b: 0.230, a: 1.0 };

pub fn ayus_theme() -> Theme {
    Theme::custom(
        "Ayus",
        Palette {
            background: BG,
            text: TEXT_COLOR,
            primary: ACCENT,
            success: Color { r: 0.450, g: 0.850, b: 0.550, a: 1.0 },
            warning: Color { r: 0.718, g: 0.494, b: 0.200, a: 1.0 },
            danger: Color { r: 0.950, g: 0.450, b: 0.450, a: 1.0 },
        },
    )
}

pub fn latin() -> Font {
    Font {
        family: Family::Name("Noto Sans"),
        ..Font::DEFAULT
    }
}

pub fn devanagari() -> Font {
    Font {
        family: Family::Name("Noto Sans Devanagari"),
        ..Font::DEFAULT
    }
}

pub fn accent_btn(_theme: &Theme, status: button::Status) -> button::Style {
    let bg = match status {
        button::Status::Hovered | button::Status::Pressed => Color {
            r: ACCENT.r * 0.85,
            g: ACCENT.g * 0.85,
            b: ACCENT.b * 0.85,
            a: 1.0,
        },
        button::Status::Disabled => Color {
            a: 0.5,
            ..ACCENT
        },
        _ => ACCENT,
    };
    button::Style {
        background: Some(Background::Color(bg)),
        text_color: Color::WHITE,
        border: Border {
            radius: 4.0.into(),
            ..Border::default()
        },
        ..button::Style::default()
    }
}

pub fn tab_active(_theme: &Theme, _status: button::Status) -> button::Style {
    button::Style {
        background: Some(Background::Color(ACCENT)),
        text_color: Color::WHITE,
        border: Border {
            radius: 4.0.into(),
            ..Border::default()
        },
        ..button::Style::default()
    }
}

pub fn tab_inactive(_theme: &Theme, status: button::Status) -> button::Style {
    button::Style {
        background: match status {
            button::Status::Hovered => Some(Background::Color(ACCENT_LIGHT)),
            _ => None,
        },
        text_color: match status {
            button::Status::Hovered => TEXT_COLOR,
            _ => TEXT_SECONDARY,
        },
        border: Border {
            radius: 4.0.into(),
            ..Border::default()
        },
        ..button::Style::default()
    }
}

pub fn card(_theme: &Theme) -> container::Style {
    container::Style {
        background: Some(Background::Color(SURFACE)),
        border: Border {
            color: BORDER_COLOR,
            width: 1.0,
            radius: 4.0.into(),
        },
        ..container::Style::default()
    }
}

pub fn tab_rule(_theme: &Theme) -> rule::Style {
    rule::Style {
        color: BORDER_COLOR,
        radius: 0.0.into(),
        fill_mode: rule::FillMode::Full,
        snap: true,
    }
}

pub fn input_style(theme: &Theme, status: text_input::Status) -> text_input::Style {
    let palette = theme.extended_palette();
    let active = text_input::Style {
        background: Background::Color(SURFACE),
        border: Border {
            color: BORDER_COLOR,
            width: 1.0,
            radius: 4.0.into(),
        },
        icon: TEXT_SECONDARY,
        placeholder: TEXT_SECONDARY,
        value: TEXT_COLOR,
        selection: palette.primary.weak.color,
    };
    match status {
        text_input::Status::Active => active,
        text_input::Status::Hovered => text_input::Style {
            border: Border {
                color: ACCENT,
                ..active.border
            },
            ..active
        },
        text_input::Status::Focused { .. } => text_input::Style {
            border: Border {
                color: ACCENT,
                width: 2.0,
                ..active.border
            },
            ..active
        },
        text_input::Status::Disabled => text_input::Style {
            background: Background::Color(BG),
            value: TEXT_SECONDARY,
            ..active
        },
    }
}

pub fn label(s: &str) -> Text<'_> {
    text(s.to_uppercase())
        .size(11)
        .font(latin())
        .color(TEXT_SECONDARY)
}

pub fn domain_pick_list(_theme: &Theme, status: pick_list::Status) -> pick_list::Style {
    let base = pick_list::Style {
        text_color: TEXT_COLOR,
        placeholder_color: TEXT_SECONDARY,
        handle_color: TEXT_SECONDARY,
        background: Background::Color(SURFACE),
        border: Border {
            color: BORDER_COLOR,
            width: 1.0,
            radius: 4.0.into(),
        },
    };
    match status {
        pick_list::Status::Hovered | pick_list::Status::Opened { .. } => pick_list::Style {
            border: Border {
                color: ACCENT,
                ..base.border
            },
            ..base
        },
        _ => base,
    }
}
