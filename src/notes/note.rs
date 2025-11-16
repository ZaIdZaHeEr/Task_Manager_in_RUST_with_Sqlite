// use crate::notes::note;

use crate::notes::*;
use std::io;

fn get_priority_from_user() -> Priority {
    println!("Enter 1 from Urgent priority.");
    println!("Enter 2 from High priority.");
    println!("Enter 3 from Medium priority.");
    println!("Enter 4 from Low priority.");
    println!("Your choice : ");
    let mut choice: String = String::new();
    io::stdin()
        .read_line(&mut choice)
        .expect("Unble to read understand the input");
    let choice: i32 = choice.trim().parse().expect("Please give a valid input.");
    if choice == 1 {
        Priority::Urgent
    } else if choice == 2 {
        Priority::High
    } else if choice == 3 {
        Priority::Medium
    } else {
        Priority::Low
    }
}
pub async fn get_notes_from_user() -> Note {
    println!("Enter the note description : ");
    let mut desc: String = String::new();
    io::stdin()
        .read_line(&mut desc)
        .expect("Unable to read the input.");
    let desc = desc.trim();

    let pr: Priority = get_priority_from_user();
    Note {
        description: desc.to_string(),
        priority: pr,
    }
}
pub async fn get_id_of_note_from_user() -> i32 {
    println!("Enter the id : ");
    let mut id: String = String::new();
    io::stdin()
        .read_line(&mut id)
        .expect("Unable to read the input");

    let id: i32 = id.trim().parse().expect("Please enter a valid ID!");
    id
}
