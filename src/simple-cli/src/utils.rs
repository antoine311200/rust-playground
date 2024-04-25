pub fn ellipsis(s: &str, max_length: usize) -> String {
    if s.len() <= max_length {
        return s.to_string();
    }

    let mut truncated = s[..max_length-3].to_string();
    truncated.push_str("...");
    truncated
}

pub fn get_git_branch() -> Option<String> {
    let output = std::process::Command::new("git")
        .arg("branch")
        .output()
        .ok()?;

    let output = String::from_utf8(output.stdout).ok()?;
    let output = output.lines().find(|l| l.starts_with('*'))?;

    Some(output[2..].to_string())
}