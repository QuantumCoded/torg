mod app;

use app::App;

use std::io;
use tui::{backend::CrosstermBackend, Terminal};

fn main() -> io::Result<()> {
    let stdout = io::stdout();
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    terminal.clear()?;

    let mut app = App::new(".", terminal).unwrap();
    loop {
        app.draw()?;
    }
}
