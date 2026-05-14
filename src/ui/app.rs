//! Main TUI application

use crate::error::AppError;
use crate::models::ItemType;
use crate::services::{
    LaunchAgentsService, LaunchDaemonsService, LoginItemsService, SystemExtensionsService,
};
use crate::state::{AppState, LoadingState, SelectedSection};
use ratatui::{
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Style},
    text::{Line, Span, Text},
    widgets::{Block, Borders, Paragraph, Row, Table},
    Frame,
};

/// Main TUI application
pub struct TuiApp {
    /// Application state
    pub state: AppState,
    /// Last error message
    pub error_message: Option<String>,
}

impl TuiApp {
    /// Create a new TUI application
    pub fn new() -> Self {
        Self {
            state: AppState::new(),
            error_message: None,
        }
    }

    /// Initialize the application (load data)
    pub fn init(&mut self) {
        self.load_data();
    }

    /// Load data for all sections
    pub fn load_data(&mut self) {
        self.error_message = None;

        // Load login items
        self.state.login_items_loading = LoadingState::Loading;
        match LoginItemsService::list() {
            Ok(items) => {
                self.state.login_items = items;
                self.state.login_items_loading = LoadingState::Loaded;
            }
            Err(e) => {
                self.state.login_items_loading = LoadingState::Error(e.to_string());
                self.error_message = Some(e.to_string());
            }
        }

        // Load launch agents
        self.state.launch_agents_loading = LoadingState::Loading;
        match LaunchAgentsService::list() {
            Ok(agents) => {
                self.state.launch_agents = agents;
                self.state.launch_agents_loading = LoadingState::Loaded;
            }
            Err(e) => {
                self.state.launch_agents_loading = LoadingState::Error(e.to_string());
            }
        }

        // Load launch daemons
        self.state.launch_daemons_loading = LoadingState::Loading;
        match LaunchDaemonsService::list() {
            Ok(daemons) => {
                self.state.launch_daemons = daemons;
                self.state.launch_daemons_loading = LoadingState::Loaded;
            }
            Err(e) => {
                self.state.launch_daemons_loading = LoadingState::Error(e.to_string());
            }
        }

        // Load system extensions
        self.state.system_extensions_loading = LoadingState::Loading;
        match SystemExtensionsService::list() {
            Ok(extensions) => {
                self.state.system_extensions = extensions;
                self.state.system_extensions_loading = LoadingState::Loaded;
            }
            Err(e) => {
                self.state.system_extensions_loading = LoadingState::Error(e.to_string());
            }
        }
    }

    /// Refresh the current section
    pub fn refresh_current(&mut self) {
        self.load_data();
    }

    /// Toggle the selected item
    pub fn toggle_selected(&mut self) {
        let items = self.get_all_items();
        
        if self.state.selected_index >= items.len() {
            return;
        }

        let item = &items[self.state.selected_index];
        
        match item.item_type {
            ItemType::LoginItem => {
                if let Some(login_item) = self.state.login_items.iter().find(|i| &i.id == &item.identifier) {
                    if let Some(path) = &login_item.plist_path {
                        let result = if login_item.enabled {
                            LoginItemsService::disable(&item.identifier, &path.to_string_lossy())
                        } else {
                            LoginItemsService::enable(&item.identifier, &path.to_string_lossy())
                        };
                        
                        if let Err(e) = result {
                            self.error_message = Some(e.to_string());
                        } else {
                            self.load_data();
                        }
                    }
                }
            }
            ItemType::LaunchAgent => {
                if let Some(agent) = self.state.launch_agents.iter().find(|a| &a.label == &item.identifier) {
                    let result = if agent.loaded {
                        LaunchAgentsService::unload(&item.identifier, &agent.plist_path.to_string_lossy())
                    } else {
                        LaunchAgentsService::load(&item.identifier, &agent.plist_path.to_string_lossy())
                    };
                    
                    if let Err(e) = result {
                        self.error_message = Some(e.to_string());
                    } else {
                        self.load_data();
                    }
                }
            }
            ItemType::LaunchDaemon => {
                if let Some(daemon) = self.state.launch_daemons.iter().find(|d| &d.label == &item.identifier) {
                    let result = if daemon.loaded {
                        LaunchDaemonsService::unload(&item.identifier, &daemon.plist_path.to_string_lossy())
                    } else {
                        LaunchDaemonsService::load(&item.identifier, &daemon.plist_path.to_string_lossy())
                    };
                    
                    if let Err(e) = result {
                        self.error_message = Some(e.to_string());
                    } else {
                        self.load_data();
                    }
                }
            }
            ItemType::SystemExtension => {
                if let Some(ext) = self.state.system_extensions.iter().find(|e| &e.identifier == &item.identifier) {
                    let result = if ext.is_activated() {
                        SystemExtensionsService::deactivate(&item.identifier)
                    } else {
                        SystemExtensionsService::activate(&item.identifier)
                    };
                    
                    if let Err(e) = result {
                        self.error_message = Some(e.to_string());
                    } else {
                        self.load_data();
                    }
                }
            }
        }
    }

    /// Get all items combined into a unified list
    fn get_all_items(&self) -> Vec<UnifiedItem> {
        let mut items = Vec::new();

        // Login Items
        for item in &self.state.login_items {
            let status = if item.enabled { "enabled" } else { "disabled" };
            items.push(UnifiedItem {
                item_type: ItemType::LoginItem,
                name: item.name.clone(),
                identifier: item.id.clone(),
                status: status.to_string(),
                detail: item.path.display().to_string(),
            });
        }

        // Launch Agents
        for agent in &self.state.launch_agents {
            let status = if agent.loaded { "loaded" } else { "unloaded" };
            let pid = agent.pid.map(|p| p.to_string()).unwrap_or_else(|| "-".to_string());
            items.push(UnifiedItem {
                item_type: ItemType::LaunchAgent,
                name: agent.bundle_name(),
                identifier: agent.label.clone(),
                status: status.to_string(),
                detail: format!("PID: {}", pid),
            });
        }

        // Launch Daemons
        for daemon in &self.state.launch_daemons {
            let status = if daemon.loaded { "loaded" } else { "unloaded" };
            let pid = daemon.pid.map(|p| p.to_string()).unwrap_or_else(|| "-".to_string());
            items.push(UnifiedItem {
                item_type: ItemType::LaunchDaemon,
                name: daemon.label.clone(),
                identifier: daemon.label.clone(),
                status: status.to_string(),
                detail: format!("PID: {}", pid),
            });
        }

        // System Extensions
        for ext in &self.state.system_extensions {
            let display_name = ext.name.as_deref().unwrap_or(&ext.identifier);
            let ext_type = ext.extension_types.first().map(|t| t.to_string()).unwrap_or_else(|| "Extension".to_string());
            items.push(UnifiedItem {
                item_type: ItemType::SystemExtension,
                name: display_name.to_string(),
                identifier: ext.identifier.clone(),
                status: ext.status.to_string().to_lowercase(),
                detail: format!("{} v{}", ext_type, ext.version),
            });
        }

        items
    }

    /// Get filtered items based on search
    fn get_filtered_items(&self) -> Vec<UnifiedItem> {
        let items = self.get_all_items();
        
        if self.state.search_query.is_empty() {
            return items;
        }

        let query = self.state.search_query.to_lowercase();
        items.into_iter()
            .filter(|item| {
                item.name.to_lowercase().contains(&query)
                    || item.identifier.to_lowercase().contains(&query)
                    || item.item_type.display_name().to_lowercase().contains(&query)
            })
            .collect()
    }

    /// Render the application
    pub fn render(&self, f: &mut Frame) {
        let area = f.size();

        // Main layout: content (with header embedded)
        let layout = Layout::default()
            .direction(Direction::Vertical)
            .constraints([
                Constraint::Length(1),  // Title bar
                Constraint::Min(0),      // Main content (table)
                Constraint::Length(1),   // Shortcuts bar
            ])
            .split(area);

        // Title bar
        self.render_title_bar(f, layout[0]);

        // Main content area with table
        self.render_table(f, layout[1]);

        // Shortcuts bar at bottom
        self.render_shortcuts_bar(f, layout[2]);

        // Help overlay (covers everything)
        if self.state.show_help {
            self.render_help_overlay(f, area);
        }

        // Error message overlay
        if let Some(ref err) = self.error_message {
            self.render_error_overlay(f, err, area);
        }
    }

    /// Render the title bar
    fn render_title_bar(&self, f: &mut Frame, area: Rect) {
        let total = self.get_filtered_items().len();
        let login_count = self.state.login_items.len();
        let agent_count = self.state.launch_agents.len();
        let daemon_count = self.state.launch_daemons.len();
        let ext_count = self.state.system_extensions.len();

        let title = format!(
            " System Extension Manager │ Items: {} │ Login:{} │ Agents:{} │ Daemons:{} │ Exts:{} ",
            total, login_count, agent_count, daemon_count, ext_count
        );
        
        let para = Paragraph::new(title)
            .style(Style::default()
                .bg(Color::Blue)
                .fg(Color::White)
                .add_modifier(ratatui::style::Modifier::BOLD));
        
        f.render_widget(para, area);
    }

    /// Render the table
    fn render_table(&self, f: &mut Frame, area: Rect) {
        let items = self.get_filtered_items();

        // Empty state
        if items.is_empty() {
            let msg = if self.state.search_query.is_empty() {
                " No items found. Press 'r' to refresh."
            } else {
                &format!(" No items match '{}'. Press Esc to clear search.", self.state.search_query)
            };
            
            let block = Block::default()
                .borders(Borders::ALL)
                .border_style(Style::default().fg(Color::DarkGray));
            
            let para = Paragraph::new(msg.to_string())
                .block(block)
                .style(Style::default().fg(Color::White));
            
            f.render_widget(para, area);
            return;
        }

        // Calculate visible area height (accounting for header)
        let header_height = 1;
        // Reserve 1 row for the selected item at the bottom
        let available_height = area.height.saturating_sub(header_height).saturating_sub(1);

        // Calculate scroll offset based on selection
        let mut scroll_offset = self.state.scroll_offset;
        
        // If selection is below visible area, scroll down
        if self.state.selected_index >= scroll_offset + available_height as usize {
            scroll_offset = self.state.selected_index - available_height as usize + 1;
        }
        // If selection is above visible area, scroll up
        if self.state.selected_index < scroll_offset {
            scroll_offset = self.state.selected_index;
        }
        
        // Ensure scroll_offset is valid
        let max_scroll = items.len().saturating_sub(available_height as usize);
        scroll_offset = scroll_offset.min(max_scroll);

        // Get visible items
        let visible_items: Vec<_> = items.iter()
            .skip(scroll_offset)
            .take(available_height as usize)
            .enumerate()
            .collect();

        // Table header
        let header = Row::new(vec!["Type", "Name", "Status", "Details"])
            .style(Style::default()
                .fg(Color::White)
                .add_modifier(ratatui::style::Modifier::BOLD))
            .height(1);

        // Table rows
        let rows: Vec<Row> = visible_items.iter().map(|(i, item)| {
            // Absolute index in full list
            let abs_index = i + scroll_offset;
            let is_selected = abs_index == self.state.selected_index;
            let type_str = match item.item_type {
                ItemType::LoginItem => "Login Item",
                ItemType::LaunchAgent => "Launch Agent",
                ItemType::LaunchDaemon => "Launch Daemon",
                ItemType::SystemExtension => "System Ext",
            };

            Row::new(vec![
                type_str,
                &item.name,
                item.status.as_str(),
                &item.detail,
            ])
            .style(if is_selected {
                Style::default().bg(Color::Blue).fg(Color::White)
            } else {
                Style::default().fg(Color::White)
            })
        }).collect();

        // Show scroll indicators if there are more items
        let total_items = items.len();
        let can_scroll_up = scroll_offset > 0;
        let can_scroll_down = (scroll_offset + available_height as usize) < total_items;

        let table = Table::new(rows, &[
            Constraint::Length(14),
            Constraint::Percentage(40),
            Constraint::Length(12),
            Constraint::Min(0),
        ])
        .header(header)
        .block(
            Block::default()
                .borders(Borders::ALL)
                .title_bottom(format!(
                    "{}{}{}",
                    if can_scroll_up { "▲ " } else { "  " },
                    format!(" {}/{} ", self.state.selected_index + 1, total_items),
                    if can_scroll_down { " ▼" } else { "  " }
                ))
        )
        .style(Style::default().bg(Color::Black));

        f.render_widget(table, area);
    }

    /// Render shortcuts bar
    fn render_shortcuts_bar(&self, f: &mut Frame, area: Rect) {
        let shortcuts = Line::from(vec![
            Span::styled(" ↑↓ ", Style::default().fg(Color::Yellow)),
            Span::raw("Navigate  "),
            Span::styled(" Space ", Style::default().fg(Color::Yellow)),
            Span::raw("Toggle  "),
            Span::styled(" / ", Style::default().fg(Color::Yellow)),
            Span::raw("Search  "),
            Span::styled(" g/G ", Style::default().fg(Color::Yellow)),
            Span::raw("Top/Bottom  "),
            Span::styled(" r ", Style::default().fg(Color::Yellow)),
            Span::raw("Refresh  "),
            Span::styled(" q ", Style::default().fg(Color::Yellow)),
            Span::raw("Quit"),
        ]);

        let para = Paragraph::new(shortcuts)
            .style(Style::default().bg(Color::Black).fg(Color::White));
        
        f.render_widget(para, area);
    }

    /// Render help overlay
    fn render_help_overlay(&self, f: &mut Frame, area: Rect) {
        let block = Block::default()
            .title(" Keyboard Shortcuts ")
            .borders(Borders::ALL)
            .border_style(Style::default().fg(Color::Cyan))
            .style(Style::default().bg(Color::Black));

        let content = Text::from(vec![
            Line::from(vec![Span::styled(" Navigation ", Style::default().fg(Color::Cyan).add_modifier(ratatui::style::Modifier::BOLD))]),
            Line::from(vec![Span::raw("")]),
            Line::from(vec![Span::styled(" ↑ / k    ", Style::default().fg(Color::Yellow)), Span::raw("Move selection up")]),
            Line::from(vec![Span::styled(" ↓ / j    ", Style::default().fg(Color::Yellow)), Span::raw("Move selection down")]),
            Line::from(vec![Span::styled(" g        ", Style::default().fg(Color::Yellow)), Span::raw("Go to top of list")]),
            Line::from(vec![Span::styled(" G        ", Style::default().fg(Color::Yellow)), Span::raw("Go to bottom of list")]),
            Line::from(vec![Span::raw("")]),
            Line::from(vec![Span::styled(" Actions ", Style::default().fg(Color::Cyan).add_modifier(ratatui::style::Modifier::BOLD))]),
            Line::from(vec![Span::raw("")]),
            Line::from(vec![Span::styled(" Space    ", Style::default().fg(Color::Yellow)), Span::raw("Toggle selected item on/off")]),
            Line::from(vec![Span::styled(" Enter    ", Style::default().fg(Color::Yellow)), Span::raw("Toggle selected item on/off")]),
            Line::from(vec![Span::styled(" /        ", Style::default().fg(Color::Yellow)), Span::raw("Focus search input")]),
            Line::from(vec![Span::styled(" Esc      ", Style::default().fg(Color::Yellow)), Span::raw("Clear search / close dialogs")]),
            Line::from(vec![Span::styled(" r        ", Style::default().fg(Color::Yellow)), Span::raw("Refresh all items")]),
            Line::from(vec![Span::raw("")]),
            Line::from(vec![Span::styled(" Quit: q  ", Style::default().fg(Color::Yellow)), Span::raw("Exit application")]),
            Line::from(vec![Span::raw("")]),
            Line::from(vec![Span::styled(" Press Esc or ? to close this help ", Style::default().fg(Color::DarkGray))]),
        ]);

        let para = Paragraph::new(content)
            .block(block)
            .style(Style::default().bg(Color::Black).fg(Color::White));

        f.render_widget(para, area);
    }

    /// Render error overlay
    fn render_error_overlay(&self, f: &mut Frame, error: &str, area: Rect) {
        let block = Block::default()
            .title(" Error ")
            .borders(Borders::ALL)
            .border_style(Style::default().fg(Color::Red));

        let content = Text::from(vec![
            Line::from(vec![Span::raw(error)]),
            Line::from(vec![Span::raw("")]),
            Line::from(vec![Span::styled("Press any key to dismiss", Style::default().fg(Color::DarkGray))]),
        ]);

        let para = Paragraph::new(content)
            .block(block)
            .style(Style::default().bg(Color::Black).fg(Color::Red));

        f.render_widget(para, area);
    }

    /// Handle keyboard input
    pub fn handle_key(&mut self, key: &str) -> bool {
        // If showing error, any key dismisses it
        if self.error_message.is_some() {
            self.error_message = None;
            return true;
        }

        // If showing help, only Escape or ? closes it
        if self.state.show_help {
            if key == "escape" || key == "?" {
                self.state.show_help = false;
            }
            return true;
        }

        match key {
            "q" | "Q" | "ctrl-c" => return false,
            "?" => {
                self.state.show_help = true;
            }
            "r" | "R" => {
                self.refresh_current();
            }
            "k" | "up" => {
                let len = self.get_filtered_items().len();
                if self.state.selected_index > 0 && len > 0 {
                    self.state.selected_index -= 1;
                }
            }
            "j" | "down" => {
                let len = self.get_filtered_items().len();
                if self.state.selected_index < len.saturating_sub(1) {
                    self.state.selected_index += 1;
                }
            }
            "g" => {
                // Go to top
                self.state.selected_index = 0;
            }
            "G" => {
                // Go to bottom
                let len = self.get_filtered_items().len();
                self.state.selected_index = len.saturating_sub(1);
            }
            " " | "space" | "enter" => {
                // Toggle selected item
                self.toggle_selected();
            }
            "/" => {
                self.state.selected_section = SelectedSection::Search;
                self.state.search_query.clear();
            }
            "escape" => {
                if !self.state.search_query.is_empty() {
                    self.state.search_query.clear();
                } else {
                    self.state.selected_section = SelectedSection::Content;
                }
            }
            "backspace" => {
                if matches!(self.state.selected_section, SelectedSection::Search) {
                    self.state.search_query.pop();
                }
            }
            _ => {
                // Handle search input
                if matches!(self.state.selected_section, SelectedSection::Search) {
                    if key.len() == 1 {
                        self.state.search_query.push(key.chars().next().unwrap());
                    }
                }
            }
        }
        true
    }
}

impl Default for TuiApp {
    fn default() -> Self {
        Self::new()
    }
}

/// Unified item representation for the combined table
struct UnifiedItem {
    item_type: ItemType,
    name: String,
    identifier: String,
    status: String,
    detail: String,
}