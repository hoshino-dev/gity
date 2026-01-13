#[cfg(not(test))]
use std::process::Command;

#[cfg(not(test))]
fn execute_command(command: &str, args: &[&str]) -> std::io::Result<std::process::Output> {
    Command::new(command).args(args).output()
}

#[cfg(test)]
fn execute_command(_command: &str, _args: &[&str]) -> std::io::Result<std::process::Output> {
    use std::os::unix::process::ExitStatusExt;
    Ok(std::process::Output {
        status: std::process::ExitStatus::from_raw(0),
        stdout: b"mock diff".to_vec(),
        stderr: Vec::new(),
    })
}

pub fn get_diff() -> Result<String, String> {
    let output = execute_command("git", &["diff", "--cached"])
        .map_err(|e| format!("Failed to execute git command: {}", e))?;

    let diff = String::from_utf8_lossy(&output.stdout).to_string();
    Ok(diff)
}

pub fn commit(message: &str) -> Result<(), String> {
    let output = execute_command("git", &["commit", "-m", message])
        .map_err(|e| format!("Failed to execute git commit: {}", e))?;

    if output.status.success() {
        println!("Commit successful!");
        Ok(())
    } else {
        Err(format!(
            "Commit failed:\n{}",
            String::from_utf8_lossy(&output.stderr)
        ))
    }
}
