mod caesar;
mod cipher;
mod vigenere;
/*
mod digraph_block;
mod affine;
mod atbash;
mod hill;
mod modular;
mod one_time_pad;
mod playfair;
mod polyalphabetic;
mod rail_fence;
mod transposition;
mod vigenere_kasiski;
*/

use crate::caesar::Caesar;
use crate::cipher::Cipher;
use crate::vigenere::Vigenere;
/*
use crate::affine::Affine;
use crate::atbash::Atbash;
use crate::digraph_block::DigraphBlock;
use crate::hill::Hill;
use crate::modular::Modular;
use crate::one_time_pad::OneTimePad;
use crate::playfair::Playfair;
use crate::polyalphabetic::Polyalphabetic;
use crate::rail_fence::RailFence;
use crate::transposition::Transposition;
use crate::vigenere_kasiski::VigenereKasiski;
*/

use arboard::Clipboard;
use crossterm::{
    event::{
        self, DisableMouseCapture, EnableMouseCapture, Event, KeyEvent, KeyEventKind, KeyModifiers,
    },
    execute,
    terminal::{EnterAlternateScreen, LeaveAlternateScreen, disable_raw_mode, enable_raw_mode},
};
use ratatui::{
    Terminal,
    backend::CrosstermBackend,
    layout::{Constraint, Direction, Layout},
    style::{Color, Style},
    widgets::ListState,
    widgets::{Block, Borders, List, ListItem, Paragraph, Wrap},
};
use std::{
    io,
    time::{Duration, Instant},
};

enum AppState {
    InputMode,
    AlgorithmSelectionMode,
    ParameterInputMode,
    ResultMode,
}

struct App {
    input_text: String,
    algorithms: Vec<Box<dyn Cipher>>,
    list_state: ListState,
    param_text: String,
    result_text: String,
    state: AppState,
    key_visible: bool,
    status_text: String,
    status_updated: Option<Instant>,
}

impl App {
    fn new() -> Self {
        Self {
            input_text: String::new(),
            algorithms: vec![
                Box::new(Vigenere),
                Box::new(Caesar),
                /*
                Box::new(Modular),
                Box::new(Atbash),
                Box::new(VigenereKasiski),
                Box::new(Polyalphabetic),
                Box::new(Playfair),
                Box::new(DigraphBlock),
                Box::new(RailFence),
                Box::new(Transposition),
                Box::new(Affine),
                Box::new(Hill),
                Box::new(OneTimePad),
                */
            ],
            list_state: ListState::default(),
            param_text: String::new(),
            result_text: String::new(),
            state: AppState::InputMode,
            key_visible: true,
            status_text: String::new(),
            status_updated: None,
        }
    }

    fn run(&mut self) -> io::Result<()> {
        enable_raw_mode()?;
        let mut stdout = io::stdout();
        execute!(stdout, EnableMouseCapture, EnterAlternateScreen)?;
        let backend = CrosstermBackend::new(stdout);
        let mut terminal = Terminal::new(backend)?;

        let res = self.main_loop(&mut terminal);

        drop(terminal);

        disable_raw_mode()?;
        let mut stdout = io::stdout();
        execute!(stdout, LeaveAlternateScreen, DisableMouseCapture)?;
        res
    }

    fn main_loop(
        &mut self,
        terminal: &mut Terminal<CrosstermBackend<std::io::Stdout>>,
    ) -> io::Result<()> {
        loop {
            terminal.draw(|f| {
                self.draw(f);
            })?;

            if let Some(updated) = self.status_updated {
                if updated.elapsed() >= Duration::from_secs(1) {
                    self.status_text.clear();
                    self.status_updated = None;
                }
            }

            if event::poll(std::time::Duration::from_millis(16))? {
                if let Event::Key(key_event) = event::read()? {
                    if key_event.code == event::KeyCode::Esc {
                        return Ok(());
                    }
                    self.handle_key(key_event);
                }
            }
        }
    }

    fn handle_key(&mut self, key: KeyEvent) {
        if key.kind == KeyEventKind::Press && key.modifiers.contains(KeyModifiers::CONTROL) {
            match key.code {
                event::KeyCode::Char('v') | event::KeyCode::Char('V') => {
                    if let Some(content) = self.read_clipboard() {
                        match self.state {
                            AppState::InputMode => self.input_text.push_str(&content),
                            AppState::ParameterInputMode => self.param_text.push_str(&content),
                            _ => {}
                        }
                        self.status_text = "Pasted clipboard text".to_string();
                    } else {
                        self.status_text = "Paste failed".to_string();
                    }
                    self.status_updated = Some(Instant::now());
                    return;
                }
                event::KeyCode::Char('c') | event::KeyCode::Char('C') => {
                    if matches!(self.state, AppState::ResultMode) && !self.result_text.is_empty() {
                        if self.copy_to_clipboard(&self.result_text) {
                            self.status_text = "Result copied to clipboard".to_string();
                        } else {
                            self.status_text = "Copy failed".to_string();
                        }
                        self.status_updated = Some(Instant::now());
                    }
                    return;
                }
                _ => {}
            }
        }

        match self.state {
            AppState::InputMode => {
                if key.kind == KeyEventKind::Press {
                    if key.code == event::KeyCode::Enter {
                        self.state = AppState::AlgorithmSelectionMode;
                        self.list_state.select(Some(0));
                    } else if let event::KeyCode::Char(c) = key.code {
                        self.input_text.push(c);
                    } else if key.code == event::KeyCode::Backspace {
                        self.input_text.pop();
                    }
                }
            }
            AppState::AlgorithmSelectionMode => {
                if key.kind != KeyEventKind::Press {
                    return;
                }
                match key.code {
                    event::KeyCode::Up => {
                        let i = self.list_state.selected().unwrap_or(0);
                        let next = if i > 0 {
                            i - 1
                        } else {
                            self.algorithms.len() - 1
                        };
                        self.list_state.select(Some(next));
                    }
                    event::KeyCode::Down => {
                        let i = self.list_state.selected().unwrap_or(0);
                        let next = if i < self.algorithms.len() - 1 {
                            i + 1
                        } else {
                            0
                        };
                        self.list_state.select(Some(next));
                    }
                    event::KeyCode::Enter => {
                        let idx = self.list_state.selected().unwrap_or(0);
                        let alg = &self.algorithms[idx];
                        if alg.requires_key() {
                            self.state = AppState::ParameterInputMode;
                            self.param_text.clear();
                        } else {
                            self.result_text = alg.encrypt(&self.input_text, "");
                            self.state = AppState::ResultMode;
                        }
                    }
                    _ => {}
                }
            }
            AppState::ParameterInputMode => {
                if key.kind != KeyEventKind::Press {
                    return;
                }
                if key.code == event::KeyCode::Enter {
                    let idx = self.list_state.selected().unwrap_or(0);
                    let alg = &self.algorithms[idx];
                    self.result_text = alg.encrypt(&self.input_text, &self.param_text);
                    self.state = AppState::ResultMode;
                } else if let event::KeyCode::Char(c) = key.code {
                    self.param_text.push(c);
                } else if key.code == event::KeyCode::Backspace {
                    self.param_text.pop();
                }
            }
            AppState::ResultMode => {
                if key.kind != KeyEventKind::Press {
                    return;
                }
                if key.code == event::KeyCode::Enter || key.code == event::KeyCode::Esc {
                    self.state = AppState::InputMode;
                    self.result_text.clear();
                } else if let event::KeyCode::Char(c) = key.code {
                    if c.to_uppercase().to_string() == "K" {
                        self.key_visible = !self.key_visible;
                    }
                }
            }
        }
    }

    fn copy_to_clipboard(&self, text: &str) -> bool {
        Clipboard::new()
            .ok()
            .and_then(|mut clipboard| clipboard.set_text(text.to_string()).ok())
            .is_some()
    }

    fn read_clipboard(&self) -> Option<String> {
        Clipboard::new()
            .ok()
            .and_then(|mut clipboard| clipboard.get_text().ok())
    }

    fn draw(&mut self, f: &mut ratatui::Frame) {
        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints([
                Constraint::Percentage(15),
                Constraint::Percentage(35),
                Constraint::Percentage(50),
            ])
            .split(f.area());

        let input_content = format!(
            "{} {}",
            self.input_text,
            if matches!(self.state, AppState::InputMode) {
                "█"
            } else {
                ""
            }
        );
        let input_block = Paragraph::new(input_content)
            .wrap(Wrap { trim: false })
            .block(
                Block::default()
                    .borders(Borders::ALL)
                    .title(" Message Input "),
            );
        f.render_widget(input_block, chunks[0]);

        let items: Vec<ListItem> = self
            .algorithms
            .iter()
            .map(|alg| ListItem::new(alg.name()))
            .collect();

        let list = List::new(items)
            .block(
                Block::default()
                    .borders(Borders::ALL)
                    .title(" Select Algorithm "),
            )
            .highlight_style(
                Style::default()
                    .fg(Color::Yellow)
                    .add_modifier(ratatui::style::Modifier::BOLD),
            );

        f.render_stateful_widget(list, chunks[1], &mut self.list_state);

        let bottom_chunks = Layout::default()
            .direction(Direction::Horizontal)
            .constraints([Constraint::Percentage(70), Constraint::Percentage(30)])
            .split(chunks[2]);

        let right_chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints([Constraint::Percentage(60), Constraint::Percentage(40)])
            .split(bottom_chunks[1]);

        let hint = match self.state {
            AppState::InputMode => "Press Enter to choose algorithm | Ctrl+V paste",
            AppState::AlgorithmSelectionMode => "↑↓ navigate, Enter to select",
            AppState::ParameterInputMode => "Enter key, press Enter when done | Ctrl+V paste",
            AppState::ResultMode => {
                "K toggle key visibility | Ctrl+C copy result | Enter/Esc to go back"
            }
        };

        f.render_widget(
            Paragraph::new(self.result_text.as_str())
                .wrap(Wrap { trim: false })
                .block(Block::default().borders(Borders::ALL).title(" Result ")),
            bottom_chunks[0],
        );

        let idx = self.list_state.selected().unwrap_or(0);
        let alg = &self.algorithms[idx];

        let key_content = if alg.requires_key() {
            let key_display = if matches!(self.state, AppState::ResultMode) && !self.key_visible {
                "•".repeat(self.param_text.len())
            } else {
                self.param_text.to_string()
            };

            let cursor = if matches!(self.state, AppState::ParameterInputMode) {
                "█"
            } else if matches!(self.state, AppState::ResultMode) && self.key_visible {
                ""
            } else if matches!(self.state, AppState::ResultMode) {
                "🔒"
            } else {
                ""
            };

            format!("{}{}", key_display, cursor)
        } else {
            "No key required".to_string()
        };

        f.render_widget(
            Paragraph::new(key_content)
                .block(Block::default().borders(Borders::ALL).title(" Key ")),
            right_chunks[0],
        );

        let status_content = if self.status_text.is_empty() {
            hint.to_string()
        } else {
            self.status_text.clone()
        };

        f.render_widget(
            Paragraph::new(status_content)
                .block(Block::default().borders(Borders::ALL).title(" Status ")),
            right_chunks[1],
        );
    }
}

fn main() -> io::Result<()> {
    let mut app = App::new();
    app.run()
}
