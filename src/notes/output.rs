use std::fmt::Display;

use colored::Colorize;
use comfy_table::{Cell, Table, ContentArrangement, Attribute, Color as TColor};

use super::theme::{Theme, Style, Color};

pub struct Output {
    theme: Theme,
}

impl Output {
    pub fn new(theme: Theme) -> Self {
        Self { theme }
    }

    pub fn theme(&self) -> &Theme {
        &self.theme
    }

    // --- Messages ---

    pub fn success(&self, msg: impl Display) {
        println!("{}", msg);
    }

    pub fn error(&self, msg: impl Display) {
        if self.theme.no_color {
            eprintln!("错误: {}", msg);
        } else {
            eprintln!("{} {}", "错误:".red().bold(), msg);
        }
    }

    pub fn warn(&self, msg: impl Display) {
        if self.theme.no_color {
            eprintln!("警告: {}", msg);
        } else {
            eprintln!("{} {}", "警告:".yellow().bold(), msg);
        }
    }

    pub fn hint(&self, msg: impl Display) {
        eprintln!("提示: {}", msg);
    }

    pub fn info(&self, msg: impl Display) {
        eprintln!("{}", msg);
    }

    pub fn empty(&self, msg: impl Display) {
        println!("{}", msg);
    }

    pub fn line(&self, msg: impl Display) {
        println!("{}", msg);
    }

    pub fn blank(&self) {
        println!();
    }

    // --- Table ---

    pub fn create_table(&self) -> Table {
        let mut table = Table::new();
        table.set_content_arrangement(ContentArrangement::Dynamic);
        table
    }

    pub fn set_headers(&self, table: &mut Table, headers: &[&str]) {
        table.set_header(
            headers.iter().map(|&h| Cell::new(h).add_attribute(Attribute::Bold)).collect::<Vec<_>>()
        );
    }

    pub fn print_table(&self, table: &Table) {
        println!("{table}");
    }

    // --- Themed cells ---

    fn to_tcolor(style: &Style) -> TColor {
        if style.fg.is_none() {
            return TColor::Reset;
        }
        match style.fg.unwrap() {
            Color::Black => TColor::Black,
            Color::Red => TColor::Red,
            Color::Green => TColor::Green,
            Color::Yellow => TColor::Yellow,
            Color::Blue => TColor::Blue,
            Color::Magenta => TColor::Magenta,
            Color::Cyan => TColor::Cyan,
            Color::White => TColor::White,
            Color::BrightBlack => TColor::DarkGrey,
            Color::BrightRed => TColor::Red,
            Color::BrightGreen => TColor::Green,
            Color::BrightYellow => TColor::Yellow,
            Color::BrightBlue => TColor::Blue,
            Color::BrightMagenta => TColor::Magenta,
            Color::BrightCyan => TColor::Cyan,
            Color::BrightWhite => TColor::Grey,
        }
    }

    fn style_color(&self, style: &Style) -> TColor {
        if self.theme.no_color {
            TColor::Reset
        } else {
            Self::to_tcolor(style)
        }
    }

    pub fn cell_id(&self, id: u32) -> Cell {
        Cell::new(id).fg(self.style_color(&self.theme.id))
    }

    pub fn cell_title(&self, title: &str) -> Cell {
        Cell::new(title).fg(self.style_color(&self.theme.title))
    }

    pub fn cell_category(&self, cat: &str) -> Cell {
        Cell::new(cat).fg(self.style_color(&self.theme.category))
    }

    pub fn cell_priority(&self, pri: &str) -> Cell {
        Cell::new(pri).fg(self.style_color(self.priority_style(pri)))
    }

    pub fn cell_date(&self, date: &str) -> Cell {
        Cell::new(date).fg(self.style_color(&self.theme.date))
    }

    pub fn cell_tag(&self, tags: &str) -> Cell {
        Cell::new(tags).fg(self.style_color(&self.theme.tag))
    }

    // --- Styled text (for detail display) ---

    pub fn styled(&self, text: &str, style: &Style) -> colored::ColoredString {
        if self.theme.no_color {
            return text.normal();
        }
        let mut s = match style.fg {
            Some(c) => text.color(color_to_colored(c)),
            None => text.normal(),
        };
        if style.bold { s = s.bold(); }
        if style.underline { s = s.underline(); }
        s
    }

    pub fn priority_style(&self, pri: &str) -> &Style {
        match pri {
            "low" => &self.theme.priority_low,
            "high" => &self.theme.priority_high,
            "urgent" => &self.theme.priority_urgent,
            _ => &self.theme.priority_normal,
        }
    }
}

fn color_to_colored(c: Color) -> colored::Color {
    match c {
        Color::Black => colored::Color::Black,
        Color::Red => colored::Color::Red,
        Color::Green => colored::Color::Green,
        Color::Yellow => colored::Color::Yellow,
        Color::Blue => colored::Color::Blue,
        Color::Magenta => colored::Color::Magenta,
        Color::Cyan => colored::Color::Cyan,
        Color::White => colored::Color::White,
        Color::BrightBlack => colored::Color::BrightBlack,
        Color::BrightRed => colored::Color::BrightRed,
        Color::BrightGreen => colored::Color::BrightGreen,
        Color::BrightYellow => colored::Color::BrightYellow,
        Color::BrightBlue => colored::Color::BrightBlue,
        Color::BrightMagenta => colored::Color::BrightMagenta,
        Color::BrightCyan => colored::Color::BrightCyan,
        Color::BrightWhite => colored::Color::BrightWhite,
    }
}
