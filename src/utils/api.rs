use mongodb;

#[server(search_note)]
pub async fn search_notes(tags: Vec<String>, page_number: u32) -> Vec<Note> {
    
}