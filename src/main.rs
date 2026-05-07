mod affine;
mod atbash;
mod caesar;
mod cipher;
mod digraph_block;
mod hill;
mod modular;
mod one_time_pad;
mod playfair;
mod polyalphabetic;
mod rail_fence;
mod transposition;
mod vigenere;
mod vigenere_kasiski;

use crate::affine::Affine;
use crate::atbash::Atbash;
use crate::caesar::Caesar;
use crate::cipher::Cipher;
use crate::digraph_block::DigraphBlock;
use crate::hill::Hill;
use crate::modular::Modular;
use crate::one_time_pad::OneTimePad;
use crate::playfair::Playfair;
use crate::polyalphabetic::Polyalphabetic;
use crate::rail_fence::RailFence;
use crate::transposition::Transposition;
use crate::vigenere::Vigenere;
use crate::vigenere_kasiski::VigenereKasiski;
use crossterm::{
    event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyEvent, KeyEventKind},
    execute,
    terminal::{EnterAlternateScreen, LeaveAlternateScreen, disable_raw_mode, enable_raw_mode},
};
use ratatui::{
    Terminal,
    backend::CrosstermBackend,
    layout::{Constraint, Direction, Layout},
    widgets::ListState,
    widgets::{Block, Borders, List, ListItem, Paragraph},
};
use std::io;

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
}

impl App {
    fn new() -> Self {
        Self {
            input_text: String::new(),
            algorithms: vec![
                Box::new(Vigenere),
                Box::new(Caesar),
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
            ],
            list_state: ListState::default(),
            param_text: String::new(),
            result_text: String::new(),
            state: AppState::InputMode,
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
                }
            }
        }
    }

    fn draw(&mut self, f: &mut ratatui::Frame) {
        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints([
                Constraint::Percentage(20),
                Constraint::Percentage(40),
                Constraint::Percentage(40),
            ])
            .split(f.area());

        // Input Field
        let input_content = format!(
            "{} {}",
            self.input_text,
            if matches!(self.state, AppState::InputMode) {
                "█"
            } else {
                ""
            }
        );
        let input_block = Paragraph::new(input_content).block(
            Block::default()
                .borders(Borders::ALL)
                .title(" Message Input "),
        );

        // Algorithm List
        let items: Vec<ListItem> = self
            .algorithms
            .iter()
            .map(|alg| ListItem::new(alg.name()))
            .collect();

        let list = List::new(items).block(
            Block::default()
                .borders(Borders::ALL)
                .title(" Select Algorithm "),
        );

        f.render_widget(input_block, chunks[0]);
        f.render_stateful_widget(list, chunks[1], &mut self.list_state);

        // Result or state hint
        let hint = match self.state {
            AppState::InputMode => "Press Enter to choose algorithm",
            AppState::AlgorithmSelectionMode => "↑↓ navigate, Enter to select",
            AppState::ParameterInputMode => "Enter key, press Enter when done",
            AppState::ResultMode => "Enter/Esc to go back",
        };
        let hint_block =
            Paragraph::new(hint).block(Block::default().borders(Borders::ALL).title(" Status "));
        f.render_widget(hint_block, chunks[2]);

        // Result text in ResultMode
        if matches!(self.state, AppState::ResultMode) && !self.result_text.is_empty() {
            f.render_widget(
                Paragraph::new(self.result_text.as_str())
                    .block(Block::default().borders(Borders::ALL).title(" Result ")),
                chunks[1],
            );
        }
    }
}

fn main() -> io::Result<()> {
    let mut app = App::new();
    app.run()
}
