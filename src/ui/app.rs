//! Main TUI application

use crate::error::AppError;
use crate::models::ItemType;
use crate::services::{
    LaunchAgentsService, LaunchDaemonsService, LoginItemsService, SystemExtensionsService,
};
use crate::state::{AppState, LoadingState, SelectedSection};
use ratatui::{
    layout::{Constraint, Direction, Layout, Margin, Rect},
    style::{Color, Style},
    text::{Line, Span, Text},
    widgets::{Block, Borders, Cell, Padding, Paragraph, Row, Table},
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
        
        // Reset scroll and clamp selection against the visible (filtered) list
        self.state.scroll_offset = 0;
        let total = self.get_filtered_items().len();
        if total == 0 {
            self.state.selected_index = 0;
        } else if self.state.selected_index >= total {
            self.state.selected_index = total - 1;
        }
    }

    /// Refresh the current section
    pub fn refresh_current(&mut self) {
        self.load_data();
    }

    /// Enable the selected item
    pub fn enable_selected(&mut self) {
        self.set_selected_state(true);
    }

    /// Disable the selected item
    pub fn disable_selected(&mut self) {
        self.set_selected_state(false);
    }

    fn set_selected_state(&mut self, enable: bool) {
        let items = self.get_filtered_items();
        if self.state.selected_index >= items.len() {
            return;
        }
        let item = &items[self.state.selected_index];
        let identifier = item.identifier.clone();

        let result = match item.item_type {
            ItemType::LoginItem => {
                self.state.login_items.iter()
                    .find(|i| i.id == identifier)
                    .and_then(|i| i.plist_path.as_ref().map(|p| {
                        if enable {
                            LoginItemsService::enable(&identifier, &p.to_string_lossy())
                        } else {
                            LoginItemsService::disable(&identifier, &p.to_string_lossy())
                        }
                    }))
            }
            ItemType::LaunchAgent => {
                self.state.launch_agents.iter()
                    .find(|a| a.label == identifier)
                    .map(|a| {
                        if enable {
                            LaunchAgentsService::load(&identifier, &a.plist_path.to_string_lossy())
                        } else {
                            LaunchAgentsService::unload(&identifier, &a.plist_path.to_string_lossy())
                        }
                    })
            }
            ItemType::LaunchDaemon => {
                self.state.launch_daemons.iter()
                    .find(|d| d.label == identifier)
                    .map(|d| {
                        if enable {
                            LaunchDaemonsService::load(&identifier, &d.plist_path.to_string_lossy())
                        } else {
                            LaunchDaemonsService::unload(&identifier, &d.plist_path.to_string_lossy())
                        }
                    })
            }
            ItemType::SystemExtension => {
                Some(Err(crate::error::AppError::ExtensionActivationFailed(
                    "System extensions cannot be managed via CLI. Use System Settings → General → Login Items & Extensions.".to_string()
                )))
            }
        };

        match result {
            Some(Err(e)) => self.error_message = Some(e.to_string()),
            Some(Ok(_)) => self.load_data(),
            None => {}
        }
    }

    /// Get all items combined into a unified list
    fn get_all_items(&self) -> Vec<UnifiedItem> {
        let mut items = Vec::new();

        let sys = self.state.show_system_names;

        // Login Items
        if self.state.show_login_items {
            for item in &self.state.login_items {
                items.push(UnifiedItem {
                    item_type: ItemType::LoginItem,
                    name: if sys { item.id.clone() } else { item.name.clone() },
                    identifier: item.id.clone(),
                    status: if item.enabled { "enabled" } else { "disabled" }.to_string(),
                    is_enabled: item.enabled,
                    disabled_count: None,
                });
            }
        }

        // Launch Agents
        if self.state.show_launch_agents {
            for agent in &self.state.launch_agents {
                items.push(UnifiedItem {
                    item_type: ItemType::LaunchAgent,
                    name: if sys { agent.label.clone() } else { agent.bundle_name() },
                    identifier: agent.label.clone(),
                    status: if agent.loaded { "loaded" } else { "unloaded" }.to_string(),
                    is_enabled: agent.loaded,
                    disabled_count: None,
                });
            }
        }

        // Launch Daemons
        if self.state.show_launch_daemons {
            for daemon in &self.state.launch_daemons {
                items.push(UnifiedItem {
                    item_type: ItemType::LaunchDaemon,
                    name: daemon.label.clone(),
                    identifier: daemon.label.clone(),
                    status: if daemon.loaded { "loaded" } else { "unloaded" }.to_string(),
                    is_enabled: daemon.loaded,
                    disabled_count: None,
                });
            }
        }

        // System Extensions
        if self.state.show_system_extensions {
            for ext in &self.state.system_extensions {
                let name = if sys {
                    ext.identifier.clone()
                } else {
                    ext.name.as_deref().unwrap_or(&ext.identifier).to_string()
                };
                items.push(UnifiedItem {
                    item_type: ItemType::SystemExtension,
                    name,
                    identifier: ext.identifier.clone(),
                    status: ext.status.to_string().to_lowercase(),
                    is_enabled: ext.is_activated(),
                    disabled_count: None,
                });
            }
        }

        items
    }

    /// Get filtered and sorted items, with disabled items collapsed into summary rows when hidden
    fn get_filtered_items(&self) -> Vec<UnifiedItem> {
        fn type_rank(t: ItemType) -> u8 {
            match t {
                ItemType::LoginItem => 0,
                ItemType::LaunchAgent => 1,
                ItemType::LaunchDaemon => 2,
                ItemType::SystemExtension => 3,
            }
        }

        let query = self.state.search_query.to_lowercase();
        let mut items: Vec<UnifiedItem> = self.get_all_items()
            .into_iter()
            .filter(|item| {
                query.is_empty()
                    || item.name.to_lowercase().contains(&query)
                    || item.identifier.to_lowercase().contains(&query)
                    || item.item_type.display_name().to_lowercase().contains(&query)
            })
            .collect();

        items.sort_by(|a, b| {
            type_rank(a.item_type).cmp(&type_rank(b.item_type))
                .then(b.is_enabled.cmp(&a.is_enabled))
                .then(a.name.to_lowercase().cmp(&b.name.to_lowercase()))
        });

        if !self.state.hide_disabled {
            return items;
        }

        // Per-type: show enabled items, then either individual disabled or a summary row
        let mut result = Vec::new();
        for item_type in [ItemType::LoginItem, ItemType::LaunchAgent, ItemType::LaunchDaemon, ItemType::SystemExtension] {
            let disabled_count = items.iter()
                .filter(|i| i.item_type == item_type && !i.is_enabled)
                .count();

            for item in items.iter().filter(|i| i.item_type == item_type && i.is_enabled).cloned() {
                result.push(item);
            }

            if disabled_count > 0 {
                if self.state.expanded_disabled.contains(&item_type) {
                    for item in items.iter().filter(|i| i.item_type == item_type && !i.is_enabled).cloned() {
                        result.push(item);
                    }
                } else {
                    let label = match item_type {
                        ItemType::LoginItem => "items",
                        ItemType::LaunchAgent => "agents",
                        ItemType::LaunchDaemon => "daemons",
                        ItemType::SystemExtension => "extensions",
                    };
                    result.push(UnifiedItem {
                        item_type,
                        name: format!("+{} disabled {}", disabled_count, label),
                        identifier: String::new(),
                        status: String::new(),
                        is_enabled: false,
                        disabled_count: Some(disabled_count),
                    });
                }
            }
        }
        result
    }

    /// Render the application
    pub fn render(&mut self, f: &mut Frame) {
        let area = f.size();

        let layout = Layout::default()
            .direction(Direction::Vertical)
            .constraints([
                Constraint::Length(1),  // Title bar
                Constraint::Length(1),  // Filter bar
                Constraint::Min(0),      // Main content (table)
                Constraint::Length(1),   // Shortcuts bar
            ])
            .split(area);

        self.render_title_bar(f, layout[0]);
        self.render_filter_bar(f, layout[1]);
        self.render_table(f, layout[2]);
        self.render_shortcuts_bar(f, layout[3]);

        // Help overlay (covers everything)
        if self.state.show_help {
            self.render_help_overlay(f, area);
        }

        // Error message overlay
        if let Some(ref err) = self.error_message {
            self.render_error_overlay(f, err, area);
        }
    }

    /// Render the filter bar
    fn render_filter_bar(&self, f: &mut Frame, area: Rect) {
        let is_search = matches!(self.state.selected_section, SelectedSection::Search);
        let query = self.state.search_query.clone();

        let content = if is_search {
            Line::from(vec![
                Span::styled("Filter: ", Style::default().fg(Color::Rgb(230, 180, 50)).add_modifier(ratatui::style::Modifier::BOLD)),
                Span::raw(query),
                Span::styled("_", Style::default().fg(Color::Rgb(230, 180, 50))),
            ])
        } else if !self.state.search_query.is_empty() {
            Line::from(vec![
                Span::styled("Filter: ", Style::default().fg(Color::Rgb(230, 180, 50))),
                Span::raw(query),
                Span::styled("  (Esc to clear)", Style::default().fg(Color::Rgb(100, 100, 100))),
            ])
        } else {
            Line::from(vec![
                Span::styled("Filter: ", Style::default().fg(Color::Rgb(100, 100, 100))),
                Span::styled("press f to filter", Style::default().fg(Color::Rgb(100, 100, 100))),
            ])
        };

        let para = Paragraph::new(content)
            .block(Block::default().padding(Padding::horizontal(1)))
            .style(Style::default().bg(Color::Rgb(18, 18, 18)));

        f.render_widget(para, area);
    }

    /// Render the title bar
    fn render_title_bar(&self, f: &mut Frame, area: Rect) {
        let total = self.get_filtered_items().len();
        let login_count = self.state.login_items.len();
        let agent_count = self.state.launch_agents.len();
        let daemon_count = self.state.launch_daemons.len();
        let ext_count = self.state.system_extensions.len();

        let title = format!(
            "System Extension Manager │ Items: {} │ Login:{} │ Agents:{} │ Daemons:{} │ Exts:{}",
            total, login_count, agent_count, daemon_count, ext_count
        );

        let para = Paragraph::new(title)
            .block(Block::default().padding(Padding::horizontal(1)))
            .style(Style::default()
                .bg(Color::Rgb(40, 80, 160))
                .fg(Color::Rgb(215, 215, 215))
                .add_modifier(ratatui::style::Modifier::BOLD));

        f.render_widget(para, area);
    }

    /// Render the table
    fn render_table(&mut self, f: &mut Frame, area: Rect) {
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
                .border_style(Style::default().fg(Color::Rgb(100, 100, 100)));
            
            let para = Paragraph::new(msg.to_string())
                .block(block)
                .style(Style::default().fg(Color::Rgb(215, 215, 215)));
            
            f.render_widget(para, area);
            return;
        }

        // The table widget renders: header + rows. Borders take 2 rows (top/bottom).
        // Total area height includes everything. We need to account for borders.
        let area_height = area.height as usize;
        
        // Table renders: 1 header row + N data rows + 2 border rows (top/bottom of block)
        // So visible data rows = area_height - 1 (header) - 2 (borders) = area_height - 3
        let table_border_rows = 2;
        let header_row = 1;
        let bottom_padding = 1;
        let data_rows = area_height.saturating_sub(header_row + table_border_rows + bottom_padding);
        let visible_rows = data_rows.max(1);
        
        let items_len = items.len();
        
        // Get selected index clamped to valid range
        let selected_idx = if items_len == 0 { 0 } else { self.state.selected_index.min(items_len - 1) };
        
        // Calculate scroll offset - always keep selected item visible
        let mut scroll_offset = self.state.scroll_offset;
        
        // If selected is below visible area, scroll down
        let bottom_visible = scroll_offset + visible_rows;
        if selected_idx >= bottom_visible {
            scroll_offset = selected_idx - visible_rows + 1;
        }
        // If selected is above visible area, scroll up
        if selected_idx < scroll_offset {
            scroll_offset = selected_idx;
        }
        
        // Clamp scroll offset to valid range
        if items_len > visible_rows {
            let max_offset = items_len - visible_rows;
            scroll_offset = scroll_offset.min(max_offset);
        } else {
            scroll_offset = 0;
        }

        // Persist so the next frame starts from the same position
        self.state.scroll_offset = scroll_offset;

        // Get items to display
        let display_items: Vec<_> = items.iter()
            .skip(scroll_offset)
            .take(visible_rows)
            .enumerate()
            .map(|(i, item)| (scroll_offset + i, item))
            .collect();

        // Table header
        let header = Row::new(vec!["Type", "Name", "Status"])
            .style(Style::default()
                .fg(Color::Rgb(215, 215, 215))
                .add_modifier(ratatui::style::Modifier::BOLD))
            .height(1);

        // Table rows
        let rows: Vec<Row> = display_items.iter().map(|(abs_index, item)| {
            let is_selected = *abs_index == selected_idx;
            let type_str = match item.item_type {
                ItemType::LoginItem => "Login Item",
                ItemType::LaunchAgent => "Launch Agent",
                ItemType::LaunchDaemon => "Launch Daemon",
                ItemType::SystemExtension => "System Extension",
            };

            if item.disabled_count.is_some() {
                // Summary row for collapsed disabled items
                let dis = Style::default().fg(Color::Rgb(100, 100, 100));
                let sel_dis = Style::default().bg(Color::Rgb(40, 80, 160)).fg(Color::Rgb(100, 100, 100));
                let s = if is_selected { sel_dis } else { dis };
                Row::new(vec![
                    Cell::from("").style(s),
                    Cell::from(item.name.as_str()).style(s),
                    Cell::from("").style(s),
                ])
            } else if is_selected {
                let sel = Style::default().bg(Color::Rgb(40, 80, 160)).fg(Color::Rgb(215, 215, 215));
                Row::new(vec![
                    Cell::from(type_str).style(sel),
                    Cell::from(item.name.as_str()).style(sel),
                    Cell::from(item.status.as_str()).style(sel),
                ])
            } else if item.is_enabled {
                let text = Style::default().fg(Color::Rgb(215, 215, 215));
                let status_style = Style::default().fg(Color::Rgb(72, 199, 142));
                Row::new(vec![
                    Cell::from(type_str).style(text),
                    Cell::from(item.name.as_str()).style(text),
                    Cell::from(item.status.as_str()).style(status_style),
                ])
            } else {
                let dis = Style::default().fg(Color::Rgb(100, 100, 100));
                Row::new(vec![
                    Cell::from(type_str).style(dis),
                    Cell::from(item.name.as_str()).style(dis),
                    Cell::from(item.status.as_str()).style(dis),
                ])
            }
        }).collect();

        // Scroll indicators
        let can_scroll_up = scroll_offset > 0;
        let can_scroll_down = scroll_offset + visible_rows < items_len;
        let position = format!(" {}/{}", selected_idx + 1, items_len);

        let table = Table::new(rows, &[
            Constraint::Length(17),
            Constraint::Percentage(60),
            Constraint::Min(20),
        ])
        .header(header)
        .block(
            Block::default()
                .borders(Borders::ALL)
                .padding(Padding::new(1, 1, 0, 1))
                .title_bottom(format!(
                    "{}{}{}",
                    if can_scroll_up { "▲ " } else { "" },
                    position,
                    if can_scroll_down { " ▼" } else { "" }
                ))
        )
        .style(Style::default().bg(Color::Rgb(18, 18, 18)));

        let outer = Rect {
            x: area.x + 1,
            y: area.y,
            width: area.width.saturating_sub(2),
            height: area.height,
        };
        f.render_widget(table, outer);
    }

    /// Render shortcuts bar
    fn render_shortcuts_bar(&self, f: &mut Frame, area: Rect) {
        let key = Style::default().fg(Color::Rgb(220, 70, 70)).add_modifier(ratatui::style::Modifier::BOLD);
        let shortcuts = Line::from(vec![
            Span::styled("↑↓", key),
            Span::raw(" Nav  "),
            Span::styled("f", key),
            Span::raw("ilter  "),
            Span::styled("n", key),
            Span::raw("ame  "),
            Span::styled("t", key),
            Span::raw("oggle  "),
            Span::styled("[ ]", key),
            Span::raw(" collapse / expand  "),
            Span::styled("o", key),
            Span::raw("pen parent dir  "),
            Span::styled("r", key),
            Span::raw("efresh  "),
            Span::styled("q", key),
            Span::raw("uit"),
        ]);

        let para = Paragraph::new(shortcuts)
            .block(Block::default().padding(Padding::horizontal(1)))
            .style(Style::default().bg(Color::Rgb(18, 18, 18)).fg(Color::Rgb(215, 215, 215)));
        
        f.render_widget(para, area);
    }

    /// Render help overlay
    fn render_help_overlay(&self, f: &mut Frame, area: Rect) {
        let block = Block::default()
            .title(" Keyboard Shortcuts ")
            .borders(Borders::ALL)
            .border_style(Style::default().fg(Color::Rgb(70, 190, 200)))
            .style(Style::default().bg(Color::Rgb(18, 18, 18)));

        let content = Text::from(vec![
            Line::from(vec![Span::styled(" Navigation ", Style::default().fg(Color::Rgb(70, 190, 200)).add_modifier(ratatui::style::Modifier::BOLD))]),
            Line::from(vec![Span::raw("")]),
            Line::from(vec![
                Span::styled("↑", Style::default().fg(Color::Rgb(220, 70, 70)).add_modifier(ratatui::style::Modifier::BOLD)),
                Span::styled("/k", Style::default().fg(Color::Rgb(220, 70, 70)).add_modifier(ratatui::style::Modifier::BOLD)),
                Span::raw("   Move selection up"),
            ]),
            Line::from(vec![
                Span::styled("↓", Style::default().fg(Color::Rgb(220, 70, 70)).add_modifier(ratatui::style::Modifier::BOLD)),
                Span::styled("/j", Style::default().fg(Color::Rgb(220, 70, 70)).add_modifier(ratatui::style::Modifier::BOLD)),
                Span::raw("   Move selection down"),
            ]),
            Line::from(vec![Span::styled("g", Style::default().fg(Color::Rgb(220, 70, 70)).add_modifier(ratatui::style::Modifier::BOLD)), Span::raw("        Go to top of list")]),
            Line::from(vec![Span::styled("G", Style::default().fg(Color::Rgb(220, 70, 70)).add_modifier(ratatui::style::Modifier::BOLD)), Span::raw("        Go to bottom of list")]),
            Line::from(vec![Span::raw("")]),
            Line::from(vec![Span::styled(" Actions ", Style::default().fg(Color::Rgb(70, 190, 200)).add_modifier(ratatui::style::Modifier::BOLD))]),
            Line::from(vec![Span::raw("")]),
            Line::from(vec![Span::styled("Space", Style::default().fg(Color::Rgb(220, 70, 70)).add_modifier(ratatui::style::Modifier::BOLD)), Span::raw("     Toggle selected item on/off")]),
            Line::from(vec![Span::styled("o", Style::default().fg(Color::Rgb(220, 70, 70)).add_modifier(ratatui::style::Modifier::BOLD)), Span::raw("        Open item location in Finder")]),
            Line::from(vec![Span::styled("/", Style::default().fg(Color::Rgb(220, 70, 70)).add_modifier(ratatui::style::Modifier::BOLD)), Span::raw("        Focus search input")]),
            Line::from(vec![Span::styled("Esc", Style::default().fg(Color::Rgb(220, 70, 70)).add_modifier(ratatui::style::Modifier::BOLD)), Span::raw("      Clear search / close dialogs")]),
            Line::from(vec![Span::styled("r", Style::default().fg(Color::Rgb(220, 70, 70)).add_modifier(ratatui::style::Modifier::BOLD)), Span::raw("        Refresh all items")]),
            Line::from(vec![Span::raw("")]),
            Line::from(vec![Span::styled("q", Style::default().fg(Color::Rgb(220, 70, 70)).add_modifier(ratatui::style::Modifier::BOLD)), Span::raw("        Exit application")]),
            Line::from(vec![Span::raw("")]),
            Line::from(vec![Span::styled(" Press Esc or ? to close this help ", Style::default().fg(Color::Rgb(100, 100, 100)))]),
        ]);

        let para = Paragraph::new(content)
            .block(block)
            .style(Style::default().bg(Color::Rgb(18, 18, 18)).fg(Color::Rgb(215, 215, 215)));

        f.render_widget(para, area);
    }

    /// Render error overlay
    fn render_error_overlay(&self, f: &mut Frame, error: &str, area: Rect) {
        let block = Block::default()
            .title(" Error ")
            .borders(Borders::ALL)
            .border_style(Style::default().fg(Color::Rgb(220, 70, 70)));

        let content = Text::from(vec![
            Line::from(vec![Span::raw(error)]),
            Line::from(vec![Span::raw("")]),
            Line::from(vec![Span::styled("Press any key to dismiss", Style::default().fg(Color::Rgb(100, 100, 100)))]),
        ]);

        let para = Paragraph::new(content)
            .block(block)
            .style(Style::default().bg(Color::Rgb(18, 18, 18)).fg(Color::Rgb(220, 70, 70)));

        f.render_widget(para, area);
    }

    /// Handle keyboard input
    pub fn handle_key(&mut self, key: &str) -> bool {
        if self.error_message.is_some() {
            self.error_message = None;
            return true;
        }

        if self.state.show_help {
            if key == "escape" || key == "?" {
                self.state.show_help = false;
            }
            return true;
        }

        // In filter mode all input goes to the query — shortcuts are suspended
        if matches!(self.state.selected_section, SelectedSection::Search) {
            match key {
                "escape" => {
                    self.state.search_query.clear();
                    self.state.selected_section = SelectedSection::Content;
                    self.state.selected_index = 0;
                    self.state.scroll_offset = 0;
                }
                "backspace" => {
                    if self.state.search_query.is_empty() {
                        self.state.selected_section = SelectedSection::Content;
                    } else {
                        self.state.search_query.pop();
                    }
                }
                "enter" => {
                    self.state.selected_section = SelectedSection::Content;
                }
                "up" => {
                    let items = self.get_filtered_items();
                    if self.state.selected_index > 0 && !items.is_empty() {
                        self.state.selected_index -= 1;
                    }
                }
                "down" => {
                    let items = self.get_filtered_items();
                    if self.state.selected_index < items.len().saturating_sub(1) {
                        self.state.selected_index += 1;
                    }
                }
                _ if key.len() == 1 => {
                    self.state.search_query.push(key.chars().next().unwrap());
                }
                _ => {}
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
"f" | "F" | "/" => {
                self.state.selected_section = SelectedSection::Search;
                self.state.search_query.clear();
            }
            "t" | "T" => {
                let items = self.get_filtered_items();
                if let Some(item) = items.get(self.state.selected_index) {
                    let enable = !item.is_enabled;
                    self.set_selected_state(enable);
                }
            }
            "]" => {
                for t in [ItemType::LoginItem, ItemType::LaunchAgent, ItemType::LaunchDaemon, ItemType::SystemExtension] {
                    self.state.expanded_disabled.insert(t);
                }
            }
            "[" => {
                self.state.expanded_disabled.clear();
            }
            "n" | "N" => {
                self.state.show_system_names = !self.state.show_system_names;
            }
            "o" | "O" => {
                self.open_in_finder();
            }
            "k" | "up" => {
                let items = self.get_filtered_items();
                if self.state.selected_index > 0 && !items.is_empty() {
                    self.state.selected_index -= 1;
                }
            }
            "j" | "down" => {
                let items = self.get_filtered_items();
                if self.state.selected_index < items.len().saturating_sub(1) {
                    self.state.selected_index += 1;
                }
            }
            "g" => {
                self.state.selected_index = 0;
                self.state.scroll_offset = 0;
            }
            "G" => {
                let items = self.get_filtered_items();
                if !items.is_empty() {
                    self.state.selected_index = items.len() - 1;
                }
            }
            _ => {}
        }
        true
    }

    /// Open the selected item's location in Finder, with the item selected
    fn open_in_finder(&mut self) {
        use std::process::Command;

        let items = self.get_filtered_items();
        if self.state.selected_index >= items.len() {
            return;
        }

        let item = &items[self.state.selected_index];
        let identifier = item.identifier.clone();

        match item.item_type {
            ItemType::LoginItem => {
                if let Some(login_item) = self.state.login_items.iter().find(|i| i.id == identifier) {
                    if login_item.path.exists() {
                        let _ = Command::new("open").args(["-R", &login_item.path.to_string_lossy()]).spawn();
                    } else if let Some(plist) = &login_item.plist_path {
                        if plist.exists() {
                            let _ = Command::new("open").args(["-R", &plist.to_string_lossy()]).spawn();
                        }
                    }
                }
            }
            ItemType::LaunchAgent => {
                if let Some(agent) = self.state.launch_agents.iter().find(|a| a.label == identifier) {
                    if agent.plist_path.exists() {
                        let _ = Command::new("open").args(["-R", &agent.plist_path.to_string_lossy()]).spawn();
                    }
                }
            }
            ItemType::LaunchDaemon => {
                if let Some(daemon) = self.state.launch_daemons.iter().find(|d| d.label == identifier) {
                    if daemon.plist_path.exists() {
                        let _ = Command::new("open").args(["-R", &daemon.plist_path.to_string_lossy()]).spawn();
                    }
                }
            }
            ItemType::SystemExtension => {
                if let Some(ext) = self.state.system_extensions.iter().find(|e| e.identifier == identifier) {
                    if let Some(path) = &ext.path {
                        if path.exists() {
                            let _ = Command::new("open").args(["-R", &path.to_string_lossy()]).spawn();
                            return;
                        }
                    }
                }
                let _ = Command::new("open").arg("/Library/SystemExtensions").spawn();
            }
        }
    }
}

impl Default for TuiApp {
    fn default() -> Self {
        Self::new()
    }
}

/// Unified item representation for the combined table
#[derive(Clone)]
struct UnifiedItem {
    item_type: ItemType,
    name: String,
    identifier: String,
    status: String,
    is_enabled: bool,
    /// Some(n) marks this as a collapsed summary row for n disabled items
    disabled_count: Option<usize>,
}