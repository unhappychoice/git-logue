use anyhow::Result;
use crossterm::{
    event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use ratatui::{
    backend::CrosstermBackend,
    layout::{Constraint, Direction, Layout},
    Frame, Terminal,
};
use std::io;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;
use std::time::{Duration, Instant};

use crate::animation::AnimationEngine;
use crate::git::{CommitMetadata, GitRepository};
use crate::panes::{EditorPane, FileTreePane, StatusBarPane, TerminalPane};

#[derive(Debug, Clone, PartialEq)]
enum UIState {
    Playing,
    WaitingForNext { resume_at: Instant },
    Finished,
}

pub struct UI<'a> {
    state: UIState,
    speed_ms: u64,
    file_tree: FileTreePane,
    editor: EditorPane,
    terminal: TerminalPane,
    status_bar: StatusBarPane,
    engine: AnimationEngine,
    metadata: Option<CommitMetadata>,
    repo: Option<&'a GitRepository>,
    should_exit: Arc<AtomicBool>,
}

impl<'a> UI<'a> {
    pub fn new(speed_ms: u64, _is_commit_specified: bool, repo: Option<&'a GitRepository>) -> Self {
        let should_exit = Arc::new(AtomicBool::new(false));
        Self::setup_signal_handler(should_exit.clone());

        Self {
            state: UIState::Playing,
            speed_ms,
            file_tree: FileTreePane,
            editor: EditorPane,
            terminal: TerminalPane,
            status_bar: StatusBarPane,
            engine: AnimationEngine::new(speed_ms),
            metadata: None,
            repo,
            should_exit,
        }
    }

    fn setup_signal_handler(should_exit: Arc<AtomicBool>) {
        ctrlc::set_handler(move || {
            should_exit.store(true, Ordering::Relaxed);
        })
        .expect("Error setting Ctrl-C handler");
    }

    pub fn load_commit(&mut self, metadata: CommitMetadata) {
        self.engine.load_commit(&metadata);
        self.metadata = Some(metadata);
        self.state = UIState::Playing;
    }

    pub fn run(&mut self) -> Result<()> {
        enable_raw_mode()?;
        let mut stdout = io::stdout();
        execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
        let backend = CrosstermBackend::new(stdout);
        let mut terminal = Terminal::new(backend)?;

        let result = self.run_loop(&mut terminal);

        self.cleanup(&mut terminal)?;

        result
    }

    fn cleanup(&mut self, terminal: &mut Terminal<CrosstermBackend<io::Stdout>>) -> Result<()> {
        disable_raw_mode()?;
        execute!(
            terminal.backend_mut(),
            LeaveAlternateScreen,
            DisableMouseCapture
        )?;
        terminal.show_cursor()?;
        Ok(())
    }

    fn run_loop(&mut self, terminal: &mut Terminal<CrosstermBackend<io::Stdout>>) -> Result<()> {
        loop {
            // Check for Ctrl+C signal
            if self.should_exit.load(Ordering::Relaxed) {
                self.state = UIState::Finished;
            }

            // Update viewport height for scroll calculation
            // Main content area height - status bar (3) - borders (2) = editor height
            let size = terminal.size()?;
            let editor_height = size
                .height
                .saturating_sub(3) // Status bar
                .saturating_sub(2); // Main content borders
            let viewport_height = (editor_height as f32 * 0.8) as usize; // 80% for editor
            let viewport_height = viewport_height.saturating_sub(2); // Editor borders
            self.engine.set_viewport_height(viewport_height);

            // Tick the animation engine
            let needs_redraw = self.engine.tick();

            if needs_redraw {
                terminal.draw(|f| self.render(f))?;
            }

            // Poll for keyboard events at frame rate
            if event::poll(std::time::Duration::from_millis(8))? {
                if let Event::Key(key) = event::read()? {
                    if key.code == KeyCode::Esc {
                        self.state = UIState::Finished;
                    }
                }
            }

            // State machine
            match self.state {
                UIState::Playing => {
                    if self.engine.is_finished() {
                        if self.repo.is_some() {
                            // Random mode - schedule next commit
                            // Wait time proportional to speed (100x the typing speed)
                            self.state = UIState::WaitingForNext {
                                resume_at: Instant::now()
                                    + Duration::from_millis(self.speed_ms * 100),
                            };
                        } else {
                            // Single commit mode - quit
                            self.state = UIState::Finished;
                        }
                    }
                }
                UIState::WaitingForNext { resume_at } => {
                    if Instant::now() >= resume_at {
                        if let Some(repo) = self.repo {
                            match repo.random_commit() {
                                Ok(metadata) => {
                                    self.load_commit(metadata);
                                }
                                Err(_) => {
                                    self.state = UIState::Finished;
                                }
                            }
                        } else {
                            self.state = UIState::Finished;
                        }
                    }
                }
                UIState::Finished => {
                    break;
                }
            }
        }

        Ok(())
    }

    fn render(&self, f: &mut Frame) {
        let size = f.area();

        let main_layout = Layout::default()
            .direction(Direction::Vertical)
            .constraints([
                Constraint::Min(0),    // Main content area
                Constraint::Length(3), // Status bar
            ])
            .split(size);

        let content_layout = Layout::default()
            .direction(Direction::Horizontal)
            .constraints([
                Constraint::Percentage(30), // Left side (file tree)
                Constraint::Percentage(70), // Right side (editor + terminal)
            ])
            .split(main_layout[0]);

        let right_layout = Layout::default()
            .direction(Direction::Vertical)
            .constraints([
                Constraint::Percentage(80), // Editor
                Constraint::Percentage(20), // Terminal
            ])
            .split(content_layout[1]);

        self.file_tree.render(
            f,
            content_layout[0],
            self.metadata.as_ref(),
            self.engine.current_file_index,
        );
        self.editor.render(f, right_layout[0], &self.engine);
        self.terminal.render(f, right_layout[1], &self.engine);
        self.status_bar
            .render(f, main_layout[1], self.metadata.as_ref());
    }
}
