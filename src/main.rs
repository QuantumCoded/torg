use orgize::Org;

use std::io;
use tui::backend::CrosstermBackend;
use tui::layout::{Constraint, Direction, Layout};
use tui::widgets::{Block, Borders, Paragraph, Wrap};
use tui::Terminal;

fn main() -> Result<(), io::Error> {
    let stdout = io::stdout();
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    terminal.clear()?;

    let mut org_text_buf = std::fs::read_to_string("demo.org").unwrap();
    let mut agenda_text_buf = String::new();

    loop {
        terminal.draw(|f| {
            let screen_chunks = Layout::default()
                .direction(Direction::Horizontal)
                .constraints([Constraint::Percentage(50), Constraint::Percentage(50)].as_ref())
                .split(f.size());
            let left_chunks = Layout::default()
                .direction(Direction::Vertical)
                .constraints([Constraint::Percentage(70), Constraint::Percentage(30)])
                .split(screen_chunks[0]);
            let right_chunks = Layout::default()
                .direction(Direction::Vertical)
                .constraints([Constraint::Percentage(70), Constraint::Percentage(30)])
                .split(screen_chunks[1]);

            let org_block = Block::default().title("org").borders(Borders::ALL);
            let agenda_block = Block::default().title("agenda").borders(Borders::ALL);
            let file_block = Block::default().title("file").borders(Borders::ALL);
            let calendar_block = Block::default().title("calendar").borders(Borders::ALL);
            
            // let org_text = Paragraph::new(org_text_buf.as_ref()).wrap(Wrap { trim: false });
            // f.render_widget(org_text, org_block.inner(chunks[0]));

            f.render_widget(org_block, left_chunks[0]);
            f.render_widget(agenda_block, right_chunks[0]);
            f.render_widget(file_block, left_chunks[1]);
            f.render_widget(calendar_block, right_chunks[1]);
        })?;

        // gap buffer, or rope

        std::thread::sleep(std::time::Duration::from_millis(25));
    }
}
