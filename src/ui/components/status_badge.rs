//! Status badge component

use ratatui::{text::Span, style::Color};

/// Status badge styles
#[derive(Debug, Clone, Copy)]
pub enum BadgeStyle {
    Enabled,
    Disabled,
    Loaded,
    Unloaded,
    Activated,
    Deactivated,
    Pending,
    Failed,
}

impl BadgeStyle {
    /// Get the text for this badge style
    pub fn text(&self) -> &'static str {
        match self {
            BadgeStyle::Enabled => "●",
            BadgeStyle::Disabled => "○",
            BadgeStyle::Loaded => "▶",
            BadgeStyle::Unloaded => "■",
            BadgeStyle::Activated => "◉",
            BadgeStyle::Deactivated => "◎",
            BadgeStyle::Pending => "◌",
            BadgeStyle::Failed => "✗",
        }
    }

    /// Get the color for this badge style
    pub fn color(&self) -> Color {
        match self {
            BadgeStyle::Enabled | BadgeStyle::Loaded | BadgeStyle::Activated => Color::Green,
            BadgeStyle::Disabled | BadgeStyle::Unloaded | BadgeStyle::Deactivated => Color::Red,
            BadgeStyle::Pending => Color::Yellow,
            BadgeStyle::Failed => Color::Red,
        }
    }
}

/// Create a status badge span
pub fn status_badge(style: BadgeStyle) -> Span<'static> {
    Span::styled(
        format!(" {} ", style.text()),
        ratatui::style::Style::default()
            .fg(style.color())
            .add_modifier(ratatui::style::Modifier::BOLD),
    )
}

/// Create a simple status indicator with label
pub fn status_indicator(label: &str, is_active: bool) -> Span<'static> {
    let style = if is_active { BadgeStyle::Enabled } else { BadgeStyle::Disabled };
    Span::styled(
        format!("{} {}", style.text(), label),
        ratatui::style::Style::default()
            .fg(style.color())
            .add_modifier(ratatui::style::Modifier::BOLD),
    )
}

/// Create a compact status badge
pub fn compact(is_enabled: bool) -> Span<'static> {
    let style = if is_enabled { BadgeStyle::Enabled } else { BadgeStyle::Disabled };
    Span::styled(style.text(), ratatui::style::Style::default().fg(style.color()))
}