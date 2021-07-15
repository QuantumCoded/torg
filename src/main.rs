mod app;

use app::App;

use std::{io, thread::sleep, time::Duration};
use tui::{backend::CrosstermBackend, Terminal};

fn main() -> io::Result<()> {
    let stdout = io::stdout();
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    terminal.clear()?;

    let mut app = App::new(".", terminal).unwrap();
    loop {
        app.draw()?;
        sleep(Duration::from_secs_f64(1.0 / 10.0));
    }
}
