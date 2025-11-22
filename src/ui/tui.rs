//! Terminal user interface using ratatui
//!
//! Provides a full-screen dashboard for real-time sync monitoring,
//! interactive conflict resolution, and repository management.

use crate::core::config::Config;
use crate::core::sync_manager::{FetchResult, PushResult, SyncManager};
use crate::models::Remote;
use crate::ui::formatter::Status;
use crate::ui::sync_monitor::SyncMonitor;
use crate::utils::error::Result;
use crossterm::{
    event::{DisableMouseCapture, EnableMouseCapture, Event, KeyCode, KeyEvent, KeyEventKind},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use ratatui::{
    backend::CrosstermBackend,
    layout::{Constraint, Direction, Layout, Margin, Rect},
    style::{Color, Modifier, Style},
    symbols,
    text::{Line, Span},
    widgets::{
        Block, BorderType, Borders, Clear, Gauge, List, ListItem, ListState, Paragraph, Row, Table,
        TableState, Tabs, Wrap,
    },
    Frame, Terminal,
};
use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use std::time::{Duration, Instant};
use tokio::sync::mpsc;
use tracing::{debug, error, info};

/// TUI application state
pub struct App {
    /// Current active tab
    pub active_tab: Tab,
    /// Configuration
    pub config: Config,
    /// Repository remotes
    pub remotes: Vec<Remote>,
    /// Sync state for each remote
    pub sync_states: HashMap<String, SyncState>,
    /// Selected item in lists
    pub selected_list_item: usize,
    /// Table state for remote list
    pub remote_table_state: TableState,
    /// List state for conflicts
    pub conflict_list_state: ListState,
    /// Current theme
    pub theme: Theme,
    /// Show help overlay
    pub show_help: bool,
    /// Last update time
    pub last_update: Instant,
    /// Is running
    pub running: bool,
    /// Sync monitor
    pub sync_monitor: Option<SyncMonitor>,
    /// Animation frame counter
    pub animation_frame: u64,
    /// Transition progress (0.0-1.0)
    pub transition_progress: f32,
    /// High contrast mode for accessibility
    pub high_contrast: bool,
    /// Reduced motion mode for accessibility
    pub reduced_motion: bool,
    /// Screen reader mode
    pub screen_reader_mode: bool,
}

/// Application tabs
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Tab {
    Dashboard,
    Remotes,
    Conflicts,
    Settings,
    Help,
}

impl Tab {
    pub fn all() -> &'static [Tab] {
        &[Self::Dashboard, Self::Remotes, Self::Conflicts, Self::Settings, Self::Help]
    }
    
    pub fn title(self) -> &'static str {
        match self {
            Self::Dashboard => "Dashboard",
            Self::Remotes => "Remotes",
            Self::Conflicts => "Conflicts",
            Self::Settings => "Settings",
            Self::Help => "Help",
        }
    }
}

/// Sync state for a remote
#[derive(Debug, Clone)]
pub struct SyncState {
    /// Remote name
    pub remote: String,
    /// Current status
    pub status: Status,
    /// Last sync time
    pub last_sync: Option<Instant>,
    /// Last push result
    pub last_push: Option<PushResult>,
    /// Last fetch result
    pub last_fetch: Option<FetchResult>,
    /// Progress (0-100)
    pub progress: u16,
    /// Current operation
    pub operation: String,
    /// Animation frame
    pub animation_frame: u8,
    /// Pulse animation for active operations
    pub pulse_phase: f32,
}

/// Color theme
#[derive(Debug, Clone)]
pub struct Theme {
    /// Primary color
    pub primary: Color,
    /// Secondary color
    pub secondary: Color,
    /// Success color
    pub success: Color,
    /// Warning color
    pub warning: Color,
    /// Error color
    pub error: Color,
    /// Background color
    pub background: Color,
    /// Foreground color
    pub foreground: Color,
    /// Border color
    pub border: Color,
}

impl Default for Theme {
    fn default() -> Self {
        Self {
            primary: Color::Blue,
            secondary: Color::Cyan,
            success: Color::Green,
            warning: Color::Yellow,
            error: Color::Red,
            background: Color::Black,
            foreground: Color::White,
            border: Color::Gray,
        }
    }
}

impl Theme {
    /// Dark theme
    pub fn dark() -> Self {
        Self {
            primary: Color::LightBlue,
            secondary: Color::LightCyan,
            success: Color::LightGreen,
            warning: Color::LightYellow,
            error: Color::LightRed,
            background: Color::Black,
            foreground: Color::White,
            border: Color::DarkGray,
        }
    }
    
    /// Light theme
    pub fn light() -> Self {
        Self {
            primary: Color::Blue,
            secondary: Color::Cyan,
            success: Color::Green,
            warning: Color::Yellow,
            error: Color::Red,
            background: Color::White,
            foreground: Color::Black,
            border: Color::Gray,
        }
    }
}

/// TUI event
#[derive(Debug, Clone)]
pub enum TuiEvent {
    /// Key press
    Key(KeyEvent),
    /// Tick (for animations/updates)
    Tick,
    /// Sync update
    SyncUpdate { remote: String, state: SyncState },
    /// Config reload
    ConfigReload,
}

/// Main TUI application
impl App {
    /// Create new application
    pub fn new(config: Config) -> Result<Self> {
        let remotes = vec![
            Remote {
                name: "github".to_string(),
                provider: crate::models::remote::ProviderType::GitHub,
                username: "user".to_string(),
                api_url: Some("https://api.github.com".to_string()),
                enabled: true,
                use_ssh: false,
                priority: 1,
            },
            Remote {
                name: "gitlab".to_string(),
                provider: crate::models::remote::ProviderType::GitLab,
                username: "user".to_string(),
                api_url: Some("https://gitlab.com/api/v4".to_string()),
                enabled: true,
                use_ssh: false,
                priority: 2,
            },
        ];
        
        let mut sync_states = HashMap::new();
        for remote in &remotes {
            sync_states.insert(remote.name.clone(), SyncState {
                remote: remote.name.clone(),
                status: Status::Info,
                last_sync: None,
                last_push: None,
                last_fetch: None,
                progress: 0,
                operation: "Idle".to_string(),
                animation_frame: 0,
                pulse_phase: 0.0,
            });
        }
        
        let mut app = Self {
            active_tab: Tab::Dashboard,
            config,
            remotes,
            sync_states,
            selected_list_item: 0,
            remote_table_state: TableState::default(),
            conflict_list_state: ListState::default(),
            theme: Theme::default(),
            show_help: false,
            last_update: Instant::now(),
            running: true,
            sync_monitor: None, // Will be set when TUI starts
            animation_frame: 0,
            transition_progress: 0.0,
            high_contrast: false,
            reduced_motion: false,
            screen_reader_mode: false,
        };
        
        // Initialize list states
        app.remote_table_state.select(Some(0));
        app.conflict_list_state.select(Some(0));
        
        Ok(app)
    }
    
    /// Handle key event
    pub fn handle_key(&mut self, key: KeyEvent) {
        match key.code {
            KeyCode::Char('q') | KeyCode::Esc => {
                if self.show_help {
                    self.show_help = false;
                } else {
                    self.running = false;
                }
            }
            KeyCode::Char('h') => self.show_help = !self.show_help,
            KeyCode::Tab => {
                let tabs = Tab::all();
                let current_idx = tabs.iter().position(|&t| t == self.active_tab).unwrap_or(0);
                let next_idx = (current_idx + 1) % tabs.len();
                self.active_tab = tabs[next_idx];
            }
            KeyCode::BackTab => {
                let tabs = Tab::all();
                let current_idx = tabs.iter().position(|&t| t == self.active_tab).unwrap_or(0);
                let prev_idx = if current_idx == 0 { tabs.len() - 1 } else { current_idx - 1 };
                self.active_tab = tabs[prev_idx];
            }
            KeyCode::Up => {
                match self.active_tab {
                    Tab::Remotes => {
                        if let Some(selected) = self.remote_table_state.selected() {
                            if selected > 0 {
                                self.remote_table_state.select(Some(selected - 1));
                            }
                        }
                    }
                    Tab::Conflicts => {
                        if let Some(selected) = self.conflict_list_state.selected() {
                            if selected > 0 {
                                self.conflict_list_state.select(Some(selected - 1));
                            }
                        }
                    }
                    _ => {}
                }
            }
            KeyCode::Down => {
                match self.active_tab {
                    Tab::Remotes => {
                        if let Some(selected) = self.remote_table_state.selected() {
                            if selected < self.remotes.len().saturating_sub(1) {
                                self.remote_table_state.select(Some(selected + 1));
                            }
                        }
                    }
                    Tab::Conflicts => {
                        if let Some(selected) = self.conflict_list_state.selected() {
                            if selected < 5 { // Placeholder conflict count
                                self.conflict_list_state.select(Some(selected + 1));
                            }
                        }
                    }
                    _ => {}
                }
            }
            KeyCode::Enter => {
                match self.active_tab {
                    Tab::Remotes => {
                        // Trigger sync for selected remote
                        if let Some(selected) = self.remote_table_state.selected() {
                            if let Some(remote) = self.remotes.get(selected) {
                                // Get the remote name before triggering sync
                                let remote_name = remote.name.clone();
                                self.trigger_sync(&remote_name);
                            }
                        }
                    }
                    Tab::Conflicts => {
                        // Open conflict resolution
                        self.resolve_conflict();
                    }
                    _ => {}
                }
            }
            KeyCode::Char('r') => {
                // Refresh all data
                self.refresh_data();
            }
            KeyCode::Char('s') => {
                // Sync all remotes
                self.sync_all();
            }
            KeyCode::Char('t') => {
                // Toggle theme
                self.theme = match self.theme.background {
                    Color::Black => Theme::light(),
                    _ => Theme::dark(),
                };
            }
            KeyCode::Char('c') => {
                // Toggle high contrast mode
                self.high_contrast = !self.high_contrast;
                if self.high_contrast {
                    self.theme = Theme {
                        primary: Color::White,
                        secondary: Color::Cyan,
                        success: Color::Green,
                        warning: Color::Yellow,
                        error: Color::Red,
                        background: Color::Black,
                        foreground: Color::White,
                        border: Color::White,
                    };
                } else {
                    self.theme = Theme::default();
                }
            }
            KeyCode::Char('m') => {
                // Toggle reduced motion
                self.reduced_motion = !self.reduced_motion;
            }
            KeyCode::F(1) => {
                // Toggle screen reader mode
                self.screen_reader_mode = !self.screen_reader_mode;
                if self.screen_reader_mode {
                    // Announce current state
                    self.announce_to_screen_reader(&format!(
                        "MultiGit TUI. Current tab: {}. {} remotes configured. {} conflicts unresolved.",
                        self.active_tab.title(),
                        self.remotes.len(),
                        self.sync_states.values().filter(|s| s.status == Status::Error).count()
                    ));
                }
            }
            _ => {}
        }
    }
    
    /// Trigger sync for a remote
    fn trigger_sync(&mut self, remote_name: &str) {
        if let Some(state) = self.sync_states.get_mut(remote_name) {
            state.status = Status::Pending;
            state.operation = "Syncing...".to_string();
            state.progress = 0;
        }
        info!("Triggered sync for remote: {}", remote_name);
        
        // Start sync if monitor is available
        if let Some(ref monitor) = self.sync_monitor {
            let monitor = monitor.clone();
            let remote = remote_name.to_string();
            tokio::spawn(async move {
                if let Err(e) = monitor.sync_remote(&remote).await {
                    error!("Failed to sync remote {}: {}", remote, e);
                }
            });
        }
    }
    
    /// Sync all remotes
    fn sync_all(&mut self) {
        info!("Triggering sync for all remotes");
        
        // Update all states to show syncing
        for (_remote, state) in &mut self.sync_states {
            state.status = Status::Pending;
            state.operation = "Syncing...".to_string();
            state.progress = 0;
        }
        
        // Start sync if monitor is available
        if let Some(ref monitor) = self.sync_monitor {
            let monitor = monitor.clone();
            tokio::spawn(async move {
                if let Err(e) = monitor.sync_all().await {
                    error!("Failed to sync all remotes: {}", e);
                }
            });
        }
    }
    
    /// Resolve conflict
    fn resolve_conflict(&mut self) {
        info!("Opening conflict resolution");
        // This would integrate with the conflict resolver
        // For now, just update the state
        if let Some(state) = self.sync_states.values_mut().next() {
            state.status = Status::Warning;
            state.operation = "Opening resolver...".to_string();
        }
    }
    
    /// Refresh data
    fn refresh_data(&mut self) {
        self.last_update = Instant::now();
        info!("Refreshing TUI data");
        // This would reload config and sync states
    }
    
    /// Update sync state
    pub fn update_sync_state(&mut self, remote: String, state: SyncState) {
        // Create new state with all required fields
        let new_state = SyncState {
            remote: state.remote,
            status: state.status,
            last_sync: state.last_sync,
            last_push: state.last_push,
            last_fetch: state.last_fetch,
            progress: state.progress,
            operation: state.operation,
            animation_frame: state.animation_frame,
            pulse_phase: state.pulse_phase,
        };
        self.sync_states.insert(remote, new_state);
    }
    
    /// Update animations
    fn update_animations(&mut self) {
        // Skip animations if reduced motion is enabled
        if self.reduced_motion {
            return;
        }
        
        self.animation_frame = self.animation_frame.wrapping_add(1);
        
        // Update transition progress
        if self.transition_progress < 1.0 {
            self.transition_progress = (self.transition_progress + 0.1).min(1.0);
        }
        
        // Update sync state animations
        for state in self.sync_states.values_mut() {
            // Animate pulse phase for active operations
            if state.status == Status::Pending {
                state.pulse_phase = (state.pulse_phase + 0.1) % (2.0 * std::f32::consts::PI);
            } else {
                state.pulse_phase = 0.0;
            }
            
            // Animate spinner for active operations
            if state.status == Status::Pending {
                state.animation_frame = (state.animation_frame + 1) % 4;
            } else {
                state.animation_frame = 0;
            }
        }
    }
    
    /// Announce text to screen reader
    fn announce_to_screen_reader(&self, message: &str) {
        if self.screen_reader_mode {
            // In a real implementation, this would use a screen reader API
            // For now, we'll print to stderr which can be captured by screen readers
            eprintln!("SCREEN_READER: {}", message);
        }
    }
    
    /// Get spinner character for animation
    fn get_spinner_char(frame: u8) -> &'static str {
        match frame {
            0 => "⠋",
            1 => "⠙",
            2 => "⠹",
            3 => "⠸",
            _ => "⠋",
        }
    }
    
    /// Get pulsing color for status
    fn get_pulsing_color(&self, base_color: Color, phase: f32) -> Color {
        if phase > 0.0 {
            // Simple pulse effect by cycling between colors
            let intensity = (phase.sin() + 1.0) / 2.0; // 0.0 to 1.0
            match base_color {
                Color::Yellow => if intensity > 0.5 { Color::LightYellow } else { Color::Yellow },
                Color::Blue => if intensity > 0.5 { Color::LightBlue } else { Color::Blue },
                _ => base_color,
            }
        } else {
            base_color
        }
    }
    
    /// Draw the UI
    pub fn draw(&mut self, f: &mut Frame) {
        let area = f.size();
        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints([Constraint::Length(3), Constraint::Min(0), Constraint::Length(3)])
            .split(area);
        
        // Draw tabs
        self.draw_tabs(f, chunks[0]);
        
        // Draw main content
        if self.show_help {
            self.draw_help(f, chunks[1]);
        } else {
            match self.active_tab {
                Tab::Dashboard => self.draw_dashboard(f, chunks[1]),
                Tab::Remotes => self.draw_remotes(f, chunks[1]),
                Tab::Conflicts => self.draw_conflicts(f, chunks[1]),
                Tab::Settings => self.draw_settings(f, chunks[1]),
                Tab::Help => self.draw_help(f, chunks[1]),
            }
        }
        
        // Draw status bar
        self.draw_status_bar(f, chunks[2]);
    }
    
    /// Draw tabs
    fn draw_tabs(&self, f: &mut Frame, area: Rect) {
        let tabs: Vec<Line> = Tab::all()
            .iter()
            .map(|t| {
                let style = if *t == self.active_tab {
                    Style::default()
                        .fg(self.theme.primary)
                        .add_modifier(Modifier::BOLD)
                } else {
                    Style::default()
                        .fg(self.theme.foreground)
                };
                Line::from(Span::styled(t.title(), style))
            })
            .collect();
        
        let tabs_block = Tabs::new(tabs)
            .block(
                Block::default()
                    .borders(Borders::ALL)
                    .border_style(Style::default().fg(self.theme.border))
                    .title("MultiGit TUI"),
            )
            .style(Style::default().fg(self.theme.foreground))
            .highlight_style(
                Style::default()
                    .fg(self.theme.primary)
                    .add_modifier(Modifier::BOLD),
            )
            .divider(" | ");
        
        f.render_widget(tabs_block, area);
    }
    
    /// Draw dashboard
    fn draw_dashboard(&self, f: &mut Frame, area: Rect) {
        let chunks = Layout::default()
            .direction(Direction::Horizontal)
            .constraints([Constraint::Percentage(50), Constraint::Percentage(50)])
            .split(area);
        
        // Left side - Overview
        self.draw_overview(f, chunks[0]);
        
        // Right side - Recent activity
        self.draw_activity(f, chunks[1]);
    }
    
    /// Draw overview panel
    fn draw_overview(&self, f: &mut Frame, area: Rect) {
        let block = Block::default()
            .borders(Borders::ALL)
            .border_style(Style::default().fg(self.theme.border))
            .title("Overview");
        
        let inner = block.inner(area);
        f.render_widget(block, area);
        
        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints([
                Constraint::Length(3),
                Constraint::Length(3),
                Constraint::Min(0),
            ])
            .margin(1)
            .split(inner);
        
        // Repository info
        let repo_info = Paragraph::new("Repository: /current/path\nBranch: main\nRemotes: 3")
            .style(Style::default().fg(self.theme.foreground));
        f.render_widget(repo_info, chunks[0]);
        
        // Quick stats
        let stats = Paragraph::new("✅ Synced: 2 | ⚠️ Pending: 1 | ❌ Failed: 0")
            .style(Style::default().fg(self.theme.foreground));
        f.render_widget(stats, chunks[1]);
        
        // Progress bars for active operations
        let progress_text = if self.sync_states.values().any(|s| s.progress > 0) {
            "Active sync operations..."
        } else {
            "No active operations"
        };
        
        let progress = Paragraph::new(progress_text)
            .style(Style::default().fg(self.theme.foreground))
            .block(Block::default().borders(Borders::ALL).title("Activity"));
        f.render_widget(progress, chunks[2]);
    }
    
    /// Draw activity panel
    fn draw_activity(&self, f: &mut Frame, area: Rect) {
        let block = Block::default()
            .borders(Borders::ALL)
            .border_style(Style::default().fg(self.theme.border))
            .title("Recent Activity");
        
        let inner = block.inner(area);
        f.render_widget(block, area);
        
        let activity_items: Vec<ListItem> = self.sync_states.values().map(|state| {
            let icon = match state.status {
                Status::Success => "✓",
                Status::Error => "✗",
                Status::Warning => "⚠",
                Status::Pending => Self::get_spinner_char(state.animation_frame),
                Status::Info => "ℹ",
            };
            
            let color = self.get_pulsing_color(
                match state.status {
                    Status::Success => self.theme.success,
                    Status::Error => self.theme.error,
                    Status::Warning => self.theme.warning,
                    Status::Pending => self.theme.primary,
                    Status::Info => self.theme.secondary,
                },
                state.pulse_phase
            );
            
            let time_ago = state.last_sync
                .map(|t| format!("{}s ago", t.elapsed().as_secs()))
                .unwrap_or_else(|| "Never".to_string());
            
            ListItem::new(format!(
                "{} {} - {} ({})",
                icon,
                state.remote,
                state.operation,
                time_ago
            )).style(Style::default().fg(color))
        }).collect();
        
        let list = List::new(activity_items)
            .block(Block::default().padding(ratatui::widgets::Padding::uniform(1)))
            .style(Style::default().fg(self.theme.foreground));
        
        f.render_widget(list, inner);
    }
    
    /// Draw remotes table
    fn draw_remotes(&mut self, f: &mut Frame, area: Rect) {
        let block = Block::default()
            .borders(Borders::ALL)
            .border_style(Style::default().fg(self.theme.border))
            .title("Remotes (↑↓ to navigate, Enter to sync)");
        
        let inner = block.inner(area);
        f.render_widget(block, area);
        
        let rows: Vec<Row> = self.remotes.iter().enumerate().map(|(i, remote)| {
            let state = self.sync_states.entry(remote.name.clone()).or_insert_with(|| SyncState {
                remote: remote.name.clone(),
                status: Status::Info,
                last_sync: None,
                last_push: None,
                last_fetch: None,
                progress: 0,
                operation: "Idle".to_string(),
                animation_frame: 0,
                pulse_phase: 0.0,
            });
            
            let style = if self.remote_table_state.selected() == Some(i) {
                Style::default()
                    .fg(self.theme.primary)
                    .add_modifier(Modifier::REVERSED)
            } else {
                Style::default().fg(self.theme.foreground)
            };
            
            Row::new(vec![
                remote.name.clone(),
                format!("{:?}", state.status),
                state.operation.clone(),
                format!("{}%", state.progress),
            ]).style(style)
        }).collect();
        
        let table = Table::new(
            rows,
            &[
                Constraint::Percentage(25),
                Constraint::Percentage(15),
                Constraint::Percentage(35),
                Constraint::Percentage(25),
            ],
        )
        .widths(&[
            Constraint::Percentage(25),
            Constraint::Percentage(25),
            Constraint::Percentage(35),
            Constraint::Percentage(15),
        ])
        .block(Block::default().padding(ratatui::widgets::Padding::uniform(1)))
        .style(Style::default().fg(self.theme.foreground));
        
        f.render_widget(table, inner);
    }
    
    /// Draw conflicts list
    fn draw_conflicts(&mut self, f: &mut Frame, area: Rect) {
        let block = Block::default()
            .borders(Borders::ALL)
            .border_style(Style::default().fg(self.theme.border))
            .title("Conflicts (↑↓ to navigate, Enter to resolve)");
        
        let inner = block.inner(area);
        f.render_widget(block, area);
        
        let conflicts = vec![
            ListItem::new("🔥 main branch - GitHub vs GitLab"),
            ListItem::new("📄 README.md - Local modifications"),
            ListItem::new("📄 src/main.rs - Merge conflict"),
        ];
        
        let list = List::new(conflicts)
            .block(Block::default().padding(ratatui::widgets::Padding::uniform(1)))
            .style(Style::default().fg(self.theme.foreground))
            .highlight_style(
                Style::default()
                    .fg(self.theme.primary)
                    .add_modifier(Modifier::BOLD)
            );
        
        f.render_stateful_widget(list, inner, &mut self.conflict_list_state);
    }
    
    /// Draw settings panel
    fn draw_settings(&self, f: &mut Frame, area: Rect) {
        let block = Block::default()
            .borders(Borders::ALL)
            .border_style(Style::default().fg(self.theme.border))
            .title("Settings");
        
        let inner = block.inner(area);
        f.render_widget(block, area);
        
        let settings_text = vec![
            Line::from("Theme: Press 't' to toggle"),
            Line::from(format!("High Contrast: {} (Press 'c')", if self.high_contrast { "ON" } else { "OFF" })),
            Line::from(format!("Reduced Motion: {} (Press 'm')", if self.reduced_motion { "ON" } else { "OFF" })),
            Line::from(format!("Screen Reader: {} (Press F1)", if self.screen_reader_mode { "ON" } else { "OFF" })),
            Line::from(""),
            Line::from("Auto-sync: Enabled"),
            Line::from("Max parallel: 4"),
            Line::from("Update interval: 5min"),
            Line::from(""),
            Line::from("Press 'r' to refresh data"),
            Line::from("Press 's' to sync all remotes"),
            Line::from("Press 'h' to toggle help"),
            Line::from("Press 'q' or Esc to quit"),
        ];
        
        let settings = Paragraph::new(settings_text)
            .style(Style::default().fg(self.theme.foreground))
            .block(Block::default().padding(ratatui::widgets::Padding::uniform(1)));
        
        f.render_widget(settings, inner);
    }
    
    /// Draw help overlay
    fn draw_help(&self, f: &mut Frame, area: Rect) {
        let block = Block::default()
            .borders(Borders::ALL)
            .border_style(Style::default().fg(self.theme.primary))
            .title("Help");
        
        let inner = block.inner(area);
        f.render_widget(block, area);
        
        let help_text = vec![
            Line::from("🎮 MultiGit TUI Keyboard Shortcuts"),
            Line::from(""),
            Line::from("Navigation:"),
            Line::from("  Tab/Shift+Tab - Switch tabs"),
            Line::from("  ↑/↓ - Navigate lists"),
            Line::from("  Enter - Select/Execute"),
            Line::from(""),
            Line::from("Actions:"),
            Line::from("  r - Refresh data"),
            Line::from("  s - Sync all remotes"),
            Line::from("  t - Toggle theme"),
            Line::from("  c - Toggle high contrast"),
            Line::from("  m - Toggle reduced motion"),
            Line::from("  F1 - Toggle screen reader mode"),
            Line::from("  h - Toggle this help"),
            Line::from("  q/Esc - Quit/Close dialog"),
            Line::from(""),
            Line::from("Tabs:"),
            Line::from("  Dashboard - Overview & activity"),
            Line::from("  Remotes - Manage remote repositories"),
            Line::from("  Conflicts - Resolve merge conflicts"),
            Line::from("  Settings - Configure preferences"),
            Line::from("  Help - Show this help"),
        ];
        
        let help = Paragraph::new(help_text)
            .style(Style::default().fg(self.theme.foreground))
            .block(Block::default().padding(ratatui::widgets::Padding::uniform(1)))
            .wrap(Wrap { trim: true });
        
        f.render_widget(help, inner);
    }
    
    /// Draw status bar
    fn draw_status_bar(&self, f: &mut Frame, area: Rect) {
        let status_text = format!(
            "Last update: {}s ago | Theme: {} | Remotes: {} | Press 'h' for help",
            self.last_update.elapsed().as_secs(),
            if self.theme.background == Color::Black { "Dark" } else { "Light" },
            self.remotes.len()
        );
        
        let status_bar = Paragraph::new(status_text)
            .style(Style::default()
                .fg(self.theme.foreground)
                .bg(self.theme.background))
            .block(Block::default()
                .borders(Borders::TOP)
                .border_style(Style::default().fg(self.theme.border)));
        
        f.render_widget(status_bar, area);
    }
}

/// Run the TUI application
pub async fn run_tui(config: Config) -> Result<()> {
    info!("Starting MultiGit TUI");
    
    // Setup terminal
    enable_raw_mode()?;
    let mut stdout = std::io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;
    
    // Setup event handling
    let (tx, mut rx) = tokio::sync::mpsc::unbounded_channel();
    
    // Create application
    let mut app = App::new(config.clone())?;
    
    // Create and start sync monitor
    let sync_monitor = SyncMonitor::new(config, tx.clone())?;
    sync_monitor.start().await?;
    app.sync_monitor = Some(sync_monitor);
    
    // Spawn event handler
    let event_tx = tx.clone();
    tokio::spawn(async move {
        let mut ticker = tokio::time::interval(Duration::from_millis(250));
        
        loop {
            tokio::select! {
                _ = ticker.tick() => {
                    if event_tx.send(TuiEvent::Tick).is_err() {
                        break;
                    }
                }
                event_result = tokio::task::spawn_blocking(|| crossterm::event::read()) => {
                    match event_result {
                        Ok(Ok(Event::Key(key))) => {
                            if key.kind == KeyEventKind::Press {
                                if event_tx.send(TuiEvent::Key(key)).is_err() {
                                    break;
                                }
                            }
                        }
                        Ok(Ok(Event::Resize(_, _))) => {
                            // Terminal resized, will trigger redraw
                        }
                        _ => {}
                    }
                }
            }
        }
    });
    
    // Main loop
    while app.running {
        // Handle events
        while let Ok(event) = rx.try_recv() {
            match event {
                TuiEvent::Key(key) => app.handle_key(key),
                TuiEvent::Tick => {
                    // Update animations
                    app.update_animations();
                }
                TuiEvent::SyncUpdate { remote, state } => {
                    app.update_sync_state(remote, state);
                }
                TuiEvent::ConfigReload => {
                    app.refresh_data();
                }
            }
        }
        
        // Draw UI
        terminal.draw(|f| app.draw(f))?;
        
        // Small delay to prevent high CPU usage
        tokio::time::sleep(Duration::from_millis(16)).await; // ~60 FPS
    }
    
    // Stop sync monitor
    if let Some(ref monitor) = app.sync_monitor {
        monitor.stop();
    }
    
    // Cleanup
    disable_raw_mode()?;
    execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen,
        DisableMouseCapture
    )?;
    terminal.show_cursor()?;
    
    info!("MultiGit TUI closed");
    Ok(())
}

/// Start the TUI dashboard
pub fn start_dashboard(config: Config) -> Result<()> {
    let rt = tokio::runtime::Runtime::new()?;
    rt.block_on(run_tui(config))
}
