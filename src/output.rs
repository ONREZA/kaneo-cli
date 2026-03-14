use console::Style;
use dialoguer::FuzzySelect;
use dialoguer::theme::ColorfulTheme;
use serde::Serialize;
use std::io::Write;

pub fn json_output<T: Serialize>(data: &T) {
    let json = serde_json::to_string_pretty(data).expect("failed to serialize");
    println!("{json}");
}

pub fn status(json: bool, icon: &str, msg: &str) {
    if json {
        return;
    }
    let dim = Style::new().dim();
    eprintln!("  {icon} {}", dim.apply_to(msg));
}

pub fn success(json: bool, msg: &str) {
    if json {
        return;
    }
    let green = Style::new().green();
    eprintln!("  {} {msg}", green.apply_to("✓"));
}

pub fn warn(json: bool, msg: &str) {
    if json {
        return;
    }
    let yellow = Style::new().yellow();
    eprintln!("  {} {msg}", yellow.apply_to("!"));
}

pub fn error_msg(msg: &str) {
    let red = Style::new().red().bold();
    eprintln!("{} {msg}", red.apply_to("Error:"));
}

pub fn header(json: bool, title: &str) {
    if json {
        return;
    }
    let bold = Style::new().bold();
    eprintln!("\n  {}", bold.apply_to(title));
}

/// Returns true if stderr is a TTY (interactive terminal).
pub fn is_interactive() -> bool {
    std::io::IsTerminal::is_terminal(&std::io::stderr())
}

/// Interactive fuzzy-select from a list of items. Returns the selected index.
pub fn select(prompt: &str, items: &[String]) -> anyhow::Result<usize> {
    FuzzySelect::with_theme(&ColorfulTheme::default())
        .with_prompt(prompt)
        .items(items)
        .default(0)
        .interact()
        .map_err(|e| anyhow::anyhow!("selection cancelled: {e}"))
}

pub fn prompt_input(label: &str) -> anyhow::Result<String> {
    let cyan = Style::new().cyan();
    eprint!("  {} ", cyan.apply_to(format!("{label}:")));
    std::io::stderr().flush()?;
    let mut input = String::new();
    std::io::stdin().read_line(&mut input)?;
    Ok(input.trim().to_string())
}

#[allow(dead_code)]
pub fn prompt_secret(label: &str) -> anyhow::Result<String> {
    let cyan = Style::new().cyan();
    eprint!("  {} ", cyan.apply_to(format!("{label}:")));
    std::io::stderr().flush()?;
    let input = rpassword_stub()?;
    eprintln!();
    Ok(input.trim().to_string())
}

#[allow(dead_code)]
fn rpassword_stub() -> anyhow::Result<String> {
    let mut input = String::new();
    std::io::stdin().read_line(&mut input)?;
    Ok(input)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn json_output_produces_valid_json() {
        // json_output prints to stdout, but we can at least verify the
        // serialization logic works for various types
        let data = serde_json::json!({"key": "value", "num": 42});
        let json = serde_json::to_string_pretty(&data).unwrap();
        assert!(json.contains("\"key\""));
        assert!(json.contains("42"));
    }

    #[test]
    fn json_output_vec() {
        let data = vec!["a", "b", "c"];
        let json = serde_json::to_string_pretty(&data).unwrap();
        let restored: Vec<String> = serde_json::from_str(&json).unwrap();
        assert_eq!(restored, vec!["a", "b", "c"]);
    }

    #[test]
    fn json_output_empty_struct() {
        #[derive(Serialize)]
        struct Empty {}
        let json = serde_json::to_string_pretty(&Empty {}).unwrap();
        assert_eq!(json, "{}");
    }
}
