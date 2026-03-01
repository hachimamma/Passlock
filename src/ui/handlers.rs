use super::app::App;
use super::screens::{InputField, MessageType, Screen};
use crossterm::event::KeyCode;

pub fn handle_cvi(app: &mut App, key: KeyCode) {
    match key {
        KeyCode::Char(c) => {
            if app.input_field == InputField::Password {
                app.input_buffer.push(c);
            } else if app.input_field == InputField::PasswordConfirm {
                app.input_buffer2.push(c);
            }
        }
        KeyCode::Backspace => {
            if app.input_field == InputField::Password {
                app.input_buffer.pop();
            } else if app.input_field == InputField::PasswordConfirm {
                app.input_buffer2.pop();
            }
        }
        KeyCode::Tab => {
            app.input_field = if app.input_field == InputField::Password {
                InputField::PasswordConfirm
            } else {
                InputField::Password
            };
        }
        KeyCode::Enter => {
            app.create_vault();
        }
        KeyCode::Esc => {
            app.should_quit = true;
        }
        _ => {}
    }
}

pub fn handle_uvi(app: &mut App, key: KeyCode) {
    match key {
        KeyCode::Char(c) => {
            app.input_buffer.push(c);
        }
        KeyCode::Backspace => {
            app.input_buffer.pop();
        }
        KeyCode::Enter => {
            app.unlock_vault();
        }
        KeyCode::Esc => {
            app.should_quit = true;
        }
        _ => {}
    }
}

#[allow(clippy::too_many_lines)]
pub fn handle_mmi(app: &mut App, key: KeyCode) -> bool {
    match key {
        KeyCode::Up => {
            if app.selected_menu > 0 {
                app.selected_menu -= 1;
                if app.selected_menu < 3 {
                    app.selected_section = 0;
                } else {
                    app.selected_section = 1;
                }
            }
        }
        KeyCode::Down => {
            if app.selected_menu < 6 {
                app.selected_menu += 1;
                if app.selected_menu < 3 {
                    app.selected_section = 0;
                } else {
                    app.selected_section = 1;
                }
            }
        }
        KeyCode::Left => {
            app.selected_section = 0;
            if app.selected_menu > 2 {
                app.selected_menu = 0;
            }
        }
        KeyCode::Right => {
            app.selected_section = 1;
            if app.selected_menu < 3 {
                app.selected_menu = 3;
            }
        }
        KeyCode::Char('1') => {
            app.screen = Screen::ViewPasswords;
            app.msg.clear();
            app.active_tf = None;
            app.search_query.clear();
            if let Some(ref vault) = app.vault {
                app.entry_disp = vault.e.clone();
            }
        }
        KeyCode::Char('2') => {
            app.screen = Screen::AddPassword;
            app.ca_form();
            app.msg.clear();
        }
        KeyCode::Char('3') => {
            app.screen = Screen::SearchPassword;
            app.search_query.clear();
            app.entry_disp.clear();
            app.msg.clear();
        }
        KeyCode::Char('4') => {
            app.screen = Screen::FilterByTag;
            app.select_tf = 0;
            app.filter_bt(None);
            app.msg.clear();
        }
        KeyCode::Char('5') => {
            app.screen = Screen::GeneratePassword;
            app.input_buffer = String::from("16");
            app.gen_pwd.clear();
            app.msg.clear();
        }
        KeyCode::Char('6') => {
            app.screen = Screen::DeletePassword;
            app.input_buffer.clear();
            app.msg.clear();
            if app.entry_disp.is_empty() {
                if let Some(ref vault) = app.vault {
                    app.entry_disp = vault.e.clone();
                }
            }
        }
        KeyCode::Char('7') => return true,
        KeyCode::Esc => {
            app.screen = Screen::OptionsMenu;
            app.options_menu_index = 0;
        }
        KeyCode::Char('8' | 't' | 'T') => {
            use super::colors::Theme;
            app.screen = Screen::ThemeSelector;
            app.theme_selector_index = Theme::all()
                .iter()
                .position(|t| t == &app.theme)
                .unwrap_or(0);
            app.msg.clear();
        }
        KeyCode::Enter => {
            app.msg.clear();
            match app.selected_menu {
                0 => {
                    app.screen = Screen::ViewPasswords;
                    app.active_tf = None;
                    app.search_query.clear();
                    if let Some(ref vault) = app.vault {
                        app.entry_disp = vault.e.clone();
                    }
                }
                1 => {
                    app.screen = Screen::AddPassword;
                    app.ca_form();
                }
                2 => {
                    app.screen = Screen::SearchPassword;
                    app.search_query.clear();
                    app.entry_disp.clear();
                }
                3 => {
                    app.screen = Screen::FilterByTag;
                    app.select_tf = 0;
                    app.filter_bt(None);
                }
                4 => {
                    app.screen = Screen::GeneratePassword;
                    app.input_buffer = String::from("16");
                    app.gen_pwd.clear();
                }
                5 => {
                    app.screen = Screen::DeletePassword;
                    app.input_buffer.clear();
                    if app.entry_disp.is_empty() {
                        if let Some(ref vault) = app.vault {
                            app.entry_disp = vault.e.clone();
                        }
                    }
                }
                6 => return true,
                _ => {}
            }
        }
        _ => {}
    }
    false
}

pub fn handle_vpi(app: &mut App, key: KeyCode) {
    match key {
        KeyCode::Up => {
            if app.selected_entry > 0 {
                app.selected_entry -= 1;
            }
        }
        KeyCode::Down => {
            if app.selected_entry < app.entry_disp.len().saturating_sub(1) {
                app.selected_entry += 1;
            }
        }
        KeyCode::Char('e' | 'E') => {
            if app.selected_entry < app.entry_disp.len() {
                let entry_id = app.entry_disp[app.selected_entry].id.clone();
                app.load_efe(&entry_id);
            }
        }
        KeyCode::Char('h' | 'H') => {
            if app.selected_entry < app.entry_disp.len() {
                app.screen = Screen::ViewHistory;
            }
        }
        KeyCode::Char('f' | 'F') => {
            app.active_tf = None;
            app.search_query.clear();
            if let Some(ref vault) = app.vault {
                app.entry_disp = vault.e.clone();
            }
            app.selected_entry = 0;
            app.set_msg("Filters cleared", MessageType::Success);
        }
        KeyCode::Esc => {
            app.screen = Screen::MainMenu;
            app.selected_entry = 0;
        }
        _ => {}
    }
}

pub fn handle_api(app: &mut App, key: KeyCode) {
    match key {
        KeyCode::Char(c) => match app.add_fi {
            0 => app.n_entry_name.push(c),
            1 => app.n_entry_user.push(c),
            2 => app.n_entry_pass.push(c),
            3 => app.n_entry_url.push(c),
            4 => app.n_entry_totp.push(c),
            5 => {
                if !c.is_ascii_digit() {
                    app.tag_input.push(c);
                } else if let Some(digit) = c.to_digit(10) {
                    let idx = (digit as usize).saturating_sub(1);
                    app.remove_tag(idx);
                }
            }
            6 => app.n_entry_notes.push(c),
            _ => {}
        },
        KeyCode::Backspace => match app.add_fi {
            0 => {
                app.n_entry_name.pop();
            }
            1 => {
                app.n_entry_user.pop();
            }
            2 => {
                app.n_entry_pass.pop();
            }
            3 => {
                app.n_entry_url.pop();
            }
            4 => {
                app.n_entry_totp.pop();
            }
            5 => {
                if app.tag_input.is_empty() && !app.n_entry_tags.is_empty() {
                    app.n_entry_tags.pop();
                } else {
                    app.tag_input.pop();
                }
            }
            6 => {
                app.n_entry_notes.pop();
            }
            _ => {}
        },
        KeyCode::Tab => {
            app.add_fi = (app.add_fi + 1) % 7;
        }
        KeyCode::BackTab => {
            app.add_fi = if app.add_fi == 0 { 6 } else { app.add_fi - 1 };
        }
        KeyCode::Enter => {
            if app.add_fi == 5 {
                app.add_tag();
            } else if app.add_fi == 6 {
                app.n_entry_notes.push('\n');
            } else {
                app.add_entry();
            }
        }
        KeyCode::Esc => {
            app.screen = Screen::MainMenu;
            app.ca_form();
        }
        _ => {}
    }
}

pub fn handle_epi(app: &mut App, key: KeyCode) {
    match key {
        KeyCode::Char(c) => match app.add_fi {
            0 => app.n_entry_name.push(c),
            1 => app.n_entry_user.push(c),
            2 => app.n_entry_pass.push(c),
            3 => app.n_entry_url.push(c),
            4 => app.n_entry_totp.push(c),
            5 => {
                if !c.is_ascii_digit() {
                    app.tag_input.push(c);
                } else if let Some(digit) = c.to_digit(10) {
                    let idx = (digit as usize).saturating_sub(1);
                    app.remove_tag(idx);
                }
            }
            6 => app.n_entry_notes.push(c),
            _ => {}
        },
        KeyCode::Backspace => match app.add_fi {
            0 => {
                app.n_entry_name.pop();
            }
            1 => {
                app.n_entry_user.pop();
            }
            2 => {
                app.n_entry_pass.pop();
            }
            3 => {
                app.n_entry_url.pop();
            }
            4 => {
                app.n_entry_totp.pop();
            }
            5 => {
                if app.tag_input.is_empty() && !app.n_entry_tags.is_empty() {
                    app.n_entry_tags.pop();
                } else {
                    app.tag_input.pop();
                }
            }
            6 => {
                app.n_entry_notes.pop();
            }
            _ => {}
        },
        KeyCode::Tab => {
            app.add_fi = (app.add_fi + 1) % 7;
        }
        KeyCode::BackTab => {
            app.add_fi = if app.add_fi == 0 { 6 } else { app.add_fi - 1 };
        }
        KeyCode::Enter => {
            if app.add_fi == 5 {
                app.add_tag();
            } else if app.add_fi == 6 {
                app.n_entry_notes.push('\n');
            } else {
                app.edit_entry();
            }
        }
        KeyCode::Esc => {
            app.screen = Screen::MainMenu;
            app.ca_form();
        }
        _ => {}
    }
}

pub fn handle_vhi(app: &mut App, key: KeyCode) {
    if key == KeyCode::Esc {
        app.screen = Screen::ViewPasswords;
    }
}

pub fn handle_si(app: &mut App, key: KeyCode) {
    match key {
        KeyCode::Char(c) => {
            app.search_query.push(c);
            app.search_entries();
        }
        KeyCode::Backspace => {
            app.search_query.pop();
            app.search_entries();
        }
        KeyCode::Enter => {
            app.screen = Screen::ViewPasswords;
        }
        KeyCode::Esc => {
            app.screen = Screen::MainMenu;
        }
        _ => {}
    }
}

pub fn handle_gi(app: &mut App, key: KeyCode) {
    match key {
        KeyCode::Char(c) if c.is_ascii_digit() => {
            app.input_buffer.push(c);
        }
        KeyCode::Backspace => {
            app.input_buffer.pop();
        }
        KeyCode::Enter => {
            app.gen_pwd();
        }
        KeyCode::Esc => {
            app.screen = Screen::MainMenu;
        }
        _ => {}
    }
}

pub fn handle_di(app: &mut App, key: KeyCode) {
    match key {
        KeyCode::Char(c) if c.is_ascii_digit() => {
            app.input_buffer.push(c);
        }
        KeyCode::Backspace => {
            app.input_buffer.pop();
        }
        KeyCode::Enter => {
            if let Ok(idx) = app.input_buffer.parse::<usize>() {
                if idx > 0 && idx <= app.entry_disp.len() {
                    let entry_id = app.entry_disp[idx - 1].id.clone();
                    if let Some(ref vault) = app.vault {
                        if let Some(vault_idx) = vault.e.iter().position(|e| e.id == entry_id) {
                            app.delete_entry(vault_idx);
                        } else {
                            app.set_msg("Entry not found in vault!", MessageType::Error);
                        }
                    }
                } else {
                    app.set_msg("Invalid entry number!", MessageType::Error);
                }
            } else {
                app.set_msg("Please enter a valid number!", MessageType::Error);
            }
        }
        KeyCode::Esc => {
            app.screen = Screen::MainMenu;
        }
        _ => {}
    }
}

pub fn handle_tfi(app: &mut App, key: KeyCode) {
    match key {
        KeyCode::Up => {
            if app.select_tf > 0 {
                app.select_tf -= 1;
            }
        }
        KeyCode::Down => {
            if app.select_tf < app.all_tags.len() {
                app.select_tf += 1;
            }
        }
        KeyCode::Enter => {
            if app.select_tf == 0 {
                app.filter_bt(None);
                app.set_msg("Showing all entries", MessageType::Success);
            } else if app.select_tf <= app.all_tags.len() {
                let tag = app.all_tags[app.select_tf - 1].0.clone();
                app.filter_bt(Some(tag.clone()));
                app.set_msg(&format!("Filtered by tag: {tag}"), MessageType::Success);
            }
        }
        KeyCode::Char('v' | 'V') => {
            if !app.entry_disp.is_empty() {
                app.selected_entry = 0;
                app.screen = Screen::ViewPasswords;
            }
        }
        KeyCode::Esc => {
            app.screen = Screen::MainMenu;
        }
        _ => {}
    }
}

pub fn handle_theme_selector(app: &mut App, key: KeyCode) {
    use super::colors::Theme;

    match key {
        KeyCode::Up | KeyCode::Char('k') => {
            if app.theme_selector_index > 0 {
                app.theme_selector_index -= 1;
            }
        }
        KeyCode::Down | KeyCode::Char('j') => {
            let themes = Theme::all();
            if app.theme_selector_index < themes.len() - 1 {
                app.theme_selector_index += 1;
            }
        }
        KeyCode::Left => {
            app.theme = app.theme.previous();
            app.theme_selector_index = Theme::all()
                .iter()
                .position(|t| t == &app.theme)
                .unwrap_or(0);
        }
        KeyCode::Right => {
            app.theme = app.theme.next();
            app.theme_selector_index = Theme::all()
                .iter()
                .position(|t| t == &app.theme)
                .unwrap_or(0);
        }
        KeyCode::Enter => {
            let themes = Theme::all();
            app.theme = themes[app.theme_selector_index];
            app.screen = Screen::MainMenu;
            app.set_msg(
                &format!("Theme changed to: {}", app.theme.name()),
                MessageType::Success,
            );
        }
        KeyCode::Esc => {
            app.screen = Screen::MainMenu;
        }
        _ => {}
    }
}

pub fn handle_options_menu(app: &mut App, key: KeyCode) -> bool {
    match key {
        KeyCode::Up | KeyCode::Char('k') => {
            if app.options_menu_index > 0 {
                app.options_menu_index -= 1;
            }
        }
        KeyCode::Down | KeyCode::Char('j') => {
            if app.options_menu_index < 2 {
                app.options_menu_index += 1;
            }
        }
        KeyCode::Char('1') => {
            app.screen = Screen::Settings;
            app.settings_menu_index = 0;
        }
        KeyCode::Char('2') => {
            app.screen = Screen::Help;
        }
        KeyCode::Char('3') => {
            return true;
        }
        KeyCode::Enter => match app.options_menu_index {
            0 => {
                app.screen = Screen::Settings;
                app.settings_menu_index = 0;
            }
            1 => {
                app.screen = Screen::Help;
            }
            2 => return true,
            _ => {}
        },
        KeyCode::Esc => {
            app.screen = Screen::MainMenu;
        }
        _ => {}
    }
    false
}

pub fn handle_settings_screen(app: &mut App, key: KeyCode) {
    match key {
        KeyCode::Up | KeyCode::Char('k') => {
            if app.settings_menu_index > 0 {
                app.settings_menu_index -= 1;
            }
        }
        KeyCode::Down | KeyCode::Char('j') => {
            if app.settings_menu_index < 2 {
                app.settings_menu_index += 1;
            }
        }
        KeyCode::Char('1') => {
            use super::colors::Theme;
            app.screen = Screen::ThemeSelector;
            app.theme_selector_index = Theme::all()
                .iter()
                .position(|t| t == &app.theme)
                .unwrap_or(0);
        }
        KeyCode::Char('2') => {
            app.clipboard_timeout = match app.clipboard_timeout {
                10 => 30,
                30 => 60,
                60 => 120,
                120 => 300,
                300 => 0,
                _ => 10,
            };

            let msg = if app.clipboard_timeout == 0 {
                "Clipboard auto-clear: DISABLED".to_string()
            } else {
                format!("Clipboard timeout: {}s", app.clipboard_timeout)
            };
            app.set_msg(&msg, MessageType::Success);
        }
        KeyCode::Char('3') => {
            // auto save toggle (future feature)
            app.set_msg("Auto-save coming soon!", MessageType::Info);
        }
        KeyCode::Enter => match app.settings_menu_index {
            0 => {
                use super::colors::Theme;
                app.screen = Screen::ThemeSelector;
                app.theme_selector_index = Theme::all()
                    .iter()
                    .position(|t| t == &app.theme)
                    .unwrap_or(0);
                app.clipboard_timeout = match app.clipboard_timeout {
                    10 => 30,
                    30 => 60,
                    60 => 120,
                    120 => 300,
                    300 => 0,
                    _ => 10,
                };
            }
            1 => {
                app.clipboard_timeout = match app.clipboard_timeout {
                    10 => 30,
                    30 => 60,
                    60 => 120,
                    120 => 300,
                    300 => 0,
                    _ => 10,
                };

                let msg = if app.clipboard_timeout == 0 {
                    "Clipboard auto-clear: DISABLED".to_string()
                } else {
                    format!("Clipboard timeout: {}s", app.clipboard_timeout)
                };
                app.set_msg(&msg, MessageType::Success);
            }
            2 => {
                app.set_msg("Auto-save coming soon!", MessageType::Info);
            }
            _ => {}
        },
        KeyCode::Esc => {
            app.screen = Screen::OptionsMenu;
        }
        _ => {}
    }
}

pub fn handle_help_screen(app: &mut App, key: KeyCode) {
    if key == KeyCode::Esc || key == KeyCode::Char('q') || key == KeyCode::Char('Q') {
        app.screen = Screen::OptionsMenu;
    }
}
