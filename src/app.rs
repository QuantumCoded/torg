//! User interface state.

use std::fs;
use std::path::{Path, PathBuf};

use orgize::Org;

pub struct App<'a> {
    /// The set of all Org files that have been loaded.
    org_files: Vec<OrgFile<'a>>,

    /// The index in `org_files` of the particular Org file that's
    /// highlighted in the lower-left pane, and whose contents are
    /// visible in the upper-left pane.
    selected_file: usize,

    /// The (year, week number) selected in the agenda view on the
    /// upper-right.
    week: (i32, u32),

    /// The (year, month) of the first month visible in the calendar
    /// view in the lower-right.
    calendar_month: (i32, u32),
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

impl<'a> App<'a> {
    pub fn new(files: &[impl AsRef<Path>]) -> Self {
        let files = files
            .iter()
            .filter_map(|filename| {
                let contents = match fs::read_to_string(filename) {
                    Ok(value) => value,
                    Err(err) => {
                        println!(
                            "Failed to load file {}: {}",
                            filename.as_ref().to_string_lossy(),
                            err
                        );
                        return None;
                    }
                };
                let parsed = Org::parse_string(contents.to_owned());

                Some(OrgFile {
                    filename: filename.as_ref().to_owned(),
                    contents,
                    parsed,
                })
            })
            .collect();

        Self {
            org_files: files,
            selected_file: 0,
            week: (0, 0),
            calendar_month: (0, 0),
        }
    }
}
