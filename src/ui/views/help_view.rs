//! Help view

use ratatui::{
    layout::{Alignment, Constraint, Direction, Layout, Rect},
    style::Style,
    widgets::{Block, Borders, Paragraph, Table, Row},
    Frame,
};

/// Help overlay view
pub struct HelpView;

impl HelpView {
    /// Render the help overlay
    pub fn render(f: &mut Frame, area: Rect) {
        let _chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints([
                Constraint::Length(1),
                Constraint::Min(0),
                Constraint::Length(1),
            ])
            .split(area);

        // Title
        let title = Paragraph::new(" Keyboard Shortcuts ")
            .style(Style::default().fg(ratatui::style::Color::Cyan))
            .alignment(Alignment::Center);

        let shortcuts = vec![
            Row::new(vec!["↑ / k", "Move selection up"]),
            Row::new(vec!["↓ / j", "Move selection down"]),
            Row::new(vec!["← / h", "Navigate to sidebar"]),
            Row::new(vec!["→ / l", "Navigate to content"]),
            Row::new(vec!["Enter / Space", "Select / Toggle item"]),
            Row::new(vec!["r", "Refresh current list"]),
            Row::new(vec!["/", "Focus search"]),
            Row::new(vec!["Esc", "Clear search / Go back"]),
            Row::new(vec!["q", "Quit application"]),
            Row::new(vec!["?", "Show / hide this help"]),
        ];

        let table = Table::new(shortcuts, &[Constraint::Length(16), Constraint::Percentage(100)])
            .style(Style::default().fg(ratatui::style::Color::White));

        let block = Block::default()
            .title(" Help - Keyboard Shortcuts ")
            .borders(Borders::ALL)
            .border_style(Style::default().fg(ratatui::style::Color::Cyan));

        let inner = block.inner(area);
        f.render_widget(block, area);

        let content_chunks = Layout::default()
            .direction(Direction::Vertical)
            .margin(1)
            .constraints([
                Constraint::Length(1),
                Constraint::Min(0),
            ])
            .split(inner);

        f.render_widget(title, content_chunks[0]);
        f.render_widget(table, content_chunks[1]);
    }
}