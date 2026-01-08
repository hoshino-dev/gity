mod gemini;
mod git;
mod ui;

use ui::Action;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenvy::dotenv().ok();

    // 1. Get git diff
    let diff_text = match git::get_diff() {
        Ok(text) => text,
        Err(e) => {
            eprintln!("{}", e);
            return Ok(());
        }
    };

    if diff_text.is_empty() {
        println!("No changes detected");
        return Ok(());
    }

    // 2. Generate commit message
    match gemini::generate_commit_message(&diff_text).await {
        Ok(message) => {
            println!("--- generated commit message ---");
            println!("{}", message);
            println!("--- end of message ---");

            // 3. User interaction loop
            let mut current_message = message;
            loop {
                match ui::prompt_action() {
                    Action::Commit => {
                        if let Err(e) = git::commit(&current_message) {
                            eprintln!("{}", e);
                        }
                        break;
                    }
                    Action::Edit => {
                        if let Some(edited) = ui::edit_message(&current_message) {
                            current_message = edited;
                            // After editing, show confirmation/action menu again or just loop?
                            // Let's loop back to action menu to allow re-editing or cancelling
                            if ui::prompt_confirm_commit(&current_message) {
                                if let Err(e) = git::commit(&current_message) {
                                    eprintln!("{}", e);
                                }
                                break;
                            } else {
                                println!("Commit cancelled (staged). Select action again.");
                            }
                        } else {
                            println!("Editing cancelled.");
                        }
                    }
                    Action::Cancel => {
                        println!("Commit cancelled.");
                        break;
                    }
                }
            }
        }
        Err(e) => eprintln!("Error generating message: {}", e),
    }

    Ok(())
}
