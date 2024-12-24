use crate::text::{EditAction, EditGraph};

pub fn myers_diff<'a>(base: usize, before: &[&'a str], after: &[&'a str]) -> Result<(usize, Vec<EditAction>), String> {
    println!("before : {:?}", &before);
    println!("after : {:?}", &after);
    
    if before.len() == 0 {
        let adds = after
            .iter()
            .enumerate()
            .map(|(_, &str)| EditAction::Add(base, str.to_string()))
            .collect::<Vec<EditAction>>();

        return Ok((base, adds));
    } else if after.len() == 0 {
        let deletes = before
            .iter()
            .enumerate()
            .map(|_| EditAction::Delete(base))
            .collect::<Vec<EditAction>>();

        return Ok((base, deletes));
    }

    let mut edit_graph = EditGraph::new(before, after);
    let middle_snake = edit_graph.find_middle_snake().expect("finding middle snake should always success");

    let before_front = before
        .iter()
        .take(middle_snake.x.0)
        .map(|&t| t)
        .collect::<Vec<&str>>();
    let after_front = after
        .iter()
        .take(middle_snake.y.0)
        .map(|&t| t)
        .collect::<Vec<&str>>();

    let (base, mut edit_actions) = myers_diff(base, &before_front, &after_front)?;
    let base = base + middle_snake.x.1 - middle_snake.x.0;

    let before_back = before
        .iter()
        .skip(middle_snake.x.1)
        .map(|&t| t)
        .collect::<Vec<&str>>();
    let after_back = after
        .iter()
        .skip(middle_snake.y.1)
        .map(|&t| t)
        .collect::<Vec<&str>>();
    let (base, back) = myers_diff(base, &before_back, &after_back)?;
    
    edit_actions.extend(back);

    Ok((base, edit_actions))
}



#[cfg(test)] 
pub mod tests {
    use super::*;

    #[test] 
    pub fn add_one_line() {
        let before = "\
hello world";
        let after = "\
hello world
I added one line
";

        let before = before.lines().collect::<Vec<&str>>();
        let after = after.lines().collect::<Vec<&str>>();

        let (len_of_lcs, edit_actions) = myers_diff(0, &before, &after).expect("failed to diff");

        assert_eq!(len_of_lcs, 1);
        assert_eq!(edit_actions, vec![EditAction::Add(1, "I added one line".to_string())]);
    }

    #[test] 
    pub fn delete_one_line() {
        let before = "\
hello world
this line will be deleted";
        let after = "\
hello world";

        let before = before.lines().collect::<Vec<&str>>();
        let after = after.lines().collect::<Vec<&str>>();

        let (len_of_lcs, edit_actions) = myers_diff(0, &before, &after).expect("failed to detect deletion");

        assert_eq!(len_of_lcs, 1);
        assert_eq!(edit_actions, vec![EditAction::Delete(1)]);
    }

    #[test] 
    pub fn modify_and_add_multiple_lines() {
        use EditAction::{Add, Delete};
        
        let before = "\
hello world
This line would be preserved";
        let after = "\
hello world!!
This line would be preserved
And I added 3rd line,
and 4th line";

        let before = before.lines().collect::<Vec<&str>>();
        let after = after.lines().collect::<Vec<&str>>();

        let (len_of_lcs, edit_actions) = myers_diff(0, &before, &after).expect("failed to diff");

        assert_eq!(len_of_lcs, 1);
        assert_eq!(edit_actions, vec![
            Add(0, "hello world!!".to_string()),
            Delete(0),
            Add(1, "And I added 3rd line,".to_string()),
            Add(1, "and 4th line".to_string())
        ]);
    }
}