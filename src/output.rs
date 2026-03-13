use console::Style;
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

pub fn prompt_input(label: &str) -> anyhow::Result<String> {
    let cyan = Style::new().cyan();
    eprint!("  {} ", cyan.apply_to(format!("{label}:")));
    std::io::stderr().flush()?;
    let mut input = String::new();
    std::io::stdin().read_line(&mut input)?;
    Ok(input.trim().to_string())
}

pub fn prompt_secret(label: &str) -> anyhow::Result<String> {
    let cyan = Style::new().cyan();
    eprint!("  {} ", cyan.apply_to(format!("{label}:")));
    std::io::stderr().flush()?;
    let input = rpassword_stub()?;
    eprintln!();
    Ok(input.trim().to_string())
}

fn rpassword_stub() -> anyhow::Result<String> {
    let mut input = String::new();
    std::io::stdin().read_line(&mut input)?;
    Ok(input)
}
