use std::{collections::HashSet, fs};
use rusqlite::{params, params_from_iter, Connection, Result, ToSql};
use crate::types::{Note, Tag, TagNote, DBPath};

pub fn db_init(db_path: &std::path::PathBuf) -> Result<Connection, String> {
    // println!("(connect_db) db_path: {:?}", &db_path.0);
    
    if let Some(parent) = db_path.parent() {
        fs::create_dir_all(parent).map_err(|e| e.to_string())?;
    }
    
    let conn: Connection = Connection::open(&db_path).map_err(|e| e.to_string())?;

    conn.execute("
        CREATE TABLE IF NOT EXISTS notes (
            id INTEGER PRIMARY KEY,
            title TEXT NOT NULL,
            last_modified INTEGER NOT NULL,
            content BLOB
        )", 
        ()
    ).map_err(|e| e.to_string())?;

    conn.execute("
        CREATE TABLE IF NOT EXISTS tags (
            id INTEGER PRIMARY KEY,
            name TEXT NOT NULL,
        )", 
        ()
    ).map_err(|e| e.to_string())?;

    conn.execute("
        CREATE TABLE IF NOT EXISTS tag_note (
            tag_id INTEGER,
            note_id INTEGER,
            FOREIGN KEY(note_id) REFERENCES notes(id) ON DELETE CASCADE,
            FOREIGN KEY(tag_id) REFERENCES tags(id) ON DELETE CASCADE,
            PRIMARY KEY (tag_id, note_id)
        )", 
        ()
    ).map_err(|e| e.to_string())?;

    Ok(conn)
}

#[tauri::command]
pub fn search_by_tags(db_path: tauri::State<DBPath>, tags: Vec<&str>) -> Result<Vec<Note>, String> {
    // println!("(find_notes_by_tags) received tags: {:?}", &tags);
    
    let conn = Connection::open(&db_path.0).map_err(|e| e.to_string())?;
    let placeholder: String = tags.iter().map(|_| "?").collect::<Vec<&str>>().join(",");
    let len = tags.len();

    // println!("(find_notes_by_tags) placeholder: {:?}", &placeholder);
    
    let query = format!("
        SELECT n.id, n.title, n.last_modified
        FROM notes n
        JOIN tag_note tn ON n.id = tn.note_id
        JOIN tags t ON t.id = tn.tag_id
        WHERE t.name IN ({})
        GROUP BY n.id
        HAVING COUNT(DISTINCT t.id) >= {}", placeholder, len);

    let params = tags.iter().map(|tag| tag as &dyn rusqlite::ToSql).collect::<Vec<_>>();

    let mut stmt = conn.prepare(&query).map_err(|e| e.to_string())?;

    let notes = stmt.query_map(params_from_iter(params), |row| {
        println!("(search_by_tags) row: {:?}", &row);
        Ok(Note {id: row.get(0)?, title: row.get(1)?, last_modified: row.get(2)?, content: "".to_string()})
    }).map_err(|e| e.to_string())?.collect::<Result<Vec<_>>>().map_err(|e| e.to_string())?;

    // println!("(find_notes_by_tags) found notes: {:?}", &notes);

    Ok(notes)
}

#[tauri::command]
pub fn fetch_note (db_path: tauri::State<DBPath>, note_id: &str) -> Result<Note, String>{
    //find a note and return
    let conn = Connection::open(&db_path.0).map_err(|e| e.to_string())?;
    let query = format!("
        SELECT n.*
        FROM notes n
        WHERE n.id = $1");

    let mut stmt = conn.prepare(&query).map_err(|e| e.to_string())?;
    let note_iter = stmt.query_map([note_id], |row| {
        println!("(fetch_note) row: {:?}", &row);
        
        Ok(Note {
            id: row.get(0)?,
            title: row.get(1)?,
            content: row.get(2)?,
            last_modified: row.get(3)?,
        })
    }).map_err(|e| e.to_string())?;

    // Fetch the first result, assuming note_id is unique
    let note = note_iter
        .into_iter()
        .next()
        .ok_or("Note not found".to_string())?
        .map_err(|e| e.to_string())?;

    // println!("(fetch_note) found note! {:?} ", note);

    Ok(note)
}

#[tauri::command]
pub fn upsert_note(db_path: tauri::State<DBPath>, note_id: String, content: String) -> Result<(), String> {
    //get title
    let title = content.lines().next().unwrap_or("Untitled");
    
    //handle case which note does not exist
    let conn = Connection::open(&db_path.0).map_err(|e| e.to_string())?;
    if note_id == "" {
        println!("(upsert_note) note_id is empty string, creating new note");
        let mut stmt = conn.prepare("
            INSERT INTO notes (title, last_modified, content) VALUES ($1, current_timestamp, $2)
        ").map_err(|e| e.to_string())?;
        stmt.execute([title, &content]).map_err(|e| e.to_string())?;
    } else {
        println!("(upsert_note) note_id is not empty string, updating note");
        let mut stmt = conn.prepare("
            UPDATE notes 
            SET title = $1, last_modified = current_timestamp, content = $2 
            WHERE id = $3
        ").map_err(|e| e.to_string())?;
        stmt.execute([title, &content, &note_id]).map_err(|e| e.to_string())?;
    }

    Ok(())
}

//여기서 update도 하고 fetch도 하게 할 수 있을까?
#[tauri::command]
pub fn upsert_tag_note(db_path: tauri::State<DBPath>, note_id: String, tag_names: Vec<String>, is_modification: bool) -> Result<HashSet<Tag>, String> {
    use std::collections::HashSet;
    
    let conn = Connection::open(&db_path.0).map_err(|e| e.to_string())?;
    
    //1. find all tags from note_id
    let mut stmt = conn.prepare("
        SELECT t.id, t.name
        FROM tags t
        JOIN tag_note tn ON t.id = tn.tag_id
        WHERE tn.note_id = $1
    ").map_err(|e| e.to_string())?;
    
    let old_tags = stmt.query_map([&note_id], |row| {
        Ok(Tag { id: row.get(0)?, name: row.get(1)? })
    })
    .map_err(|e| e.to_string())?
    .collect::<Result<HashSet<_>, _>>()
    .map_err(|e| e.to_string())?;

    if is_modification {
        //new_tags - old_tags = tags to be added
        let new_tags = tag_names.iter().map(|tag_name| get_tag_id(&conn, tag_name)).collect::<Result<HashSet<_>, _>>().map_err(|e| e.to_string())?;
        let tags_to_add = new_tags.difference(&old_tags).collect::<HashSet<_>>();
        for tag in &tags_to_add {
            println!("(upsert_tag_note) adding tag: {:?}", tag.name);
        }
        
        for tag in tags_to_add {
            conn.execute("INSERT INTO tag_note (tag_id, note_id) VALUES (?, ?)", params![tag.id, &note_id]).unwrap();
        }
        
        //old_tags - new_tags = tags to be deleted
        let tags_to_delete = old_tags.difference(&new_tags).collect::<HashSet<_>>();
        for tag in &tags_to_delete {
            println!("(upsert_tag_note) deleting tag: {:?}", tag.name);
        }

        for tag in tags_to_delete {
            conn.execute("DELETE FROM tag_note WHERE tag_id = ? AND note_id = ?", params![tag.id, &note_id]).unwrap();
        }

        return Ok(new_tags);
    }
    

    //return 
    Ok(old_tags)
}

fn get_tag_id(conn: &Connection, tag: &str) -> Result<Tag, rusqlite::Error> {
    let mut stmt = conn.prepare("SELECT * FROM tags WHERE name = ?").unwrap();
    let id = stmt.query_row(params![tag], |row| row.get(0)).ok();
    if id.is_none() {
        conn.execute("INSERT INTO tags (name, file_count) VALUES (?, 1)", params![tag]).unwrap();
        let id = conn.last_insert_rowid() as u64;
        // println!("(insert_notes_with_tags) inserted tag: {} with id: {}", tag, id);
        Ok(Tag {id, name: tag.to_string()})
    } else {
        Ok(Tag {id: id.unwrap(), name: tag.to_string()})
    }
}