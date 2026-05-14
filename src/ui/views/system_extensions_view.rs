//! System extensions view

use crate::models::{ExtensionStatus, SystemExtension};
use crate::state::LoadingState;
use ratatui::{
    layout::{Constraint, Rect},
    style::{Color, Style},
    text::{Line, Span},
    widgets::{Block, Borders, Paragraph, Row, Table},
    Frame,
};

/// View for displaying system extensions
pub struct SystemExtensionsView;

impl SystemExtensionsView {
    /// Render the system extensions view
    pub fn render(f: &mut Frame, area: Rect, items: &[SystemExtension], loading: &LoadingState, selected_index: usize, search_query: &str) {
        match loading {
            LoadingState::Loading => {
                let text = Line::from(vec![
                    Span::raw("  "),
                    Span::styled("Loading system extensions...", Style::default().fg(Color::Yellow)),
                ]);
                let para = Paragraph::new(text)
                    .block(Block::default().title(" System Extensions ").borders(Borders::ALL));
                f.render_widget(para, area);
            }
            LoadingState::Error(msg) => {
                let text = Line::from(vec![
                    Span::styled("X ", Style::default().fg(Color::Red)),
                    Span::raw(msg),
                ]);
                let para = Paragraph::new(text)
                    .block(Block::default().title(" System Extensions - Error ").borders(Borders::ALL))
                    .style(Style::default().bg(Color::DarkGray));
                f.render_widget(para, area);
            }
            _ => {
                let filtered: Vec<&SystemExtension> = if search_query.is_empty() {
                    items.iter().collect()
                } else {
                    items.iter()
                        .filter(|e| e.identifier.to_lowercase().contains(&search_query.to_lowercase()))
                        .collect()
                };

                if filtered.is_empty() {
                    let text = if search_query.is_empty() {
                        Line::from("  No system extensions found.")
                    } else {
                        Line::from(format!("  No items matching '{}'.", search_query))
                    };
                    let para = Paragraph::new(text)
                        .block(Block::default().title(" System Extensions ").borders(Borders::ALL))
                        .style(Style::default().bg(Color::DarkGray));
                    f.render_widget(para, area);
                    return;
                }

                let header = Row::new(vec!["", "Identifier", "Version", "Type"])
                    .style(Style::default().fg(Color::Yellow).add_modifier(ratatui::style::Modifier::BOLD));

                let rows: Vec<Row> = filtered.iter().enumerate().map(|(i, ext)| {
                    let (status_symbol, _) = match ext.status {
                        ExtensionStatus::Activated => ("◉", Color::Green),
                        ExtensionStatus::Deactivated => ("◎", Color::Red),
                        ExtensionStatus::Pending => ("◌", Color::Yellow),
                        ExtensionStatus::Failed => ("X", Color::Red),
                        ExtensionStatus::Unknown => ("?", Color::Gray),
                    };
                    let is_selected = i == selected_index;
                    let type_str = ext.extension_types.first()
                        .map(|t| t.to_string())
                        .unwrap_or_else(|| "Unknown".to_string());

                    Row::new(vec![
                        status_symbol.to_string(),
                        ext.identifier.clone(),
                        ext.version.clone(),
                        type_str,
                    ])
                    .style(if is_selected {
                        Style::default().bg(Color::Blue).fg(Color::White)
                    } else {
                        Style::default()
                    })
                }).collect();

                let table = Table::new(rows, &[
                    Constraint::Length(2),
                    Constraint::Percentage(50),
                    Constraint::Length(12),
                    Constraint::Length(20),
                ])
                    .block(Block::default().title(" System Extensions (Admin Required) ").borders(Borders::ALL))
                    .header(header);

                f.render_widget(table, area);
            }
        }
    }
}