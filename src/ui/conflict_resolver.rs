//! Interactive conflict resolution UI
//!
//! Provides a visual interface for resolving merge conflicts.

use crate::core::config::Config;
use crate::ui::formatter::{colors, Status};
use crate::utils::error::Result;
use crossterm::{
    event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode, KeyEventKind},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use ratatui::{
    backend::CrosstermBackend,
    layout::{Constraint, Direction, Layout, Margin, Rect},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{
        Block, BorderType, Borders, Clear, List, ListItem, ListState, Paragraph, Tabs, Wrap,
    },
    Frame, Terminal,
};
use std::collections::HashMap;
use std::time::{Duration, Instant};
use tokio::sync::mpsc;
use tracing::{debug, info, warn};

/// Conflict resolution state
#[derive(Debug, Clone)]
pub struct Conflict {
    /// File path
    pub file: String,
    /// Conflict type
    pub conflict_type: ConflictType,
    /// Local version
    pub local_content: String,
    /// Remote version
    pub remote_content: String,
    /// Base version
    pub base_content: Option<String>,
    /// Current resolution choice
    pub resolution: ResolutionChoice,
    /// Resolved content
    pub resolved_content: Option<String>,
}

/// Types of conflicts
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ConflictType {
    /// Content conflict
    Content,
    /// Deletion conflict
    Deletion,
    /// Addition conflict
    Addition,
    /// Rename conflict
    Rename,
}

/// Resolution choices
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ResolutionChoice {
    /// Unresolved
    Unresolved,
    /// Choose local version
    Local,
    /// Choose remote version
    Remote,
    /// Choose base version
    Base,
    /// Manual merge
    Manual,
}

/// Conflict resolution application
pub struct ConflictResolver {
    /// Configuration
    config: Config,
    /// List of conflicts
    conflicts: Vec<Conflict>,
    /// Currently selected conflict
    selected_conflict: usize,
    /// List state
    conflict_list_state: ListState,
    /// Current view mode
    view_mode: ViewMode,
    /// Manual editor content
    manual_editor_content: String,
    /// Editor cursor position
    editor_cursor: usize,
    /// Is running
    running: bool,
    /// Theme
    theme: crate::ui::tui::Theme,
}

/// View modes for conflict resolution
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ViewMode {
    /// Conflict list
    ConflictList,
    /// Diff view
    DiffView,
    /// Manual editor
    ManualEditor,
    /// Resolution preview
    ResolutionPreview,
}

impl ConflictResolver {
    /// Create new conflict resolver
    pub fn new(config: Config) -> Result<Self> {
        let conflicts = Self::detect_conflicts()?;
        
        Ok(Self {
            config,
            conflicts,
            selected_conflict: 0,
            conflict_list_state: ListState::default(),
            view_mode: ViewMode::ConflictList,
            manual_editor_content: String::new(),
            editor_cursor: 0,
            running: true,
            theme: crate::ui::tui::Theme::default(),
        })
    }
    
    /// Detect conflicts in the repository
    fn detect_conflicts() -> Result<Vec<Conflict>> {
        // This would use git to detect actual conflicts
        // For now, return demo conflicts
        Ok(vec![
            Conflict {
                file: "src/main.rs".to_string(),
                conflict_type: ConflictType::Content,
                local_content: "fn main() {\n    println!(\"Local version\");\n}".to_string(),
                remote_content: "fn main() {\n    println!(\"Remote version\");\n}".to_string(),
                base_content: Some("fn main() {\n    println!(\"Base version\");\n}".to_string()),
                resolution: ResolutionChoice::Unresolved,
                resolved_content: None,
            },
            Conflict {
                file: "README.md".to_string(),
                conflict_type: ConflictType::Content,
                local_content: "# My Project\nLocal description".to_string(),
                remote_content: "# My Project\nRemote description".to_string(),
                base_content: Some("# My Project\nBase description".to_string()),
                resolution: ResolutionChoice::Unresolved,
                resolved_content: None,
            },
        ])
    }
    
    /// Handle key event
    pub fn handle_key(&mut self, key: crossterm::event::KeyEvent) {
        match key.code {
            KeyCode::Char('q') | KeyCode::Esc => {
                if self.view_mode != ViewMode::ConflictList {
                    self.view_mode = ViewMode::ConflictList;
                } else {
                    self.running = false;
                }
            }
            KeyCode::Up => {
                match self.view_mode {
                    ViewMode::ConflictList => {
                        if self.selected_conflict > 0 {
                            self.selected_conflict -= 1;
                            self.conflict_list_state.select(Some(self.selected_conflict));
                        }
                    }
                    ViewMode::ManualEditor => {
                        // Move cursor up in editor
                        self.move_editor_cursor_up();
                    }
                    _ => {}
                }
            }
            KeyCode::Down => {
                match self.view_mode {
                    ViewMode::ConflictList => {
                        if self.selected_conflict < self.conflicts.len().saturating_sub(1) {
                            self.selected_conflict += 1;
                            self.conflict_list_state.select(Some(self.selected_conflict));
                        }
                    }
                    ViewMode::ManualEditor => {
                        // Move cursor down in editor
                        self.move_editor_cursor_down();
                    }
                    _ => {}
                }
            }
            KeyCode::Left => {
                if self.view_mode == ViewMode::ManualEditor {
                    self.move_editor_cursor_left();
                }
            }
            KeyCode::Right => {
                if self.view_mode == ViewMode::ManualEditor {
                    self.move_editor_cursor_right();
                }
            }
            KeyCode::Enter => {
                match self.view_mode {
                    ViewMode::ConflictList => {
                        self.view_mode = ViewMode::DiffView;
                    }
                    ViewMode::DiffView => {
                        // Apply resolution based on current choice
                        self.apply_current_resolution();
                    }
                    _ => {}
                }
            }
            KeyCode::Char('1') => {
                // Choose local version
                if self.view_mode == ViewMode::DiffView {
                    self.set_resolution(ResolutionChoice::Local);
                }
            }
            KeyCode::Char('2') => {
                // Choose remote version
                if self.view_mode == ViewMode::DiffView {
                    self.set_resolution(ResolutionChoice::Remote);
                }
            }
            KeyCode::Char('3') => {
                // Choose base version
                if self.view_mode == ViewMode::DiffView {
                    self.set_resolution(ResolutionChoice::Base);
                }
            }
            KeyCode::Char('4') => {
                // Manual edit
                if self.view_mode == ViewMode::DiffView {
                    self.view_mode = ViewMode::ManualEditor;
                    self.manual_editor_content = self.get_current_conflict()
                        .map(|c| c.local_content.clone())
                        .unwrap_or_default();
                }
            }
            KeyCode::Char('m') => {
                // Toggle view mode
                self.view_mode = match self.view_mode {
                    ViewMode::ConflictList => ViewMode::DiffView,
                    ViewMode::DiffView => ViewMode::ResolutionPreview,
                    ViewMode::ResolutionPreview => ViewMode::ConflictList,
                    ViewMode::ManualEditor => ViewMode::ConflictList,
                };
            }
            KeyCode::Backspace | KeyCode::Delete => {
                if self.view_mode == ViewMode::ManualEditor {
                    self.delete_editor_char();
                }
            }
            KeyCode::Char(c) => {
                if self.view_mode == ViewMode::ManualEditor {
                    self.insert_editor_char(c);
                }
            }
            _ => {}
        }
    }
    
    /// Set resolution for current conflict
    fn set_resolution(&mut self, resolution: ResolutionChoice) {
        let current_content = self.manual_editor_content.clone();
        if let Some(conflict) = self.get_current_conflict_mut() {
            conflict.resolution = resolution.clone();
            
            // Auto-resolve if not manual
            match resolution {
                ResolutionChoice::Local => {
                    conflict.resolved_content = Some(conflict.local_content.clone());
                }
                ResolutionChoice::Remote => {
                    conflict.resolved_content = Some(conflict.remote_content.clone());
                }
                ResolutionChoice::Base => {
                    conflict.resolved_content = conflict.base_content.clone();
                }
                ResolutionChoice::Manual => {
                    // Keep manual editor content
                    conflict.resolved_content = Some(current_content);
                }
                ResolutionChoice::Unresolved => {
                    conflict.resolved_content = None;
                }
            }
        }
    }
    
    /// Apply current resolution
    fn apply_current_resolution(&mut self) {
        if let Some(conflict) = self.get_current_conflict() {
            if conflict.resolution != ResolutionChoice::Unresolved {
                info!("Applied resolution for conflict: {}", conflict.file);
                // Move to next conflict
                if self.selected_conflict < self.conflicts.len().saturating_sub(1) {
                    self.selected_conflict += 1;
                    self.conflict_list_state.select(Some(self.selected_conflict));
                }
            }
        }
    }
    
    /// Get current conflict
    fn get_current_conflict(&self) -> Option<&Conflict> {
        self.conflicts.get(self.selected_conflict)
    }
    
    /// Get current conflict as mutable
    fn get_current_conflict_mut(&mut self) -> Option<&mut Conflict> {
        self.conflicts.get_mut(self.selected_conflict)
    }
    
    /// Move editor cursor up
    fn move_editor_cursor_up(&mut self) {
        let lines: Vec<&str> = self.manual_editor_content.lines().collect();
        let current_line = self.manual_editor_content[..self.editor_cursor]
            .lines()
            .count()
            .saturating_sub(1);
        
        if current_line > 0 {
            // Find position of previous line
            let mut new_cursor = 0;
            for (i, line) in lines.iter().enumerate() {
                if i == current_line - 1 {
                    break;
                }
                new_cursor += line.len() + 1; // +1 for newline
            }
            self.editor_cursor = new_cursor;
        }
    }
    
    /// Move editor cursor down
    fn move_editor_cursor_down(&mut self) {
        let lines: Vec<&str> = self.manual_editor_content.lines().collect();
        let current_line = self.manual_editor_content[..self.editor_cursor]
            .lines()
            .count()
            .saturating_sub(1);
        
        if current_line < lines.len().saturating_sub(1) {
            // Find position of next line
            let mut new_cursor = 0;
            for (i, line) in lines.iter().enumerate() {
                if i <= current_line {
                    new_cursor += line.len() + 1; // +1 for newline
                } else {
                    break;
                }
            }
            self.editor_cursor = new_cursor.min(self.manual_editor_content.len());
        }
    }
    
    /// Move editor cursor left
    fn move_editor_cursor_left(&mut self) {
        if self.editor_cursor > 0 {
            self.editor_cursor -= 1;
        }
    }
    
    /// Move editor cursor right
    fn move_editor_cursor_right(&mut self) {
        if self.editor_cursor < self.manual_editor_content.len() {
            self.editor_cursor += 1;
        }
    }
    
    /// Insert character in editor
    fn insert_editor_char(&mut self, c: char) {
        self.manual_editor_content.insert(self.editor_cursor, c);
        self.editor_cursor += 1;
    }
    
    /// Delete character in editor
    fn delete_editor_char(&mut self) {
        if self.editor_cursor > 0 {
            self.manual_editor_content.remove(self.editor_cursor - 1);
            self.editor_cursor -= 1;
        }
    }
    
    /// Get all unresolved conflicts
    pub fn unresolved_conflicts(&self) -> usize {
        self.conflicts.iter()
            .filter(|c| c.resolution == ResolutionChoice::Unresolved)
            .count()
    }
    
    /// Get all resolved conflicts
    pub fn resolved_conflicts(&self) -> usize {
        self.conflicts.iter()
            .filter(|c| c.resolution != ResolutionChoice::Unresolved)
            .count()
    }
    
    /// Draw the UI
    pub fn draw(&mut self, f: &mut Frame) {
        let area = f.size();
        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints([Constraint::Length(3), Constraint::Min(0), Constraint::Length(3)])
            .split(area);
        
        // Draw title bar
        self.draw_title_bar(f, chunks[0]);
        
        // Draw main content
        match self.view_mode {
            ViewMode::ConflictList => self.draw_conflict_list(f, chunks[1]),
            ViewMode::DiffView => self.draw_diff_view(f, chunks[1]),
            ViewMode::ManualEditor => self.draw_manual_editor(f, chunks[1]),
            ViewMode::ResolutionPreview => self.draw_resolution_preview(f, chunks[1]),
        }
        
        // Draw status bar
        self.draw_status_bar(f, chunks[2]);
    }
    
    /// Draw title bar
    fn draw_title_bar(&self, f: &mut Frame, area: Rect) {
        let title = format!("Conflict Resolver - {} conflicts", self.conflicts.len());
        let title_bar = Paragraph::new(title)
            .style(Style::default()
                .fg(self.theme.foreground)
                .bg(self.theme.background))
            .block(Block::default()
                .borders(Borders::ALL)
                .border_style(Style::default().fg(self.theme.border))
                .title("MultiGit Conflict Resolution"));
        
        f.render_widget(title_bar, area);
    }
    
    /// Draw conflict list
    fn draw_conflict_list(&mut self, f: &mut Frame, area: Rect) {
        let block = Block::default()
            .borders(Borders::ALL)
            .border_style(Style::default().fg(self.theme.border))
            .title("Conflicts (↑↓ to navigate, Enter to resolve)");
        
        let inner = block.inner(area);
        f.render_widget(block, area);
        
        let conflict_items: Vec<ListItem> = self.conflicts.iter().enumerate().map(|(i, conflict)| {
            let status = match conflict.resolution {
                ResolutionChoice::Unresolved => "🔥",
                ResolutionChoice::Local => "✓",
                ResolutionChoice::Remote => "✓",
                ResolutionChoice::Base => "✓",
                ResolutionChoice::Manual => "✏️",
            };
            
            let style = if i == self.selected_conflict {
                Style::default()
                    .fg(self.theme.primary)
                    .add_modifier(Modifier::BOLD)
            } else {
                Style::default().fg(self.theme.foreground)
            };
            
            ListItem::new(format!("{} {} - {:?}", status, conflict.file, conflict.resolution))
                .style(style)
        }).collect();
        
        let list = List::new(conflict_items)
            .block(Block::default().padding(ratatui::widgets::Padding::uniform(1)))
            .style(Style::default().fg(self.theme.foreground))
            .highlight_style(
                Style::default()
                    .fg(self.theme.primary)
                    .add_modifier(Modifier::REVERSED)
            );
        
        f.render_stateful_widget(list, inner, &mut self.conflict_list_state);
    }
    
    /// Draw diff view
    fn draw_diff_view(&self, f: &mut Frame, area: Rect) {
        let chunks = Layout::default()
            .direction(Direction::Horizontal)
            .constraints([Constraint::Percentage(50), Constraint::Percentage(50)])
            .split(area);
        
        // Left side - Local version
        self.draw_version_panel(f, chunks[0], "Local (Press 1)", 
            self.get_current_conflict().map(|c| c.local_content.as_str()).unwrap_or("No conflict selected"));
        
        // Right side - Remote version
        self.draw_version_panel(f, chunks[1], "Remote (Press 2)", 
            self.get_current_conflict().map(|c| c.remote_content.as_str()).unwrap_or("No conflict selected"));
    }
    
    /// Draw version panel
    fn draw_version_panel(&self, f: &mut Frame, area: Rect, title: &str, content: &str) {
        let block = Block::default()
            .borders(Borders::ALL)
            .border_style(Style::default().fg(self.theme.border))
            .title(title);
        
        let inner = block.inner(area);
        f.render_widget(block, area);
        
        let paragraph = Paragraph::new(content)
            .style(Style::default().fg(self.theme.foreground))
            .block(Block::default().padding(ratatui::widgets::Padding::uniform(1)))
            .wrap(Wrap { trim: true });
        
        f.render_widget(paragraph, inner);
    }
    
    /// Draw manual editor
    fn draw_manual_editor(&self, f: &mut Frame, area: Rect) {
        let block = Block::default()
            .borders(Borders::ALL)
            .border_style(Style::default().fg(self.theme.border))
            .title("Manual Editor (Esc to exit)");
        
        let inner = block.inner(area);
        f.render_widget(block, area);
        
        let paragraph = Paragraph::new(self.manual_editor_content.as_str())
            .style(Style::default().fg(self.theme.foreground))
            .block(Block::default().padding(ratatui::widgets::Padding::uniform(1)))
            .wrap(Wrap { trim: true });
        
        f.render_widget(paragraph, inner);
    }
    
    /// Draw resolution preview
    fn draw_resolution_preview(&self, f: &mut Frame, area: Rect) {
        let block = Block::default()
            .borders(Borders::ALL)
            .border_style(Style::default().fg(self.theme.border))
            .title("Resolution Preview");
        
        let inner = block.inner(area);
        f.render_widget(block, area);
        
        let content = if let Some(conflict) = self.get_current_conflict() {
            match &conflict.resolved_content {
                Some(resolved) => resolved.as_str(),
                None => "No resolution selected",
            }
        } else {
            "No conflict selected"
        };
        
        let paragraph = Paragraph::new(content)
            .style(Style::default().fg(self.theme.foreground))
            .block(Block::default().padding(ratatui::widgets::Padding::uniform(1)))
            .wrap(Wrap { trim: true });
        
        f.render_widget(paragraph, inner);
    }
    
    /// Draw status bar
    fn draw_status_bar(&self, f: &mut Frame, area: Rect) {
        let status_text = format!(
            "Unresolved: {} | Resolved: {} | Mode: {:?} | Press 'm' to toggle view | 'q' to quit",
            self.unresolved_conflicts(),
            self.resolved_conflicts(),
            self.view_mode
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
    
    /// Apply all resolutions
    pub fn apply_all_resolutions(&self) -> Result<()> {
        info!("Applying {} conflict resolutions", self.resolved_conflicts());
        
        for conflict in &self.conflicts {
            if conflict.resolution != ResolutionChoice::Unresolved {
                if let Some(resolved_content) = &conflict.resolved_content {
                    // Write resolved content to file
                    std::fs::write(&conflict.file, resolved_content)?;
                    info!("Resolved conflict in file: {}", conflict.file);
                }
            }
        }
        
        Ok(())
    }
}

/// Run the conflict resolver
pub async fn run_conflict_resolver(config: Config) -> Result<()> {
    info!("Starting conflict resolver");
    
    // Setup terminal
    enable_raw_mode()?;
    let mut stdout = std::io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;
    
    // Create resolver
    let mut resolver = ConflictResolver::new(config)?;
    
    // Setup event handling
    let (tx, mut rx) = mpsc::channel(100);
    
    // Spawn event handler
    let event_tx = tx.clone();
    tokio::spawn(async move {
        loop {
            tokio::select! {
                result = tokio::task::spawn_blocking(|| crossterm::event::read()) => {
                    match result {
                        Ok(Ok(Event::Key(key))) => {
                            if key.kind == KeyEventKind::Press {
                                if event_tx.send(key).await.is_err() {
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
    while resolver.running {
        // Handle events
        while let Ok(key) = rx.try_recv() {
            resolver.handle_key(key);
        }
        
        // Draw UI
        terminal.draw(|f| resolver.draw(f))?;
        
        // Small delay to prevent high CPU usage
        tokio::time::sleep(Duration::from_millis(16)).await; // ~60 FPS
    }
    
    // Apply resolutions
    resolver.apply_all_resolutions()?;
    
    // Cleanup
    disable_raw_mode()?;
    execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen,
        DisableMouseCapture
    )?;
    terminal.show_cursor()?;
    
    info!("Conflict resolver closed");
    Ok(())
}
