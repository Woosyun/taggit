use serde::{Serialize, Deserialize};

#[derive(PartialEq, Debug, Serialize, Deserialize)]
pub enum EditAction {
    Add(usize, String),
    Delete(usize),
}

impl EditAction {
    pub fn index(&self) -> &usize {
        match self {
            EditAction::Add(idx, _) => &idx,
            EditAction::Delete(idx) => &idx,
        }
    }
}

pub fn apply_edit_actions(base: Vec<&str>, edit_actions: Vec<EditAction>) -> Result<Vec<String>, String> {
    dbg!(&edit_actions);
    
    let mut result = vec![];
    let mut edit_iter = edit_actions.into_iter().peekable();
    let mut line_index = 0;

    let mut len_of_lcs = 0;

    while line_index < base.len() {
        while let Some(edit) = edit_iter.peek() {
            match edit {
                EditAction::Delete(index) if *index == len_of_lcs => {
                    result.push("- ".to_string() + base[line_index]);
                    edit_iter.next();
                    line_index += 1;
                    continue;
                }
                EditAction::Add(index, str) if *index == len_of_lcs => {
                    result.push("+ ".to_string() + str);
                    edit_iter.next();
                }
                _ => break,
            }
        }

        if line_index >= base.len() {
            break;
        }
        
        result.push("  ".to_string() + base[line_index]);
        line_index += 1;
        len_of_lcs += 1;
    }

    // Process any remaining additions at the end
    while let Some(EditAction::Add(_, str)) = edit_iter.next() {
        result.push("+ ".to_string() + &str);
    }

    Ok(dbg!(result))
}