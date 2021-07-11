//! User interface state.

use std::fs;
use std::path::{Path, PathBuf};

use chrono::{Datelike, IsoWeek, Local};
use orgize::Org;
use tui::layout::{Constraint, Direction, Layout};
use tui::widgets::{Block, Borders, List, ListItem};
use tui::{backend::Backend, terminal::CompletedFrame, Terminal};

pub struct App<'a, B: Backend> {
    /// The set of all Org files that have been loaded.
    org_files: Vec<OrgFile<'a>>,

    /// The index in `org_files` of the particular Org file that's
    /// highlighted in the lower-left pane, and whose contents are
    /// visible in the upper-left pane.
    selected_file: usize,

    /// The (year, week number) selected in the agenda view on the
    /// upper-right.
    week: IsoWeek,

    /// The (year, month) of the first month visible in the calendar
    /// view in the lower-right.
    calendar_month: (i32, u32),

    term: Terminal<B>,
}

/// An Org file that has been opened and parsed.
pub struct OrgFile<'a> {
    /// The name of the file on disk where this Org file resides.
    filename: PathBuf,

    /// The complete contents of the file.
    contents: String,

    /// The set of scheduled events parsed from the file's contents.
    parsed: Org<'a>,
}

impl<'a, B: Backend> App<'a, B> {
    // pub fn new(files: &[impl AsRef<Path>], term: Terminal<B>) -> Self {
    pub fn new<T: AsRef<Path>>(dir_name: T, term: Terminal<B>) -> std::io::Result<Self> {
        let files: Vec<_> = fs::read_dir(dir_name)?
            .filter_map(|entry| {
                let entry = entry.expect("IO error during loading");
                let filename = if entry
                    .file_type()
                    .expect("IO error during loading")
                    .is_file()
                {
                    entry.file_name()
                } else {
                    return None;
                };

                let contents = match fs::read_to_string(&filename) {
                    Ok(value) => value,
                    Err(err) => {
                        println!(
                            "Failed to load file {}: {}",
                            filename.to_string_lossy(),
                            err
                        );
                        return None;
                    }
                };
                let parsed = Org::parse_string(contents.to_owned());

                Some(OrgFile {
                    filename: filename.into(),
                    contents,
                    parsed,
                })
            })
            .collect();

        let now = Local::now();
        Ok(Self {
            org_files: files,
            selected_file: 0,
            week: now.iso_week(),
            calendar_month: (now.year(), now.month()),
            term,
        })
    }

    pub fn draw(&mut self) -> Result<CompletedFrame, std::io::Error> {
        self.term.draw(|f| {
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

            let file_list = List::new(
                fs::read_dir(".")
                    .unwrap()
                    .filter_map(|dir| {
                        if let Ok(entry) = dir {
                            let path = entry.path();

                            if path.is_file() {
                                if let Some(name) = path.file_name() {
                                    return Some(ListItem::new(name.to_str().unwrap().to_string()));
                                }
                            }
                        }
                        None
                    })
                    .collect::<Vec<_>>(),
            );

            // let org_text = Paragraph::new(org_text_buf.as_ref()).wrap(Wrap { trim: false });
            // f.render_widget(org_text, org_block.inner(chunks[0]));

            f.render_widget(file_list, file_block.inner(left_chunks[1]));

            // render blocks
            f.render_widget(org_block, left_chunks[0]);
            f.render_widget(agenda_block, right_chunks[0]);
            f.render_widget(file_block, left_chunks[1]);
            f.render_widget(calendar_block, right_chunks[1]);
        })
    }
}
