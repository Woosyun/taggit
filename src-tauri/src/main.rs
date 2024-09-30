// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod db;
mod types;
// use db::{find_notes_by_tags, insert_notes_with_tags};

use tauri::Manager;
use types::DBPath;
use db::{db_init, search_by_tags, fetch_note, upsert_note, upetch_tag_note};

fn main() {
    tauri::Builder::default()
        .setup(|app: &mut tauri::App| {
            let db_path = app.path_resolver().app_data_dir().unwrap().join("database.db");
            db_init(&db_path)?;

            app.manage(DBPath(db_path));
            
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            search_by_tags,
            fetch_note,
            upsert_note,
            upetch_tag_note
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
