mod errors;
mod ui;

use crate::errors::Result;
use crate::ui::App;

fn main() -> Result<()> {
    color_eyre::install()?;
    let mut terminal = ratatui::init();
    App::default().run(&mut terminal)?;
    ratatui::restore();
    Ok(())
}
