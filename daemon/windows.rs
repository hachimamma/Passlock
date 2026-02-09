use std::process::Command;

#[derive(Debug, Clone)]
pub struct WindowContext {
    pub title: String,
    pub app_name: String,
    pub suggested_name: String,
    pub suggested_url: Option<String>,
}

pub fn get_active_window_context() -> Result<WindowContext, Box<dyn std::error::Error>> {
    let title = get_active_window_title()?;
    let context = parse_window_title(&title);
    Ok(context)
}

#[cfg(target_os = "linux")]
fn get_active_window_title() -> Result<String, Box<dyn std::error::Error>> {
    if let Ok(output) = Command::new("xdotool")
        .args(&["getactivewindow", "getwindowname"])
        .output()
    {
        if output.status.success() {
            return Ok(String::from_utf8_lossy(&output.stdout).trim().to_string());
        }
    }

    if let Ok(output) = Command::new("wmctrl")
        .args(&["-l"])
        .output()
    {
        if output.status.success() {
            let lines = String::from_utf8_lossy(&output.stdout);
            if let Some(line) = lines.lines().next() {
                let parts: Vec<&str> = line.split_whitespace().collect();
                if parts.len() > 3 {
                    return Ok(parts[3..].join(" "));
                }
            }
        }
    }

    if let Ok(output) = Command::new("swaymsg")
        .args(&["-t", "get_tree"])
        .output()
    {
        if output.status.success() {
            let json = String::from_utf8_lossy(&output.stdout);
            if let Some(start) = json.find("\"focused\":true") {
                if let Some(name_start) = json[start..].find("\"name\":\"") {
                    if let Some(name_end) = json[start + name_start + 8..].find('\"') {
                        let name = &json[start + name_start + 8..start + name_start + 8 + name_end];
                        return Ok(name.to_string());
                    }
                }
            }
        }
    }

    Ok("Unknown".to_string())
}

#[cfg(target_os = "macos")]
fn get_active_window_title() -> Result<String, Box<dyn std::error::Error>> {
    let script = r#"
        tell application "System Events"
            set frontApp to name of first application process whose frontmost is true
            set frontWindow to name of front window of application process frontApp
            return frontApp & " - " & frontWindow
        end tell
    "#;

    let output = Command::new("osascript")
        .arg("-e")
        .arg(script)
        .output()?;

    if output.status.success() {
        Ok(String::from_utf8_lossy(&output.stdout).trim().to_string())
    } else {
        Ok("Unknown".to_string())
    }
}

#[cfg(target_os = "windows")]
fn get_active_window_title() -> Result<String, Box<dyn std::error::Error>> {
    Ok("Unknown".to_string())
}

fn parse_window_title(title: &str) -> WindowContext {
    let title = title.to_string();
    
    let patterns = [
        ("GitHub", "github.com"),
        ("GitLab", "gitlab.com"),
        ("Google", "google.com"),
        ("Facebook", "facebook.com"),
        ("Twitter", "twitter.com"),
        ("LinkedIn", "linkedin.com"),
        ("Reddit", "reddit.com"),
        ("Stack Overflow", "stackoverflow.com"),
        ("AWS", "aws.amazon.com"),
        ("Azure", "portal.azure.com"),
    ];

    for (name, url) in &patterns {
        if title.to_lowercase().contains(&name.to_lowercase()) {
            return WindowContext {
                title: title.clone(),
                app_name: name.to_string(),
                suggested_name: name.to_string(),
                suggested_url: Some(url.to_string()),
            };
        }
    }

    let app_name = if title.contains(" - ") {
        title.split(" - ").next().unwrap_or(&title).to_string()
    } else if title.contains(" — ") {
        title.split(" — ").next().unwrap_or(&title).to_string()
    } else {
        title.clone()
    };

    WindowContext {
        title: title.clone(),
        app_name: app_name.clone(),
        suggested_name: app_name,
        suggested_url: None,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_github() {
        let ctx = parse_window_title("GitHub - Sign in — Mozilla Firefox");
        assert_eq!(ctx.suggested_name, "GitHub");
        assert_eq!(ctx.suggested_url, Some("github.com".to_string()));
    }

    #[test]
    fn test_parse_generic() {
        let ctx = parse_window_title("My App - Window Title");
        assert_eq!(ctx.app_name, "My App");
    }
}