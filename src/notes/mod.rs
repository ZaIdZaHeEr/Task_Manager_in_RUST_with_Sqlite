pub mod note;
pub use note::get_notes_from_user;
use sqlx::prelude::FromRow;
use std::fmt;

#[derive(Debug)]
pub enum Priority {
    Urgent,
    High,
    Medium,
    Low,
}

impl fmt::Display for Priority {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Priority::Low => write!(f, "Low"),
            Priority::Medium => write!(f, "Medium"),
            Priority::High => write!(f, "High"),
            Priority::Urgent => write!(f, "Urgent"),
        }
    }
}

#[derive(Debug, FromRow)]
pub struct Note {
    pub description: String,
    pub priority: Priority,
}

#[derive(Debug, FromRow)]
pub struct NoteRow {
    pub id: i32,
    pub description: String,
    pub priority: String,
}
