//! Sidebar layout component

use crate::models::ItemType;
use crate::state::AppState;
use ratatui::{
    layout::Rect,
    style::Style,
    text::Line,
    widgets::{Block, Borders, List, ListItem},
    Frame,
};

/// Sidebar widget
pub struct Sidebar;

impl Sidebar {
    /// Render the sidebar
    pub fn render(f: &mut Frame, area: Rect, state: &AppState) {
        let items = [
            ListItem::new(Line::from(vec![
                ratatui::text::Span::raw(if state.current_item_type == ItemType::LoginItem {
                    "→ "
                } else {
                    "  "
                }),
                ratatui::text::Span::styled(
                    ItemType::LoginItem.display_name(),
                    if state.current_item_type == ItemType::LoginItem {
                        Style::default().fg(ratatui::style::Color::Cyan).add_modifier(ratatui::style::Modifier::BOLD)
                    } else {
                        Style::default().fg(ratatui::style::Color::White)
                    },
                ),
            ])),
            ListItem::new(Line::from(vec![
                ratatui::text::Span::raw(if state.current_item_type == ItemType::LaunchAgent {
                    "→ "
                } else {
                    "  "
                }),
                ratatui::text::Span::styled(
                    ItemType::LaunchAgent.display_name(),
                    if state.current_item_type == ItemType::LaunchAgent {
                        Style::default().fg(ratatui::style::Color::Cyan).add_modifier(ratatui::style::Modifier::BOLD)
                    } else {
                        Style::default().fg(ratatui::style::Color::White)
                    },
                ),
            ])),
            ListItem::new(Line::from(vec![
                ratatui::text::Span::raw(if state.current_item_type == ItemType::LaunchDaemon {
                    "→ "
                } else {
                    "  "
                }),
                ratatui::text::Span::styled(
                    ItemType::LaunchDaemon.display_name(),
                    if state.current_item_type == ItemType::LaunchDaemon {
                        Style::default().fg(ratatui::style::Color::Cyan).add_modifier(ratatui::style::Modifier::BOLD)
                    } else {
                        Style::default().fg(ratatui::style::Color::White)
                    },
                ),
            ])),
            ListItem::new(Line::from(vec![
                ratatui::text::Span::raw(if state.current_item_type == ItemType::SystemExtension {
                    "→ "
                } else {
                    "  "
                }),
                ratatui::text::Span::styled(
                    ItemType::SystemExtension.display_name(),
                    if state.current_item_type == ItemType::SystemExtension {
                        Style::default().fg(ratatui::style::Color::Cyan).add_modifier(ratatui::style::Modifier::BOLD)
                    } else {
                        Style::default().fg(ratatui::style::Color::White)
                    },
                ),
            ])),
        ];

        let list = List::new(items)
            .block(
                Block::default()
                    .title(" Navigation ")
                    .borders(Borders::ALL)
                    .border_style(Style::default().fg(ratatui::style::Color::White)),
            )
            .style(Style::default().bg(ratatui::style::Color::DarkGray));

        f.render_widget(list, area);
    }

    /// Get the selected index based on current item type
    pub fn selected_index(state: &AppState) -> usize {
        match state.current_item_type {
            ItemType::LoginItem => 0,
            ItemType::OpenAtLogin => 1,
            ItemType::LaunchAgent => 2,
            ItemType::LaunchDaemon => 3,
            ItemType::SystemExtension => 4,
            ItemType::BackgroundItem => 5,
        }
    }
}