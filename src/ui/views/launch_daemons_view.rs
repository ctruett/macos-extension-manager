//! Launch daemons view

use crate::models::LaunchDaemon;
use crate::state::LoadingState;
use ratatui::{
    layout::{Constraint, Rect},
    style::{Color, Style},
    text::{Line, Span},
    widgets::{Block, Borders, Paragraph, Row, Table},
    Frame,
};

/// View for displaying launch daemons
pub struct LaunchDaemonsView;

impl LaunchDaemonsView {
    /// Render the launch daemons view
    pub fn render(f: &mut Frame, area: Rect, items: &[LaunchDaemon], loading: &LoadingState, selected_index: usize, search_query: &str) {
        match loading {
            LoadingState::Loading => {
                let text = Line::from(vec![
                    Span::raw("  "),
                    Span::styled("Loading launch daemons...", Style::default().fg(Color::Yellow)),
                ]);
                let para = Paragraph::new(text)
                    .block(Block::default().title(" Launch Daemons ").borders(Borders::ALL));
                f.render_widget(para, area);
            }
            LoadingState::Error(msg) => {
                let text = Line::from(vec![
                    Span::styled("X ", Style::default().fg(Color::Red)),
                    Span::raw(msg),
                ]);
                let para = Paragraph::new(text)
                    .block(Block::default().title(" Launch Daemons - Error ").borders(Borders::ALL))
                    .style(Style::default().bg(Color::DarkGray));
                f.render_widget(para, area);
            }
            _ => {
                let filtered: Vec<&LaunchDaemon> = if search_query.is_empty() {
                    items.iter().collect()
                } else {
                    items.iter()
                        .filter(|d| d.label.to_lowercase().contains(&search_query.to_lowercase()) 
                            || d.bundle_name().to_lowercase().contains(&search_query.to_lowercase()))
                        .collect()
                };

                if filtered.is_empty() {
                    let text = if search_query.is_empty() {
                        Line::from("  No launch daemons found.")
                    } else {
                        Line::from(format!("  No items matching '{}'.", search_query))
                    };
                    let para = Paragraph::new(text)
                        .block(Block::default().title(" Launch Daemons ").borders(Borders::ALL))
                        .style(Style::default().bg(Color::DarkGray));
                    f.render_widget(para, area);
                    return;
                }

                let header = Row::new(vec!["", "Label", "PID", "R/L", "K/A"])
                    .style(Style::default().fg(Color::Yellow).add_modifier(ratatui::style::Modifier::BOLD));

                let rows: Vec<Row> = filtered.iter().enumerate().map(|(i, daemon)| {
                    let status = if daemon.loaded { "▶" } else { "■" };
                    let is_selected = i == selected_index;
                    let pid_str = daemon.pid.map(|p| p.to_string()).unwrap_or_else(|| "-".to_string());
                    let run_at_load = if daemon.run_at_load { "●" } else { "○" };
                    let keep_alive = if daemon.keep_alive { "●" } else { "○" };

                    Row::new(vec![
                        status.to_string(),
                        daemon.label.clone(),
                        pid_str,
                        run_at_load.to_string(),
                        keep_alive.to_string(),
                    ])
                    .style(if is_selected {
                        Style::default().bg(Color::Blue).fg(Color::White)
                    } else {
                        Style::default()
                    })
                }).collect();

                let table = Table::new(rows, &[
                    Constraint::Length(2),
                    Constraint::Percentage(35),
                    Constraint::Length(8),
                    Constraint::Length(3),
                    Constraint::Length(3),
                ])
                    .block(Block::default().title(" Launch Daemons (Admin Required) ").borders(Borders::ALL))
                    .header(header);

                f.render_widget(table, area);
            }
        }
    }
}