// use std::io;

use crate::notes::note::*;

use super::notes::*;
use sqlx::SqlitePool;

pub async fn create_table(pool: &SqlitePool) {
    sqlx::query(
        "CREATE TABLE IF NOT EXISTS notes(id INTEGER PRIMARY KEY AUTOINCREMENT,description TEXT NOT NULL,priority TEXT NOT NULL)",
    )
    .execute(pool)
    .await
    .expect("Failed to create the table");
}
pub async fn delete_table(pool: &SqlitePool) {
    sqlx::query("DROP TABLE notes")
        .execute(pool)
        .await
        .expect("Table cannot be deleted.");
}
pub async fn add_note(pool: &SqlitePool) {
    let note: Note = get_notes_from_user().await;
    sqlx::query("INSERT INTO notes (description, priority) VALUES(?,?)")
        .bind(note.description)
        .bind(note.priority.to_string())
        .execute(pool)
        .await
        .expect("Note can't be added.");
    println!("Success");
}
pub async fn get_all_notes(pool: &SqlitePool) {
    let notes: Vec<NoteRow> =
        sqlx::query_as::<_, NoteRow>("SELECT id, description, priority FROM notes")
            .fetch_all(pool)
            .await
            .expect("Unable o fetch the notes");

    println!("Data comming from the database :");
    for n in &notes {
        println!("{:#?}", n);
    }
    println!("Showing total of {:#?} entrtes", notes.len());
}
pub async fn update_note(pool: &SqlitePool) {
    let a: i32 = get_id_of_note_from_user().await;
    let updated_note: Note = get_notes_from_user().await;
    let result = sqlx::query("UPDATE notes SET description = ?,priority = ? WHERE id = ?")
        .bind(updated_note.description)
        .bind(updated_note.priority.to_string())
        .bind(a)
        .execute(pool)
        .await;

    match result {
        Ok(res) => {
            if res.rows_affected() == 0 {
                println!("No task found with the given ID. Update failed.");
            } else {
                println!("Task updated successfully!");
            }
        }
        Err(e) => {
            println!("Failed to update the task: {}", e);
        }
    }
    println!("Record updated successfully");
}
pub async fn delete_specific_note(pool: &SqlitePool) {
    let id_to_delete = get_id_of_note_from_user().await;

    sqlx::query("DELETE FROM notes WHERE id = ?")
        .bind(id_to_delete)
        .execute(pool)
        .await
        .expect("Unable to delete the note. Check note id");
    println!("Record deleted successfully");
}
pub async fn delete_all_notes(pool: &SqlitePool) {
    sqlx::query("DELETE FROM notes")
        .execute(pool)
        .await
        .expect("Notes cannot be deleted");
    println!("All notes deleted successfully");
}
