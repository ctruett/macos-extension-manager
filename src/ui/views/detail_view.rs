//! Detail view for selected items.

use ratatui::{
    layout::Rect,
    style::{Color, Style, Stylize},
    text::{Line, Span},
    widgets::{Block, Borders, Paragraph},
};

/// Renders the detail view for a selected item.
pub fn render_detail_view(area: Rect, item_type: crate::models::ItemType, item: Option<&str>) -> Paragraph<'static> {
    match item_type {
        crate::models::ItemType::LoginItem => render_login_item_detail(area, item),
        crate::models::ItemType::LaunchAgent => render_launch_agent_detail(area, item),
        crate::models::ItemType::LaunchDaemon => render_launch_daemon_detail(area, item),
        crate::models::ItemType::SystemExtension => render_system_extension_detail(area, item),
    }
}

/// Renders the detail view for a login item.
fn render_login_item_detail(_area: Rect, item: Option<&str>) -> Paragraph<'static> {
    let content = match item {
        Some(name) => vec![
            Line::from(vec![Span::styled("Login Item Details", Style::new().fg(Color::Blue).bold())]),
            Line::from(vec![Span::raw("")]),
            Line::from(vec![Span::raw(format!("Name: {}", name))]),
            Line::from(vec![Span::raw("")]),
            Line::from(vec![Span::raw("Path: /Applications/example.app")]),
            Line::from(vec![Span::raw("")]),
            Line::from(vec![Span::raw("Enabled: ●")]),
            Line::from(vec![Span::raw("")]),
            Line::from(vec![Span::raw("Actions:")]),
            Line::from(vec![Span::raw("  [e] Enable / Disable")]),
            Line::from(vec![Span::raw("  [d] Remove")]),
        ],
        None => vec![
            Line::from(vec![Span::raw("")]),
            Line::from(vec![Span::raw("Select an item to view details.")]),
            Line::from(vec![Span::raw("")]),
            Line::from(vec![Span::raw("Use ↑/↓ to navigate.")]),
        ],
    };

    Paragraph::new(content)
        .block(
            Block::default()
                .title(" Details ")
                .borders(Borders::ALL),
        )
        .style(Style::new())
}

/// Renders the detail view for a launch agent.
fn render_launch_agent_detail(_area: Rect, item: Option<&str>) -> Paragraph<'static> {
    let content = match item {
        Some(label) => vec![
            Line::from(vec![Span::styled("Launch Agent Details", Style::new().fg(Color::Blue).bold())]),
            Line::from(vec![Span::raw("")]),
            Line::from(vec![Span::raw(format!("Label: {}", label))]),
            Line::from(vec![Span::raw("")]),
            Line::from(vec![Span::raw("Path: ~/Library/LaunchAgents/com.example.agent.plist")]),
            Line::from(vec![Span::raw("")]),
            Line::from(vec![Span::raw("Run At Load: ●")]),
            Line::from(vec![Span::raw("Keep Alive: ○")]),
            Line::from(vec![Span::raw("")]),
            Line::from(vec![Span::raw("Actions:")]),
            Line::from(vec![Span::raw("  [l] Load / Unload")]),
            Line::from(vec![Span::raw("  [d] Delete plist")]),
        ],
        None => vec![
            Line::from(vec![Span::raw("")]),
            Line::from(vec![Span::raw("Select a launch agent to view details.")]),
        ],
    };

    Paragraph::new(content)
        .block(
            Block::default()
                .title(" Details ")
                .borders(Borders::ALL),
        )
        .style(Style::new())
}

/// Renders the detail view for a launch daemon.
fn render_launch_daemon_detail(_area: Rect, item: Option<&str>) -> Paragraph<'static> {
    let content = match item {
        Some(label) => vec![
            Line::from(vec![Span::styled("Launch Daemon Details", Style::new().fg(Color::Blue).bold())]),
            Line::from(vec![Span::raw("")]),
            Line::from(vec![Span::raw(format!("Label: {}", label))]),
            Line::from(vec![Span::raw("")]),
            Line::from(vec![Span::raw("Path: /Library/LaunchDaemons/com.example.daemon.plist")]),
            Line::from(vec![Span::raw("")]),
            Line::from(vec![Span::styled("⚠ Requires Admin Privileges", Style::new().fg(Color::Yellow))]),
            Line::from(vec![Span::raw("")]),
            Line::from(vec![Span::raw("Actions:")]),
            Line::from(vec![Span::raw("  [l] Load / Unload")]),
            Line::from(vec![Span::raw("  [d] Delete (requires admin)")]),
        ],
        None => vec![
            Line::from(vec![Span::raw("")]),
            Line::from(vec![Span::raw("Select a launch daemon to view details.")]),
        ],
    };

    Paragraph::new(content)
        .block(
            Block::default()
                .title(" Details ")
                .borders(Borders::ALL),
        )
        .style(Style::new())
}

/// Renders the detail view for a system extension.
fn render_system_extension_detail(_area: Rect, item: Option<&str>) -> Paragraph<'static> {
    let content = match item {
        Some(id) => vec![
            Line::from(vec![Span::styled("System Extension Details", Style::new().fg(Color::Blue).bold())]),
            Line::from(vec![Span::raw("")]),
            Line::from(vec![Span::raw(format!("Identifier: {}", id))]),
            Line::from(vec![Span::raw("")]),
            Line::from(vec![Span::raw("Version: 1.0.0")]),
            Line::from(vec![Span::raw("Type: Network Extension")]),
            Line::from(vec![Span::raw("Status: ● Activated")]),
            Line::from(vec![Span::raw("")]),
            Line::from(vec![Span::styled("⚠ Requires Admin Privileges", Style::new().fg(Color::Yellow))]),
            Line::from(vec![Span::raw("")]),
            Line::from(vec![Span::raw("Actions:")]),
            Line::from(vec![Span::raw("  [a] Activate / Deactivate")]),
        ],
        None => vec![
            Line::from(vec![Span::raw("")]),
            Line::from(vec![Span::raw("Select a system extension to view details.")]),
        ],
    };

    Paragraph::new(content)
        .block(
            Block::default()
                .title(" Details ")
                .borders(Borders::ALL),
        )
        .style(Style::new())
}

/// Renders the help overlay.
pub fn render_help(area: Rect) -> Paragraph<'static> {
    let content = vec![
        Line::from(vec![Span::styled("Keyboard Shortcuts", Style::new().fg(Color::Blue).bold())]),
        Line::from(vec![Span::raw("")]),
        Line::from(vec![Span::raw("↑ / k     Move selection up")]),
        Line::from(vec![Span::raw("↓ / j     Move selection down")]),
        Line::from(vec![Span::raw("← / h     Navigate to sidebar")]),
        Line::from(vec![Span::raw("→ / l     Navigate to detail")]),
        Line::from(vec![Span::raw("Enter    Select / Toggle")]),
        Line::from(vec![Span::raw("r        Refresh current list")]),
        Line::from(vec![Span::raw("/        Focus search")]),
        Line::from(vec![Span::raw("Esc      Clear search / Go back")]),
        Line::from(vec![Span::raw("q        Quit application")]),
        Line::from(vec![Span::raw("?        Show help")]),
        Line::from(vec![Span::raw("")]),
        Line::from(vec![Span::raw("Press any key to close.")]),
    ];

    Paragraph::new(content)
        .block(
            Block::default()
                .title(" Help ")
                .borders(Borders::ALL)
                .border_style(Style::new().fg(Color::Blue)),
        )
        .style(Style::new())
}