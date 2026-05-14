//! Login items view

use crate::models::LoginItem;
use crate::state::LoadingState;
use ratatui::{
    layout::{Constraint, Rect},
    style::{Color, Style},
    text::{Line, Span},
    widgets::{Block, Borders, Paragraph, Row, Table},
    Frame,
};

/// View for displaying login items
pub struct LoginItemsView;

impl LoginItemsView {
    /// Render the login items view
    pub fn render(f: &mut Frame, area: Rect, items: &[LoginItem], loading: &LoadingState, selected_index: usize, search_query: &str) {
        match loading {
            LoadingState::Loading => {
                let text = Line::from(vec![
                    Span::raw("  "),
                    Span::styled("Loading login items...", Style::default().fg(Color::Yellow)),
                ]);
                let para = Paragraph::new(text)
                    .block(Block::default().title(" Login Items ").borders(Borders::ALL));
                f.render_widget(para, area);
            }
            LoadingState::Error(msg) => {
                let text = Line::from(vec![
                    Span::styled("X ", Style::default().fg(Color::Red)),
                    Span::raw(msg),
                ]);
                let para = Paragraph::new(text)
                    .block(Block::default().title(" Login Items - Error ").borders(Borders::ALL))
                    .style(Style::default().bg(Color::DarkGray));
                f.render_widget(para, area);
            }
            _ => {
                let filtered: Vec<&LoginItem> = if search_query.is_empty() {
                    items.iter().collect()
                } else {
                    items.iter()
                        .filter(|i| i.name.to_lowercase().contains(&search_query.to_lowercase()))
                        .collect()
                };

                if filtered.is_empty() {
                    let text = if search_query.is_empty() {
                        Line::from("  No login items found.")
                    } else {
                        Line::from(format!("  No items matching '{}'.", search_query))
                    };
                    let para = Paragraph::new(text)
                        .block(Block::default().title(" Login Items ").borders(Borders::ALL))
                        .style(Style::default().bg(Color::DarkGray));
                    f.render_widget(para, area);
                    return;
                }

                let header = Row::new(vec!["", "Name", "Path"])
                    .style(Style::default().fg(Color::Yellow).add_modifier(ratatui::style::Modifier::BOLD));

                let rows: Vec<Row> = filtered.iter().enumerate().map(|(i, item)| {
                    let status = if item.enabled { "●" } else { "○" };
                    let _status_color = if item.enabled { Color::Green } else { Color::Red };
                    let is_selected = i == selected_index;

                    Row::new(vec![
                        Span::raw(status).to_string(),
                        if is_selected {
                            format!(" {} ", item.name)
                        } else {
                            format!(" {}", item.name)
                        },
                        item.path.display().to_string(),
                    ])
                    .style(if is_selected {
                        Style::default().bg(Color::Blue).fg(Color::White)
                    } else {
                        Style::default()
                    })
                }).collect();

                let table = Table::new(rows, &[Constraint::Length(3), Constraint::Percentage(40), Constraint::Percentage(57)])
                    .block(Block::default().title(" Login Items ").borders(Borders::ALL))
                    .header(header);

                f.render_widget(table, area);
            }
        }
    }
}