use ratatui::{
    DefaultTerminal, Frame,
    layout::{Constraint, Layout},
    widgets::Block,
};

fn main() -> color_eyre::Result<()> {
    color_eyre::install()?;
    ratatui::run(app)?;
    Ok(())
}

fn app(terminal: &mut DefaultTerminal) -> std::io::Result<()> {
    loop {
        terminal.draw(render)?;
        if crossterm::event::read()?.is_key_press() {
            break Ok(());
        }
    }
}

fn render(frame: &mut Frame) {
    let layout = Layout::vertical([
        Constraint::Min(1),
        Constraint::Length(3),
        Constraint::Min(1),
    ]);

    let [messages_area, input_area, help_area] = frame.area().layout(&layout);

    render_message_area(frame, messages_area);
    render_input_area(frame, input_area);
    render_help_area(frame, help_area);
}

fn render_message_area(frame: &mut Frame, area: ratatui::prelude::Rect) {
    let surrounding = Block::bordered().title("messages");
    frame.render_widget(surrounding, area);
}

fn render_input_area(frame: &mut Frame, area: ratatui::prelude::Rect) {
    let surrounding = Block::bordered().title("input");
    frame.render_widget(surrounding, area);
}

fn render_help_area(frame: &mut Frame, area: ratatui::prelude::Rect) {
    frame.render_widget("help area", area);
}
