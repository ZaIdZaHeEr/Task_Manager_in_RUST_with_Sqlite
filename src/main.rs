pub mod db;
pub mod notes;

use std::io;

use sqlx::sqlite::SqlitePool;

#[tokio::main]
async fn main() {
    let pool = SqlitePool::connect("sqlite://demo.db")
        .await
        .expect("Can't connect to the database");
    db::create_table(&pool).await;

    println!("------------------TASK TRACKER IN RUST WITH SQLITE------------------");
    loop {
        println!("Enter 1 to Add New Task");
        println!("Enter 2 to View Tasks");
        println!("Enter 3 to update Task");
        println!("Enter 4 to Delete Task");
        println!("Enter 5 to Delete All Task");
        println!("Enter 6 to exit");
        let mut choice: String = String::new();
        io::stdin()
            .read_line(&mut choice)
            .expect("Please give a valid choice.");
        let choice: i32 = choice.trim().parse().expect("Please give a valid number.");
        match choice {
            1 => db::add_note(&pool).await,
            2 => db::get_all_notes(&pool).await,
            3 => db::update_note(&pool).await,
            4 => db::delete_specific_note(&pool).await,
            5 => db::delete_all_notes(&pool).await,
            _ => break,
        }
    }
    // db::delete_table(&pool).await; //uncomment this and comment the loop to delete the table structure completely.
}
