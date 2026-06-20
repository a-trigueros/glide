use std::io;

use crossterm::event::{Event, KeyCode, KeyEvent};
use ratatui::{
    DefaultTerminal, Frame,
    layout::{Constraint, Layout},
    text::Line,
    widgets::{Block, List, Paragraph},
};
use tui_input::{Input, backend::crossterm::EventHandler};

#[derive(Debug, Default)]
struct App {
    input: Input,
    messages: Vec<String>,
    need_exit: bool,
}

#[allow(dead_code)]
#[derive(Debug)]
enum Error {
    NoMatchingCommand,
    Io {
        #[allow(dead_code)]
        source: io::Error,
    },
    Eyre {
        #[allow(dead_code)]
        source: color_eyre::eyre::Error,
    },
}

impl From<io::Error> for Error {
    fn from(value: io::Error) -> Self {
        Error::Io { source: value }
    }
}

impl From<color_eyre::eyre::Error> for Error {
    fn from(value: color_eyre::eyre::Error) -> Self {
        Error::Eyre { source: value }
    }
}

type Result<T> = std::result::Result<T, Error>;

impl App {
    fn run(&mut self, terminal: &mut DefaultTerminal) -> Result<()> {
        loop {
            if self.need_exit {
                return Ok(());
            }

            terminal.draw(|frame| self.render(frame))?;
            let event = crossterm::event::read()?;
            if let Event::Key(KeyEvent {
                code,
                modifiers: _,
                kind: _,
                state: _,
            }) = event
            {
                match code {
                    KeyCode::Enter => {
                        let typed = self.input.value().to_string();
                        self.input.reset();
                        self.handle_command(&typed)
                            // or_else validate fuzzyfinding
                            .or_else(|_| -> Result<()> {
                                self.messages.push(typed);
                                Ok(())
                            })?;
                    }
                    KeyCode::Char('@') => {
                        self.messages
                            .push("missing feature: fzf in files".to_string());
                        self.input.handle_event(&event);
                    }
                    KeyCode::Char('/') => {
                        self.messages.push("missing feature: suggest commands (native and user added) if there are some".to_string());
                        self.input.handle_event(&event);
                    }
                    _ => {
                        self.input.handle_event(&event);
                    }
                }
            }
        }
    }

    fn render(&self, frame: &mut Frame) {
        let layout = Layout::vertical([
            Constraint::Min(1),
            Constraint::Length(3),
            Constraint::Min(1),
        ]);

        let [messages_area, input_area, help_area] = frame.area().layout(&layout);

        self.render_message_area(frame, messages_area);
        self.render_input_area(frame, input_area);
        self.render_help_area(frame, help_area);
    }

    fn render_message_area(&self, frame: &mut Frame, area: ratatui::prelude::Rect) {
        let messages = self
            .messages
            .iter()
            .map(|m| Line::from(m.to_string()))
            .collect::<Vec<_>>();
        let list = List::new(messages).block(Block::bordered());

        frame.render_widget(list, area);
    }
    fn render_input_area(&self, frame: &mut Frame, area: ratatui::prelude::Rect) {
        let paragraph = Paragraph::new(self.input.value()).block(Block::bordered().title("Input"));
        frame.render_widget(paragraph, area);
    }
    fn render_help_area(&self, frame: &mut Frame, area: ratatui::prelude::Rect) {
        frame.render_widget("help area", area);
    }

    fn handle_command(&mut self, command: &str) -> Result<()> {
        match command {
            "/quit" => {
                self.need_exit = true;
                Ok(())
            }
            _ => Err(Error::NoMatchingCommand),
        }
    }
}

fn main() -> Result<()> {
    color_eyre::install()?;
    let mut terminal = ratatui::init();
    App::default().run(&mut terminal)?;
    ratatui::restore();
    Ok(())
}
