use super::app::App;
use super::screens::{InputField, MessageType, Screen};
use crate::backup;
use crate::config;
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
            if app.selected_menu < 7 {
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
        KeyCode::Char('7') => {
            app.screen = Screen::ImportExportMenu;
            app.import_export_menu_index = 0;
            app.msg.clear();
        }
        KeyCode::Char('8') => return true,
        KeyCode::Esc => {
            app.screen = Screen::OptionsMenu;
            app.options_menu_index = 0;
        }
        KeyCode::Char('t' | 'T') => {
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
                6 => {
                    app.screen = Screen::ImportExportMenu;
                    app.import_export_menu_index = 0;
                }
                7 => return true,
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
            app.save_theme();
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
            app.set_msg("Auto-backup coming soon!", MessageType::Info);
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

pub fn handle_import_export_menu(app: &mut App, key: KeyCode) {
    match key {
        KeyCode::Esc | KeyCode::Char('q') => {
            app.import_export_menu_index = 0;
            app.screen = Screen::MainMenu;
        }
        KeyCode::Down | KeyCode::Char('j') => {
            if app.import_export_menu_index < 5 {
                app.import_export_menu_index += 1;
            }
        }
        KeyCode::Up | KeyCode::Char('k') => {
            if app.import_export_menu_index > 0 {
                app.import_export_menu_index -= 1;
            }
        }
        KeyCode::Enter | KeyCode::Char('1'..='6') => {
            let selection = if let KeyCode::Char(c) = key {
                c.to_digit(10).map(|d| d as usize - 1)
            } else {
                Some(app.import_export_menu_index)
            };

            if let Some(idx) = selection {
                match idx {
                    0 => {
                        app.import_file_path.clear();
                        app.msg.clear();
                        app.screen = Screen::ImportCSV;
                    }
                    1 => {
                        app.import_file_path.clear();
                        app.msg.clear();
                        app.screen = Screen::ImportJSON;
                    }
                    2 => {
                        app.export_file_path.clear();
                        app.export_filter_type = 0;
                        app.export_filter_value.clear();
                        app.msg.clear();
                        app.screen = Screen::ExportCSV;
                    }
                    3 => {
                        app.export_file_path.clear();
                        app.export_filter_type = 1;
                        app.export_filter_value.clear();
                        app.msg.clear();
                        app.screen = Screen::ExportCSV;
                    }
                    4 => {
                        app.export_file_path.clear();
                        app.msg.clear();
                        app.screen = Screen::ExportJSON;
                    }
                    5 => {
                        app.export_file_path.clear();
                        app.msg.clear();
                        app.screen = Screen::ExportVault;
                    }
                    _ => {}
                }
            }
        }
        _ => {}
    }
}

pub fn handle_import_csv(app: &mut App, key: KeyCode) {
    match key {
        KeyCode::Esc => {
            app.import_file_path.clear();
            app.msg.clear();
            app.screen = Screen::ImportExportMenu;
        }
        KeyCode::Char(c) => {
            app.import_file_path.push(c);
        }
        KeyCode::Backspace => {
            app.import_file_path.pop();
        }
        KeyCode::Enter => {
            if app.import_file_path.is_empty() {
                app.set_msg("Please enter a file path", MessageType::Error);
                return;
            }

            let cfg = match config::load_config() {
                Ok(c) => c,
                Err(e) => {
                    app.set_msg(&format!("Config error: {e}"), MessageType::Error);
                    return;
                }
            };

            match backup::preview_csv_import(
                &cfg.active_vault,
                &app.master_pwd,
                &app.import_file_path,
            ) {
                Ok(preview) => {
                    app.import_preview = Some(preview);
                    app.duplicate_handling = 0;
                    app.screen = Screen::ImportPreview;
                }
                Err(e) => {
                    app.set_msg(&format!("Preview failed: {e}"), MessageType::Error);
                }
            }
        }
        _ => {}
    }
}

pub fn handle_import_json(app: &mut App, key: KeyCode) {
    match key {
        KeyCode::Esc => {
            app.import_file_path.clear();
            app.msg.clear();
            app.screen = Screen::ImportExportMenu;
        }
        KeyCode::Char(c) => {
            app.import_file_path.push(c);
        }
        KeyCode::Backspace => {
            app.import_file_path.pop();
        }
        KeyCode::Enter => {
            if app.import_file_path.is_empty() {
                app.set_msg("Please enter a file path", MessageType::Error);
                return;
            }

            let cfg = match config::load_config() {
                Ok(c) => c,
                Err(e) => {
                    app.set_msg(&format!("Config error: {e}"), MessageType::Error);
                    return;
                }
            };

            match backup::preview_json_import(
                &cfg.active_vault,
                &app.master_pwd,
                &app.import_file_path,
            ) {
                Ok(preview) => {
                    app.import_preview = Some(preview);
                    app.duplicate_handling = 0;
                    app.screen = Screen::ImportPreview;
                }
                Err(e) => {
                    app.set_msg(&format!("Preview failed: {e}"), MessageType::Error);
                }
            }
        }
        _ => {}
    }
}

pub fn handle_import_preview(app: &mut App, key: KeyCode) {
    use std::path::Path;
    match key {
        KeyCode::Esc => {
            app.import_preview = None;
            app.screen = Screen::ImportExportMenu;
        }
        KeyCode::Down => {
            if app.duplicate_handling < 2 {
                app.duplicate_handling += 1;
            }
        }
        KeyCode::Up => {
            if app.duplicate_handling > 0 {
                app.duplicate_handling -= 1;
            }
        }
        KeyCode::Enter => {
            let cfg = match config::load_config() {
                Ok(c) => c,
                Err(e) => {
                    app.set_msg(&format!("Config error: {e}"), MessageType::Error);
                    return;
                }
            };

            let is_csv = Path::new(&app.import_file_path)
                .extension()
                .is_some_and(|ext| ext.eq_ignore_ascii_case("csv"));

            let result = if is_csv {
                if app.duplicate_handling == 0 {
                    backup::import_csv(&cfg.active_vault, &app.master_pwd, &app.import_file_path)
                        .map(|()| (0, 0))
                } else {
                    let skip = app.duplicate_handling == 1;
                    let merge = app.duplicate_handling == 2;
                    backup::import_csv_smart(
                        &cfg.active_vault,
                        &app.master_pwd,
                        &app.import_file_path,
                        skip,
                        merge,
                    )
                }
            } else {
                backup::import_json(&cfg.active_vault, &app.master_pwd, &app.import_file_path)
                    .map(|()| (0, 0))
            };

            match result {
                Ok((imported, skipped)) => {
                    match crate::storage::ld_vt(&app.master_pwd) {
                        Ok(vault) => {
                            app.vault = Some(vault);
                            app.load_at();
                            if let Some(ref vault) = app.vault {
                                app.entry_disp = vault.e.clone();
                            }

                            let msg = if skipped > 0 {
                                format!("Imported {imported} entries, skipped {skipped} duplicates")
                            } else {
                                "Import successful!".to_string()
                            };
                            app.set_msg(&msg, MessageType::Success);
                        }
                        Err(e) => {
                            app.set_msg(
                                &format!("Failed to reload vault: {e}"),
                                MessageType::Error,
                            );
                        }
                    }

                    app.import_preview = None;
                    app.import_file_path.clear();
                    app.screen = Screen::MainMenu;
                }
                Err(e) => {
                    app.set_msg(&format!("Import failed: {e}"), MessageType::Error);
                }
            }
        }
        _ => {}
    }
}

pub fn handle_export_csv(app: &mut App, key: KeyCode) {
    match key {
        KeyCode::Esc => {
            app.export_file_path.clear();
            app.export_filter_value.clear();
            app.msg.clear();
            app.screen = Screen::ImportExportMenu;
        }
        KeyCode::Tab => {
            app.msg.clear();
        }
        KeyCode::Up => {
            if app.export_file_path.is_empty()
                && app.export_filter_value.is_empty()
                && app.export_filter_type > 0
            {
                app.export_filter_type -= 1;
            }
        }
        KeyCode::Down => {
            if app.export_file_path.is_empty()
                && app.export_filter_value.is_empty()
                && app.export_filter_type < 2
            {
                app.export_filter_type += 1;
            }
        }
        KeyCode::Char(c) => {
            if app.export_filter_type > 0
                && app.export_filter_value.is_empty()
                && app.export_file_path.is_empty()
            {
                app.export_filter_value.push(c);
            } else {
                app.export_file_path.push(c);
            }
        }
        KeyCode::Backspace => {
            if !app.export_file_path.is_empty() {
                app.export_file_path.pop();
            } else if !app.export_filter_value.is_empty() {
                app.export_filter_value.pop();
            }
        }
        KeyCode::Enter => {
            if app.export_file_path.is_empty() {
                app.set_msg("Please enter output file path", MessageType::Error);
                return;
            }

            if app.export_filter_type > 0 && app.export_filter_value.is_empty() {
                app.set_msg("Please enter filter value", MessageType::Error);
                return;
            }

            let cfg = match config::load_config() {
                Ok(c) => c,
                Err(e) => {
                    app.set_msg(&format!("Config error: {e}"), MessageType::Error);
                    return;
                }
            };

            let result = if app.export_filter_type == 0 {
                backup::export_csv(&cfg.active_vault, &app.master_pwd, &app.export_file_path)
            } else if app.export_filter_type == 1 {
                backup::export_csv_filtered(
                    &cfg.active_vault,
                    &app.master_pwd,
                    &app.export_file_path,
                    Some(&app.export_filter_value),
                    None,
                )
            } else {
                backup::export_csv_filtered(
                    &cfg.active_vault,
                    &app.master_pwd,
                    &app.export_file_path,
                    None,
                    Some(&app.export_filter_value),
                )
            };

            match result {
                Ok(()) => {
                    let msg = if app.export_filter_type == 0 {
                        "Export successful!".to_string()
                    } else {
                        format!("Exported filtered entries ({})", app.export_filter_value)
                    };
                    app.set_msg(&msg, MessageType::Success);
                    app.export_file_path.clear();
                    app.export_filter_value.clear();
                    app.screen = Screen::MainMenu;
                }
                Err(e) => {
                    app.set_msg(&format!("Export failed: {e}"), MessageType::Error);
                }
            }
        }
        _ => {}
    }
}

pub fn handle_export_json(app: &mut App, key: KeyCode) {
    match key {
        KeyCode::Esc => {
            app.export_file_path.clear();
            app.msg.clear();
            app.screen = Screen::ImportExportMenu;
        }
        KeyCode::Char(c) => {
            app.export_file_path.push(c);
        }
        KeyCode::Backspace => {
            app.export_file_path.pop();
        }
        KeyCode::Enter => {
            if app.export_file_path.is_empty() {
                app.set_msg("Please enter output file path", MessageType::Error);
                return;
            }

            let cfg = match config::load_config() {
                Ok(c) => c,
                Err(e) => {
                    app.set_msg(&format!("Config error: {e}"), MessageType::Error);
                    return;
                }
            };

            match backup::export_json(&cfg.active_vault, &app.master_pwd, &app.export_file_path) {
                Ok(()) => {
                    app.set_msg("Export successful!", MessageType::Success);
                    app.export_file_path.clear();
                    app.screen = Screen::MainMenu;
                }
                Err(e) => {
                    app.set_msg(&format!("Export failed: {e}"), MessageType::Error);
                }
            }
        }
        _ => {}
    }
}

pub fn handle_export_vault(app: &mut App, key: KeyCode) {
    match key {
        KeyCode::Esc => {
            app.export_file_path.clear();
            app.msg.clear();
            app.screen = Screen::ImportExportMenu;
        }
        KeyCode::Char(c) => {
            app.export_file_path.push(c);
        }
        KeyCode::Backspace => {
            app.export_file_path.pop();
        }
        KeyCode::Enter => {
            if app.export_file_path.is_empty() {
                app.set_msg("Please enter output file path", MessageType::Error);
                return;
            }

            let cfg = match config::load_config() {
                Ok(c) => c,
                Err(e) => {
                    app.set_msg(&format!("Config error: {e}"), MessageType::Error);
                    return;
                }
            };

            match backup::export_vault(&cfg.active_vault, &app.master_pwd, &app.export_file_path) {
                Ok(()) => {
                    app.set_msg("Encrypted vault exported!", MessageType::Success);
                    app.export_file_path.clear();
                    app.screen = Screen::MainMenu;
                }
                Err(e) => {
                    app.set_msg(&format!("Export failed: {e}"), MessageType::Error);
                }
            }
        }
        _ => {}
    }
}
