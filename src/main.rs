use std::io;

use ratatui::{
    DefaultTerminal, Frame,
    layout::{Constraint, Layout},
    text::Line,
    widgets::{Block, Borders, List},
};
use ratatui_textarea::{Input, Key, TextArea};

#[derive(Debug, Default)]
struct App {
    messages: Vec<String>,
    need_exit: bool,
}

#[derive(Debug)]
enum Error {
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
        let mut textarea = TextArea::default();

        textarea.set_block(Block::default().borders(Borders::ALL));

        loop {
            if self.need_exit {
                return Ok(());
            }

            terminal.draw(|frame| self.render(&textarea, frame))?;

            let event = crossterm::event::read()?;
            let input = event.into();

            self.handle_input_event(&mut textarea, input);
        }
    }

    fn handle_input_event(&mut self, textarea: &mut TextArea<'_>, input: Input) {
        match input {
            Input {
                key: Key::Enter, ..
            } => {
                let typed = textarea.lines().join("\n");

                if typed == "/quit" {
                    self.need_exit = true;
                }

                textarea.clear();
                self.messages.push(typed);
            }
            Input {
                key: Key::Char('@'),
                ..
            } => {
                self.messages
                    .push("missing feature: fzf in files".to_string());
                textarea.input(input);
            }
            Input {
                key: Key::Char('/'),
                ..
            } => {
                self.messages.push(
                    "missing feature: suggest commands (native and user added) if there are some"
                        .to_string(),
                );
                textarea.input(input);
            }
            input => {
                textarea.input(input);
            }
        }
    }

    fn render(&self, textarea: &TextArea, frame: &mut Frame) {
        let layout = Layout::vertical([
            Constraint::Min(1),
            Constraint::Length(3),
            Constraint::Min(1),
        ]);

        let [messages_area, input_area, help_area] = frame.area().layout(&layout);

        self.render_message_area(frame, messages_area);
        frame.render_widget(textarea, input_area);
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

    fn render_help_area(&self, frame: &mut Frame, area: ratatui::prelude::Rect) {
        frame.render_widget("help area", area);
    }
}

fn main() -> Result<()> {
    color_eyre::install()?;
    let mut terminal = ratatui::init();
    App::default().run(&mut terminal)?;
    ratatui::restore();
    Ok(())
}
