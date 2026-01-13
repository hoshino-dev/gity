pub enum Action {
    Commit,
    Edit,
    Cancel,
}

pub fn prompt_action() -> Action {
    let options = vec!["Commit", "Edit", "Cancel"];

    let ans = inquire::Select::new("Select an action:", options).prompt();

    match ans {
        Ok("Commit") => Action::Commit,
        Ok("Edit") => Action::Edit,
        _ => Action::Cancel,
    }
}

pub fn prompt_confirm_commit(message: &str) -> bool {
    println!("\nMessage to commit:\n---\n{}\n---\n", message);
    let ans = inquire::Confirm::new("Do you want to commit with this message?")
        .with_default(true)
        .prompt();

    ans.unwrap_or(false)
}

pub fn edit_message(message: &str) -> Option<String> {
    inquire::Editor::new("Edit commit message")
        .with_predefined_text(message)
        .with_file_extension(".txt")
        .prompt()
        .ok()
}
