use crate::models::Entry;
use std::process::Command;

pub fn type_credentials(entry: &Entry) -> Result<(), Box<dyn std::error::Error>> {
    type_text(&entry.u)?;
    press_tab()?;
    type_text(&entry.p)?;
    Ok(())
}

#[cfg(target_os = "linux")]
fn type_text(text: &str) -> Result<(), Box<dyn std::error::Error>> {
    let result = Command::new("xdotool")
        .args(&["type", "--clearmodifiers", text])
        .output();

    if result.is_ok() && result.as_ref().unwrap().status.success() {
        return Ok(());
    }

    let result = Command::new("ydotool")
        .args(&["type", text])
        .output();

    if result.is_ok() && result.as_ref().unwrap().status.success() {
        return Ok(());
    }

    Err("No suitable typing tool found (xdotool or ydotool required)".into())
}

#[cfg(target_os = "linux")]
fn press_tab() -> Result<(), Box<dyn std::error::Error>> {
    let result = Command::new("xdotool")
        .args(&["key", "Tab"])
        .output();

    if result.is_ok() && result.as_ref().unwrap().status.success() {
        return Ok(());
    }

    let result = Command::new("ydotool")
        .args(&["key", "15:1", "15:0"])  
        .output();

    if result.is_ok() && result.as_ref().unwrap().status.success() {
        return Ok(());
    }

    Err("No suitable key tool found".into())
}

#[cfg(target_os = "macos")]
fn type_text(text: &str) -> Result<(), Box<dyn std::error::Error>> {
    let script = format!(
        r#"
        tell application "System Events"
            keystroke "{}"
        end tell
        "#,
        text.replace('"', "\\\"")
    );

    Command::new("osascript")
        .arg("-e")
        .arg(&script)
        .output()?;

    Ok(())
}

#[cfg(target_os = "macos")]
fn press_tab() -> Result<(), Box<dyn std::error::Error>> {
    let script = r#"
        tell application "System Events"
            key code 48
        end tell
    "#;

    Command::new("osascript")
        .arg("-e")
        .arg(script)
        .output()?;

    Ok(())
}

#[cfg(target_os = "windows")]
fn type_text(_text: &str) -> Result<(), Box<dyn std::error::Error>> {
    Err("Windows auto-fill not implemented yet".into())
}

#[cfg(target_os = "windows")]
fn press_tab() -> Result<(), Box<dyn std::error::Error>> {
    Err("Windows auto-fill not implemented yet".into())
}