use crate::text::EditAction;

pub fn apply_edit_actions(base: Vec<&str>, edit_actions: Vec<EditAction>) -> Result<Vec<String>, String> {
    let mut result = vec![];
    let mut edit_iter = edit_actions.into_iter().peekable();
    let mut line_index = 0;

    while line_index < base.len() {
        // Check if there is a pending edit at the current line index
        while let Some(edit) = edit_iter.peek() {
            match edit {
                // Handle deletion: Skip the current line in the base
                EditAction::Delete(index) if *index == line_index => {
                    edit_iter.next(); // Consume the delete action
                    line_index += 1; // Skip the current line
                    continue;
                }
                // Handle addition: Add the new string before the current line
                EditAction::Add(index, str) if *index == line_index => {
                    result.push(str.clone());
                    edit_iter.next(); // Consume the add action
                }
                _ => break, // No more edits at this index
            }
        }

        // Add the current line from the base
        result.push(base[line_index].to_owned());
        line_index += 1;
    }

    // Process any remaining additions at the end
    while let Some(EditAction::Add(_, str)) = edit_iter.next() {
        result.push(str);
    }

    Ok(result)
}
