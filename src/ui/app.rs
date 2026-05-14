//! Main TUI application

use crate::models::ItemType;
use crate::services::{
    LaunchAgentsService, LaunchDaemonsService, LoginItemsService, SystemExtensionsService,
};
use crate::state::{AppState, LoadingState, SelectedSection};
use crate::ui::layouts::Sidebar;
use crate::ui::views::{
    HelpView, LaunchAgentsView, LaunchDaemonsView, LoginItemsView, SystemExtensionsView,
};
use ratatui::{
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Style},
    text::{Line, Span},
    widgets::Paragraph,
    Frame,
};

/// Main TUI application
pub struct TuiApp {
    /// Application state
    pub state: AppState,
}

impl TuiApp {
    /// Create a new TUI application
    pub fn new() -> Self {
        Self {
            state: AppState::new(),
        }
    }

    /// Initialize the application (load data)
    pub fn init(&mut self) {
        self.load_data();
    }

    /// Load data for all sections
    pub fn load_data(&mut self) {
        // Load login items
        self.state.login_items_loading = LoadingState::Loading;
        match LoginItemsService::list() {
            Ok(items) => {
                self.state.login_items = items;
                self.state.login_items_loading = LoadingState::Loaded;
            }
            Err(e) => {
                self.state.login_items_loading = LoadingState::Error(e.to_string());
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
        match self.state.current_item_type {
            ItemType::LoginItem => {
                self.state.login_items_loading = LoadingState::Loading;
                match LoginItemsService::list() {
                    Ok(items) => {
                        self.state.login_items = items;
                        self.state.login_items_loading = LoadingState::Loaded;
                    }
                    Err(e) => {
                        self.state.login_items_loading = LoadingState::Error(e.to_string());
                    }
                }
            }
            ItemType::LaunchAgent => {
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
            }
            ItemType::LaunchDaemon => {
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
            }
            ItemType::SystemExtension => {
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
        }
    }

    /// Render the application
    pub fn render(&self, f: &mut Frame) {
        let area = f.size();

        // Main layout: header, content, footer
        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints([
                Constraint::Length(1),
                Constraint::Min(0),
                Constraint::Length(1),
            ])
            .split(area);

        // Header
        self.render_header(f, chunks[0]);

        // Content area: sidebar + main content
        let content_chunks = Layout::default()
            .direction(Direction::Horizontal)
            .constraints([
                Constraint::Length(20),
                Constraint::Min(0),
            ])
            .split(chunks[1]);

        // Sidebar
        Sidebar::render(f, content_chunks[0], &self.state);

        // Main content based on selected item type
        self.render_content(f, content_chunks[1]);

        // Footer
        self.render_footer(f, chunks[2]);

        // Help overlay
        if self.state.show_help {
            HelpView::render(f, area);
        }
    }

    /// Render the header
    fn render_header(&self, f: &mut Frame, area: Rect) {
        let title = " System Extension Manager ";
        let text = Line::from(vec![
            Span::styled("┌", Style::default().fg(Color::White)),
            Span::styled(title, Style::default().fg(Color::Cyan).add_modifier(ratatui::style::Modifier::BOLD)),
            Span::styled("│", Style::default().fg(Color::White)),
            Span::raw(" "),
            Span::styled("[?] Help", Style::default().fg(Color::DarkGray)),
        ]);

        let para = Paragraph::new(text)
            .style(Style::default().bg(Color::Black));

        f.render_widget(para, area);
    }

    /// Render the main content based on selected item type
    fn render_content(&self, f: &mut Frame, area: Rect) {
        match self.state.current_item_type {
            ItemType::LoginItem => {
                LoginItemsView::render(
                    f,
                    area,
                    &self.state.login_items,
                    &self.state.login_items_loading,
                    self.state.selected_index,
                    &self.state.search_query,
                );
            }
            ItemType::LaunchAgent => {
                LaunchAgentsView::render(
                    f,
                    area,
                    &self.state.launch_agents,
                    &self.state.launch_agents_loading,
                    self.state.selected_index,
                    &self.state.search_query,
                );
            }
            ItemType::LaunchDaemon => {
                LaunchDaemonsView::render(
                    f,
                    area,
                    &self.state.launch_daemons,
                    &self.state.launch_daemons_loading,
                    self.state.selected_index,
                    &self.state.search_query,
                );
            }
            ItemType::SystemExtension => {
                SystemExtensionsView::render(
                    f,
                    area,
                    &self.state.system_extensions,
                    &self.state.system_extensions_loading,
                    self.state.selected_index,
                    &self.state.search_query,
                );
            }
        }
    }

    /// Render the footer
    fn render_footer(&self, f: &mut Frame, area: Rect) {
        let shortcuts = Line::from(vec![
            Span::styled("↑↓", Style::default().fg(Color::Yellow)),
            Span::raw(" Navigate "),
            Span::styled("Enter", Style::default().fg(Color::Yellow)),
            Span::raw(" Select "),
            Span::styled("r", Style::default().fg(Color::Yellow)),
            Span::raw(" Refresh "),
            Span::styled("/ ", Style::default().fg(Color::Yellow)),
            Span::raw(" Search "),
            Span::styled("?", Style::default().fg(Color::Yellow)),
            Span::raw(" Help "),
            Span::styled("q", Style::default().fg(Color::Yellow)),
            Span::raw(" Quit "),
        ]);

        let para = Paragraph::new(shortcuts)
            .style(Style::default().bg(Color::Black));

        f.render_widget(para, area);
    }

    /// Handle keyboard input
    pub fn handle_key(&mut self, key: &str) -> bool {
        match key {
            "q" | "Q" | "ctrl-c" => false, // Quit
            "?" => {
                self.state.toggle_help();
                true
            }
            "r" | "R" => {
                self.refresh_current();
                true
            }
            "k" | "up" => {
                self.state.move_up();
                true
            }
            "j" | "down" => {
                self.state.move_down();
                true
            }
            "h" | "left" => {
                self.state.navigate_to_sidebar();
                true
            }
            "l" | "right" => {
                self.state.navigate_to_content();
                true
            }
            "/" => {
                self.state.focus_search();
                true
            }
            "escape" => {
                if !self.state.search_query.is_empty() {
                    self.state.clear_search();
                } else {
                    self.state.navigate_to_content();
                }
                true
            }
            _ => {
                // Handle search input
                if matches!(self.state.selected_section, SelectedSection::Search) {
                    if key == "backspace" {
                        self.state.search_query.pop();
                    } else if key.len() == 1 {
                        self.state.search_query.push(key.chars().next().unwrap());
                    }
                }
                true
            }
        }
    }
}

impl Default for TuiApp {
    fn default() -> Self {
        Self::new()
    }
}