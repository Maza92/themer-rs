use colored::*;

const ICON_SUCCESS: &str = "✓";
const ICON_ERROR: &str = "✗";
const ICON_WARNING: &str = "⚠";
const ICON_INFO: &str = "ℹ";
const ICON_BULLET: &str = "•";

pub fn header(text: &str) {
    println!("\n{}", text.cyan().bold());
}

pub fn success(text: &str) {
    println!("{} {}", ICON_SUCCESS.green().bold(), text);
}

pub fn error(text: &str) {
    eprintln!("{} {}", ICON_ERROR.red().bold(), text);
}

pub fn warning(text: &str) {
    println!("{} {}", ICON_WARNING.yellow().bold(), text);
}

pub fn info(text: &str) {
    println!("{} {}", ICON_INFO.blue().bold(), text);
}

pub fn item(badge: Option<&str>, name: &str, description: Option<&str>) {
    let bullet = ICON_BULLET.dimmed();
    let badge_str = badge
        .map(|b| format!("[{}] ", b.cyan()))
        .unwrap_or_default();
    let name_str = name.green();

    match description {
        Some(desc) => println!("  {} {}{} {}", bullet, badge_str, name_str, desc.dimmed()),
        None => println!("  {} {}{}", bullet, badge_str, name_str),
    }
}
