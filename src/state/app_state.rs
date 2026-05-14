//! Application state management

use crate::error::AppError;
use crate::models::{ItemType, LaunchAgent, LaunchDaemon, LoginItem, SystemExtension};

/// Loading state for async operations
#[derive(Debug, Clone)]
#[derive(Default)]
pub enum LoadingState {
    /// Not currently loading
    #[default]
    Idle,
    /// Currently loading data
    Loading,
    /// Successfully loaded
    Loaded,
    /// Error occurred
    Error(String),
}


/// Currently selected section
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[derive(Default)]
pub enum SelectedSection {
    /// Sidebar navigation
    #[default]
    Sidebar,
    /// Main content area
    Content,
    /// Detail view
    Detail,
    /// Search input
    Search,
}


/// Application state
#[derive(Debug, Default)]
pub struct AppState {
    /// Current item type being viewed
    pub current_item_type: ItemType,
    
    /// Selected section of the UI
    pub selected_section: SelectedSection,
    
    /// Selected index in the current list
    pub selected_index: usize,
    
    /// Search query
    pub search_query: String,
    
    /// Login items
    pub login_items: Vec<LoginItem>,
    pub login_items_loading: LoadingState,
    
    /// Launch agents
    pub launch_agents: Vec<LaunchAgent>,
    pub launch_agents_loading: LoadingState,
    
    /// Launch daemons
    pub launch_daemons: Vec<LaunchDaemon>,
    pub launch_daemons_loading: LoadingState,
    
    /// System extensions
    pub system_extensions: Vec<SystemExtension>,
    pub system_extensions_loading: LoadingState,
    
    /// Global error message
    pub error_message: Option<String>,
    
    /// Show help overlay
    pub show_help: bool,
    
    /// Refresh tick for UI updates
    pub tick: u64,
}

impl AppState {
    /// Create a new AppState
    pub fn new() -> Self {
        Self::default()
    }

    /// Get the current list of items based on selected item type
    pub fn current_list(&self) -> Vec<String> {
        match self.current_item_type {
            ItemType::LoginItem => self.login_items.iter().map(|i| i.name.clone()).collect(),
            ItemType::LaunchAgent => self.launch_agents.iter().map(|a| a.name()).collect(),
            ItemType::LaunchDaemon => self.launch_daemons.iter().map(|d| d.label.clone()).collect(),
            ItemType::SystemExtension => self.system_extensions.iter().map(|e| e.identifier.clone()).collect(),
        }
    }

    /// Get filtered list based on search query
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

    /// Get loading state for current item type
    pub fn current_loading(&self) -> &LoadingState {
        match self.current_item_type {
            ItemType::LoginItem => &self.login_items_loading,
            ItemType::LaunchAgent => &self.launch_agents_loading,
            ItemType::LaunchDaemon => &self.launch_daemons_loading,
            ItemType::SystemExtension => &self.system_extensions_loading,
        }
    }

    /// Set error message
    pub fn set_error(&mut self, error: AppError) {
        self.error_message = Some(error.to_string());
    }

    /// Clear error message
    pub fn clear_error(&mut self) {
        self.error_message = None;
    }

    /// Increment tick counter for UI refresh
    pub fn tick(&mut self) {
        self.tick += 1;
    }

    /// Move selection up
    pub fn move_up(&mut self) {
        if self.selected_index > 0 {
            self.selected_index -= 1;
        }
    }

    /// Move selection down
    pub fn move_down(&mut self) {
        let list_len = self.filtered_list().len();
        if self.selected_index < list_len.saturating_sub(1) {
            self.selected_index += 1;
        }
    }

    /// Select previous item type
    pub fn select_prev_section(&mut self) {
        match self.current_item_type {
            ItemType::LoginItem => self.current_item_type = ItemType::SystemExtension,
            ItemType::LaunchAgent => self.current_item_type = ItemType::LoginItem,
            ItemType::LaunchDaemon => self.current_item_type = ItemType::LaunchAgent,
            ItemType::SystemExtension => self.current_item_type = ItemType::LaunchDaemon,
        }
        self.selected_index = 0;
    }

    /// Select next item type
    pub fn select_next_section(&mut self) {
        match self.current_item_type {
            ItemType::LoginItem => self.current_item_type = ItemType::LaunchAgent,
            ItemType::LaunchAgent => self.current_item_type = ItemType::LaunchDaemon,
            ItemType::LaunchDaemon => self.current_item_type = ItemType::SystemExtension,
            ItemType::SystemExtension => self.current_item_type = ItemType::LoginItem,
        }
        self.selected_index = 0;
    }

    /// Navigate to sidebar
    pub fn navigate_to_sidebar(&mut self) {
        self.selected_section = SelectedSection::Sidebar;
    }

    /// Navigate to content
    pub fn navigate_to_content(&mut self) {
        self.selected_section = SelectedSection::Content;
    }

    /// Navigate to detail
    pub fn navigate_to_detail(&mut self) {
        self.selected_section = SelectedSection::Detail;
    }

    /// Focus search
    pub fn focus_search(&mut self) {
        self.selected_section = SelectedSection::Search;
    }

    /// Clear search
    pub fn clear_search(&mut self) {
        self.search_query.clear();
        self.selected_section = SelectedSection::Content;
    }

    /// Toggle help
    pub fn toggle_help(&mut self) {
        self.show_help = !self.show_help;
    }
}