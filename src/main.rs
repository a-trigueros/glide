use crossterm::event::{Event, KeyCode};
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
}

impl App {
    fn run(&mut self, terminal: &mut DefaultTerminal) -> std::io::Result<()> {
        loop {
            terminal.draw(|frame| self.render(frame))?;
            let event = crossterm::event::read()?;
            if let Event::Key(key) = event {
                if key.code == KeyCode::Enter {
                    let typed = self.input.value().to_string();
                    self.input.reset();
                    if typed.eq("/quit") {
                        return Ok(());
                    } else {
                        self.messages.push(typed.to_string());
                    }
                } else {
                    self.input.handle_event(&event);
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
}

fn main() -> color_eyre::Result<()> {
    color_eyre::install()?;
    let mut terminal = ratatui::init();
    App::default().run(&mut terminal)?;
    ratatui::restore();
    Ok(())
}
