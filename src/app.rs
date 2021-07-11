//! User interface state.

use std::path::PathBuf;

use chrono::{Duration, NaiveTime};

pub struct App {
    /// The set of all Org files that have been loaded.
    org_files: Vec<OrgFile>,

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
pub struct OrgFile {
    /// The name of the file on disk where this Org file resides.
    filename: PathBuf,

    /// The complete contents of the file.
    contents: String,

    /// The set of scheduled events parsed from the file's contents.
    events: Vec<Event>,
}

/// An event that appears on the calendar, i.e., an Org headline with
/// an attached "SCHEDULED", "DEADLINE", or "CLOSED" line.
pub struct Event {
    /// Name of the event, i.e. the text of the headline.
    name: String,

    /// The date and time of the event's first occurrence.
    time: NaiveTime,

    /// Amount of time until the event repeats, if indeed it repeats.
    repeat: Option<Duration>,
    
    /// Amount of time before deadline to warn, if there is a warning period.
    warning: Option<Duration>,
}
