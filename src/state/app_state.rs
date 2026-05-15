//! Application state management

use crate::error::AppError;
use crate::models::{BackgroundItem, ItemType, LaunchAgent, LaunchDaemon, LoginItem, OpenAtLoginItem, SystemExtension};


/// Loading state for async operations
#[derive(Debug, Clone)]
#[derive(Default)]
pub enum LoadingState {
    #[default]
    Idle,
    Loading,
    Loaded,
    Error(String),
}


/// Currently selected section
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[derive(Default)]
pub enum SelectedSection {
    #[default]
    Sidebar,
    Content,
    Detail,
    Search,
}


/// Scope filter for the item list
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum ScopeFilter {
    #[default]
    All,
    User,
    System,
}


/// Application state
#[derive(Debug)]
pub struct AppState {
    pub current_item_type: ItemType,
    pub selected_section: SelectedSection,
    pub selected_index: usize,
    pub scroll_offset: usize,
    pub search_query: String,

    pub show_login_items: bool,
    pub show_launch_agents: bool,
    pub show_launch_daemons: bool,
    pub show_system_extensions: bool,

    pub login_items: Vec<LoginItem>,
    pub login_items_loading: LoadingState,

    pub open_at_login_items: Vec<OpenAtLoginItem>,
    pub open_at_login_loading: LoadingState,

    pub launch_agents: Vec<LaunchAgent>,
    pub launch_agents_loading: LoadingState,

    pub launch_daemons: Vec<LaunchDaemon>,
    pub launch_daemons_loading: LoadingState,

    pub system_extensions: Vec<SystemExtension>,
    pub system_extensions_loading: LoadingState,

    pub background_items: Vec<BackgroundItem>,
    pub background_items_loading: LoadingState,

    pub error_message: Option<String>,
    pub show_help: bool,

    /// Show system identifiers instead of display names
    pub show_system_names: bool,
    pub scope_filter: ScopeFilter,

    /// Item name awaiting deletion confirmation (ctrl-x pressed once)
    pub pending_delete: Option<String>,

    /// True while load_data is running — shows a refresh indicator
    pub refreshing: bool,

    pub tick: u64,

    /// Active flash ticks when a row is being copied (0 = inactive)
    pub copy_flash_ticks: u8,
}

impl Default for AppState {
    fn default() -> Self {
        Self {
            current_item_type: ItemType::LoginItem,
            selected_section: SelectedSection::Content,
            selected_index: 0,
            scroll_offset: 0,
            search_query: String::new(),
            show_login_items: true,
            show_launch_agents: true,
            show_launch_daemons: true,
            show_system_extensions: true,
            login_items: Vec::new(),
            login_items_loading: LoadingState::Idle,
            open_at_login_items: Vec::new(),
            open_at_login_loading: LoadingState::Idle,
            launch_agents: Vec::new(),
            launch_agents_loading: LoadingState::Idle,
            launch_daemons: Vec::new(),
            launch_daemons_loading: LoadingState::Idle,
            system_extensions: Vec::new(),
            system_extensions_loading: LoadingState::Idle,
            background_items: Vec::new(),
            background_items_loading: LoadingState::Idle,
            error_message: None,
            show_help: false,
            show_system_names: true,
            scope_filter: ScopeFilter::All,
            pending_delete: None,
            refreshing: false,
            tick: 0,
            copy_flash_ticks: 0,
        }
    }
}

impl AppState {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn current_list(&self) -> Vec<String> {
        match self.current_item_type {
            ItemType::LoginItem => self.login_items.iter().map(|i| i.name.clone()).collect(),
            ItemType::OpenAtLogin => self.open_at_login_items.iter().map(|i| i.name.clone()).collect(),
            ItemType::LaunchAgent => self.launch_agents.iter().map(|a| a.name()).collect(),
            ItemType::LaunchDaemon => self.launch_daemons.iter().map(|d| d.label.clone()).collect(),
            ItemType::SystemExtension => self.system_extensions.iter().map(|e| e.identifier.clone()).collect(),
            ItemType::BackgroundItem => self.background_items.iter().map(|b| b.identifier.clone()).collect(),
        }
    }

    pub fn filtered_list(&self) -> Vec<String> {
        let list = self.current_list();
        if self.search_query.is_empty() {
            return list;
        }
        let query = self.search_query.to_lowercase();
        list.into_iter()
            .filter(|item| item.to_lowercase().contains(&query))
            .collect()
    }

    pub fn current_loading(&self) -> &LoadingState {
        match self.current_item_type {
            ItemType::LoginItem => &self.login_items_loading,
            ItemType::OpenAtLogin => &self.open_at_login_loading,
            ItemType::LaunchAgent => &self.launch_agents_loading,
            ItemType::LaunchDaemon => &self.launch_daemons_loading,
            ItemType::SystemExtension => &self.system_extensions_loading,
            ItemType::BackgroundItem => &self.background_items_loading,
        }
    }

    pub fn set_error(&mut self, error: AppError) {
        self.error_message = Some(error.to_string());
    }

    pub fn clear_error(&mut self) {
        self.error_message = None;
    }

    pub fn tick(&mut self) {
        self.tick += 1;
    }

    pub fn move_up(&mut self) {
        if self.selected_index > 0 {
            self.selected_index -= 1;
        }
    }

    pub fn move_down(&mut self) {
        let list_len = self.filtered_list().len();
        if self.selected_index < list_len.saturating_sub(1) {
            self.selected_index += 1;
        }
    }

    pub fn select_prev_section(&mut self) {
        match self.current_item_type {
            ItemType::LoginItem => self.current_item_type = ItemType::BackgroundItem,
            ItemType::OpenAtLogin => self.current_item_type = ItemType::LoginItem,
            ItemType::LaunchAgent => self.current_item_type = ItemType::OpenAtLogin,
            ItemType::LaunchDaemon => self.current_item_type = ItemType::LaunchAgent,
            ItemType::SystemExtension => self.current_item_type = ItemType::LaunchDaemon,
            ItemType::BackgroundItem => self.current_item_type = ItemType::SystemExtension,
        }
        self.selected_index = 0;
    }

    pub fn select_next_section(&mut self) {
        match self.current_item_type {
            ItemType::LoginItem => self.current_item_type = ItemType::OpenAtLogin,
            ItemType::OpenAtLogin => self.current_item_type = ItemType::LaunchAgent,
            ItemType::LaunchAgent => self.current_item_type = ItemType::LaunchDaemon,
            ItemType::LaunchDaemon => self.current_item_type = ItemType::SystemExtension,
            ItemType::SystemExtension => self.current_item_type = ItemType::BackgroundItem,
            ItemType::BackgroundItem => self.current_item_type = ItemType::LoginItem,
        }
        self.selected_index = 0;
    }

    pub fn navigate_to_sidebar(&mut self) { self.selected_section = SelectedSection::Sidebar; }
    pub fn navigate_to_content(&mut self) { self.selected_section = SelectedSection::Content; }
    pub fn navigate_to_detail(&mut self)  { self.selected_section = SelectedSection::Detail; }
    pub fn focus_search(&mut self)        { self.selected_section = SelectedSection::Search; }

    pub fn clear_search(&mut self) {
        self.search_query.clear();
        self.selected_section = SelectedSection::Content;
    }

    pub fn toggle_help(&mut self) {
        self.show_help = !self.show_help;
    }
}
