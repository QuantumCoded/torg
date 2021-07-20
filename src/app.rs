//! User interface state.

use std::collections::BTreeSet;
use std::fmt::Display;
use std::fs;
use std::path::{Path, PathBuf};

use chrono::{Datelike, IsoWeek, Local, NaiveDateTime};
use orgize::elements::Timestamp;
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
        let file_list = List::new(
            self.org_files
                .iter()
                .map(|file| ListItem::new(file.filename.to_string_lossy().to_string()))
                .collect::<Vec<_>>(),
        );

        let agenda = build_agenda(&self.org_files);

        let org_block = Block::default().title("org").borders(Borders::ALL);
        let agenda_block = Block::default().title("agenda").borders(Borders::ALL);
        let file_block = Block::default().title("file").borders(Borders::ALL);
        let calendar_block = Block::default().title("calendar").borders(Borders::ALL);

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

            // Render block contents.
            f.render_widget(file_list, file_block.inner(left_chunks[1]));
            f.render_widget(agenda, agenda_block.inner(right_chunks[0]));

            // Render blocks.
            f.render_widget(org_block, left_chunks[0]);
            f.render_widget(agenda_block, right_chunks[0]);
            f.render_widget(file_block, left_chunks[1]);
            f.render_widget(calendar_block, right_chunks[1]);
        })
    }
}

/// An element in the agenda view.
#[derive(PartialEq, PartialOrd, Eq, Ord)]
struct AgendaElement {
    time: NaiveDateTime,
    name: String,
}

impl Display for AgendaElement {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "[{}] {}", self.time, self.name)
    }
}

impl AgendaElement {
    fn from_timestamp(timestamp: &Timestamp, name: &str) -> Vec<Self> {
        match timestamp {
            Timestamp::Active {
                start,
                repeater: _,
                delay: _,
            } => {
                vec![Self {
                    time: start.into(),
                    name: name.to_owned(),
                }]
            }
            Timestamp::Inactive {
                start,
                repeater: _,
                delay: _,
            } => {
                vec![Self {
                    time: start.into(),
                    name: name.to_owned(),
                }]
            }
            Timestamp::ActiveRange {
                start,
                end,
                start_repeater: _,
                end_repeater: _,
                start_delay: _,
                end_delay: _,
            } => {
                vec![
                    Self {
                        time: start.into(),
                        name: name.to_owned(),
                    },
                    Self {
                        time: end.into(),
                        name: name.to_owned(),
                    },
                ]
            }
            Timestamp::InactiveRange {
                start,
                end,
                start_repeater: _,
                end_repeater: _,
                start_delay: _,
                end_delay: _,
            } => {
                vec![
                    Self {
                        time: start.into(),
                        name: name.to_owned(),
                    },
                    Self {
                        time: end.into(),
                        name: name.to_owned(),
                    },
                ]
            }
            Timestamp::Diary { value: _ } => todo!(),
        }
    }
}

/// Search for scheduled entries in each org file, and assemble them
/// into a sorted agenda of events.
fn build_agenda(files: &[OrgFile]) -> List<'static> {
    let mut agenda = BTreeSet::new();
    for file in files {
        for elem in file.parsed.iter().filter_map(|elem| match elem {
            orgize::Event::Start(orgize::Element::Title(title)) => Some(title),
            _ => None,
        }) {
            if let Some(planning) = &elem.planning {
                if let Some(sched) = &planning.scheduled {
                    agenda.extend(AgendaElement::from_timestamp(sched, &elem.raw).into_iter());
                }

                if let Some(sched) = &planning.deadline {
                    agenda.extend(AgendaElement::from_timestamp(sched, &elem.raw).into_iter());
                }

                if let Some(sched) = &planning.closed {
                    agenda.extend(AgendaElement::from_timestamp(sched, &elem.raw).into_iter());
                }
            }
        }
    }

    List::new(
        agenda
            .into_iter()
            .map(|item| ListItem::new(item.to_string()))
            .collect::<Vec<_>>(),
    )
}
