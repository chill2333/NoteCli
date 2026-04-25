#[derive(Debug, Clone, Copy)]
pub enum Color {
    Black,
    Red,
    Green,
    Yellow,
    Blue,
    Magenta,
    Cyan,
    White,
    BrightBlack,
    BrightRed,
    BrightGreen,
    BrightYellow,
    BrightBlue,
    BrightMagenta,
    BrightCyan,
    BrightWhite,
}

#[derive(Debug, Clone, Default)]
pub struct Style {
    pub fg: Option<Color>,
    pub bold: bool,
    pub underline: bool,
}

impl Style {
    pub fn parse(s: &str) -> Self {
        let mut style = Self::default();
        for part in s.split_whitespace() {
            match part {
                "bold" => style.bold = true,
                "underline" => style.underline = true,
                "black" => style.fg = Some(Color::Black),
                "red" => style.fg = Some(Color::Red),
                "green" => style.fg = Some(Color::Green),
                "yellow" => style.fg = Some(Color::Yellow),
                "blue" => style.fg = Some(Color::Blue),
                "magenta" => style.fg = Some(Color::Magenta),
                "cyan" => style.fg = Some(Color::Cyan),
                "white" => style.fg = Some(Color::White),
                "bright_black" | "gray" | "dark_gray" => style.fg = Some(Color::BrightBlack),
                "bright_red" => style.fg = Some(Color::BrightRed),
                "bright_green" => style.fg = Some(Color::BrightGreen),
                "bright_yellow" => style.fg = Some(Color::BrightYellow),
                "bright_blue" => style.fg = Some(Color::BrightBlue),
                "bright_magenta" => style.fg = Some(Color::BrightMagenta),
                "bright_cyan" => style.fg = Some(Color::BrightCyan),
                "bright_white" => style.fg = Some(Color::BrightWhite),
                _ => {}
            }
        }
        style
    }
}

pub struct Theme {
    pub title: Style,
    pub id: Style,
    pub tag: Style,
    pub category: Style,
    pub priority_low: Style,
    pub priority_normal: Style,
    pub priority_high: Style,
    pub priority_urgent: Style,
    pub separator: Style,
    pub date: Style,
    pub no_color: bool,
}

impl Default for Theme {
    fn default() -> Self {
        Self {
            title: Style::parse("cyan bold"),
            id: Style::parse("yellow"),
            tag: Style::parse("green"),
            category: Style::parse("blue"),
            priority_low: Style::parse("white"),
            priority_normal: Style::parse("green"),
            priority_high: Style::parse("yellow bold"),
            priority_urgent: Style::parse("red bold"),
            separator: Style::parse("dark_gray"),
            date: Style::parse("dark_gray"),
            no_color: false,
        }
    }
}

impl Theme {
    pub fn from_config(cfg: &super::config::ThemeConfig, no_color: bool) -> Self {
        Self {
            title: Style::parse(&cfg.title),
            id: Style::parse(&cfg.id),
            tag: Style::parse(&cfg.tag),
            category: Style::parse(&cfg.category),
            priority_low: Style::parse(&cfg.priority_low),
            priority_normal: Style::parse(&cfg.priority_normal),
            priority_high: Style::parse(&cfg.priority_high),
            priority_urgent: Style::parse(&cfg.priority_urgent),
            separator: Style::parse(&cfg.separator),
            date: Style::parse(&cfg.date),
            no_color,
        }
    }

}

impl Clone for Theme {
    fn clone(&self) -> Self {
        Self {
            title: self.title.clone(),
            id: self.id.clone(),
            tag: self.tag.clone(),
            category: self.category.clone(),
            priority_low: self.priority_low.clone(),
            priority_normal: self.priority_normal.clone(),
            priority_high: self.priority_high.clone(),
            priority_urgent: self.priority_urgent.clone(),
            separator: self.separator.clone(),
            date: self.date.clone(),
            no_color: self.no_color,
        }
    }
}
