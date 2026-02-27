use super::super::app::App;
use super::super::colors::ThemeColors;
use super::super::screens::MessageType;
use super::utility::centered_rect;
use crate::crypto;
use ratatui::{
    layout::{Alignment, Constraint, Direction, Layout, Rect},
    style::{Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, BorderType, List, ListItem, Paragraph, Wrap},
    Frame,
};

#[allow(clippy::too_many_lines, clippy::cast_possible_truncation)]
pub fn draw_view_pwds(f: &mut Frame, size: Rect, app: &mut App) {
    app.entry_row_map.clear();

    let block = Block::default()
        .borders(Borders::ALL)
        .border_type(BorderType::Rounded)
        .border_style(Style::default().fg(app.theme.green()))
        .title(Span::styled(
            " [ Passwords ] ",
            Style::default()
                .fg(app.theme.green())
                .add_modifier(Modifier::BOLD),
        ))
        .style(Style::default().bg(app.theme.bg0()));
    f.render_widget(block, size);

    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .margin(2)
        .constraints([
            Constraint::Length(4),
            Constraint::Min(5),
        ])
        .split(size);

    let filter_status = if let Some(ref tag) = app.active_tf {
        format!(" (Filtered by: {tag})")
    } else if !app.search_query.is_empty() {
        format!(" (Search: {})", app.search_query)
    } else {
        String::new()
    };

    let clipboard_status = if let Some(expires_at) = app.clipboard_countdown {
        let now = crate::get_timestamp();
        if expires_at > now {
            format!(" │ Clipboard: {}s", expires_at - now)
        } else {
            String::new()
        }
    } else {
        String::new()
    };

    let title = Paragraph::new(vec![
        Line::from(vec![
            Span::styled(
                format!("Total: {} entries", app.entry_disp.len()),
                Style::default().fg(app.theme.yellow()).add_modifier(Modifier::BOLD),
            ),
            Span::styled(filter_status, Style::default().fg(app.theme.purple())),
            Span::styled(clipboard_status, Style::default().fg(app.theme.aqua())),
        ]),
        Line::from(""),
        Line::from(vec![
            Span::styled("Right-click for options", Style::default().fg(app.theme.gray())),
        ]),
    ])
    .alignment(Alignment::Center)
    .block(
        Block::default()
            .borders(Borders::ALL)
            .border_type(BorderType::Rounded)
            .border_style(Style::default().fg(app.theme.gray()))
    );
    f.render_widget(title, chunks[0]);

    if app.entry_disp.is_empty() {
        let empty_area = centered_rect(60, 20, chunks[1]);
        let empty_msg = if app.active_tf.is_some() || !app.search_query.is_empty() {
            "No matching entries found"
        } else {
            "No passwords saved yet"
        };
        let empty = Paragraph::new(vec![
            Line::from(""),
            Line::from(Span::styled(empty_msg, Style::default().fg(app.theme.gray()).add_modifier(Modifier::BOLD))),
            Line::from(""),
            Line::from(Span::styled("Press '2' to add your first password", Style::default().fg(app.theme.gray()))),
        ])
        .alignment(Alignment::Center)
        .block(
            Block::default()
                .borders(Borders::ALL)
                .border_type(BorderType::Rounded)
                .border_style(Style::default().fg(app.theme.gray()))
        );
        f.render_widget(empty, empty_area);
    } else {
        let content_area_top = chunks[1].y;
        let mut current_row = content_area_top;
        let mut entry_lines_vec: Vec<Vec<Line>> = Vec::new();

        for (i, entry) in app.entry_disp.iter().enumerate() {
            let is_selected = i == app.selected_entry;
            let prefix = if is_selected { "▶ " } else { "  " };
            let time_ago = App::get_ta(entry.last_modified);

            let mut lines = vec![
                Line::from(vec![
                    Span::styled(prefix, Style::default().fg(app.theme.yellow())),
                    Span::styled(
                        format!("[{}] ", i + 1),
                        Style::default().fg(app.theme.orange())),
                    Span::styled(
                        &entry.n,
                        if is_selected {
                            Style::default()
                                .fg(app.theme.yellow())
                                .add_modifier(Modifier::BOLD)
                        } else {
                            Style::default().fg(app.theme.yellow())
                        },
                    ),
                    Span::styled(
                        format!("  (Modified: {time_ago})"),
                        Style::default().fg(app.theme.gray()),
                    ),
                ]),
                Line::from(vec![
                    Span::raw("     "),
                    Span::styled("├─ User: ", Style::default().fg(app.theme.gray())),
                    Span::styled(&entry.u, Style::default().fg(app.theme.blue())),
                ]),
                Line::from(vec![
                    Span::raw("     "),
                    Span::styled("├─ Pass: ", Style::default().fg(app.theme.gray())),
                    Span::styled(&entry.p, Style::default().fg(app.theme.green())),
                ]),
            ];

            if let Some(ref url) = entry.url {
                lines.push(Line::from(vec![
                    Span::raw("     "),
                    Span::styled("├─ URL:  ", Style::default().fg(app.theme.gray())),
                    Span::styled(url, Style::default().fg(app.theme.aqua())),
                ]));
            }

            if let Some(ref totp_secret) = entry.totp_secret {
                if app.show_totp_codes {
                    match crate::totp::generate_totp(totp_secret) {
                        Ok(code) => {
                            let formatted_code = crate::totp::format_totp_code(&code);
                            let remaining = crate::totp::get_totp_remaining_seconds();

                            lines.push(Line::from(vec![
                                Span::raw("     "),
                                Span::styled("├─ 2FA:  ", Style::default().fg(app.theme.gray())),
                                Span::styled(
                                    formatted_code,
                                    Style::default()
                                        .fg(app.theme.green())
                                        .add_modifier(Modifier::BOLD),
                                ),
                                Span::raw("  "),
                                Span::styled(
                                    format!("(⏱ {remaining}s)"),
                                    if remaining < 10 {
                                        Style::default().fg(app.theme.red())
                                    } else {
                                        Style::default().fg(app.theme.gray())
                                    },
                                ),
                            ]));
                        }
                        Err(_) => {
                            lines.push(Line::from(vec![
                                Span::raw("     "),
                                Span::styled("├─ 2FA:  ", Style::default().fg(app.theme.gray())),
                                Span::styled(
                                    "⚠ Invalid secret",
                                    Style::default().fg(app.theme.red()),
                                ),
                            ]));
                        }
                    }
                }
            }

            if !entry.history.is_empty() {
                lines.push(Line::from(vec![
                    Span::raw("     "),
                    Span::styled(
                        format!("├─ History: {} changes", entry.history.len()),
                        Style::default().fg(app.theme.purple()),
                    ),
                ]));
            }

            if entry.tags.is_empty() {
                lines.push(Line::from(vec![
                    Span::raw("     "),
                    Span::styled("└─", Style::default().fg(app.theme.gray())),
                ]));
            } else {
                lines.push(Line::from(vec![
                    Span::raw("     "),
                    Span::styled("└─ Tags: ", Style::default().fg(app.theme.gray())),
                    Span::styled(
                        entry.tags.join(", "),
                        Style::default().fg(app.theme.orange()),
                    ),
                ]));
            }

            lines.push(Line::from(""));

            let line_count = lines.len() as u16;
            let clickable_start = current_row + 1;
            let clickable_end = current_row + line_count - 2;

            app.entry_row_map.push((clickable_start, clickable_end, i));
            current_row += line_count;

            entry_lines_vec.push(lines);
        }

        let items: Vec<ListItem> = entry_lines_vec.into_iter().map(ListItem::new).collect();

        let list = List::new(items).block(Block::default().borders(Borders::NONE));
        f.render_widget(list, chunks[1]);
    }
}

#[allow(clippy::too_many_lines, clippy::cast_sign_loss)]
pub fn draw_add_pwd(f: &mut Frame, size: Rect, app: &App) {
    let area = centered_rect(80, 85, size);
    let block = Block::default()
        .borders(Borders::ALL)
        .border_type(BorderType::Rounded)
        .border_style(Style::default().fg(app.theme.green()))
        .title(Span::styled(
            " [ Add New Password ] ",
            Style::default()
                .fg(app.theme.green())
                .add_modifier(Modifier::BOLD),
        ))
        .style(Style::default().bg(app.theme.bg0()));
    f.render_widget(block, area);

    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .margin(2)
        .constraints([
            Constraint::Length(2), // 0: title
            Constraint::Length(1), // 1: spacer
            Constraint::Length(3), // 2: name
            Constraint::Length(3), // 3: username
            Constraint::Length(3), // 4: pwd
            Constraint::Length(2), // 5: strength bar
            Constraint::Length(2), // 6: strength feedback
            Constraint::Length(3), // 7: url
            Constraint::Length(3), // 8: totp
            Constraint::Length(3), // 9: tags input
            Constraint::Length(3), // 10: Tags display
            Constraint::Min(4),    // 11: notes
            Constraint::Length(3), // 12: msg
        ])
        .split(area);
        
    let title = Paragraph::new("Fill in the details below")
        .style(Style::default().fg(app.theme.yellow()))
        .alignment(Alignment::Center);
    f.render_widget(title, chunks[0]);
    
    let active_style = Style::default()
        .fg(app.theme.green())
        .add_modifier(Modifier::BOLD);
    let inactive_style = Style::default().fg(app.theme.gray());
    
    let name_field = Paragraph::new(format!("Name: {}", app.n_entry_name))
        .style(if app.add_fi == 0 { active_style } else { inactive_style })
        .block(
            Block::default()
                .borders(Borders::ALL)
                .border_type(BorderType::Rounded)
                .border_style(Style::default().fg(if app.add_fi == 0 { app.theme.green() } else { app.theme.gray() }))
        );
    f.render_widget(name_field, chunks[2]);
    
    let user_field = Paragraph::new(format!("Username: {}", app.n_entry_user))
        .style(if app.add_fi == 1 { active_style } else { inactive_style })
        .block(
            Block::default()
                .borders(Borders::ALL)
                .border_type(BorderType::Rounded)
                .border_style(Style::default().fg(if app.add_fi == 1 { app.theme.green() } else { app.theme.gray() }))
        );
    f.render_widget(user_field, chunks[3]);
    
    let pass_field = Paragraph::new(format!("Password: {}", app.n_entry_pass))
        .style(if app.add_fi == 2 { active_style } else { inactive_style })
        .block(
            Block::default()
                .borders(Borders::ALL)
                .border_type(BorderType::Rounded)
                .border_style(Style::default().fg(if app.add_fi == 2 { app.theme.green() } else { app.theme.gray() }))
        );
    f.render_widget(pass_field, chunks[4]);
    
    if !app.n_entry_pass.is_empty() && app.add_fi == 2 {
        let strength = crypto::calc_pwd_strength(&app.n_entry_pass);
        let strength_color = match strength.strength.as_str() {
            "Weak" => app.theme.red(),
            "Fair" => app.theme.orange(),
            "Good" => app.theme.yellow(),
            "Strong" => app.theme.green(),
            _ => app.theme.gray(),
        };
        let bar_width = (35 * strength.percentage) / 100;
        let empty_width = 35 - bar_width;
        let bar = format!(
            "[{}{}] {}% - {}",
            "█".repeat(bar_width as usize),
            "─".repeat(empty_width as usize),
            strength.percentage,
            strength.strength
        );
        let strength_display = Paragraph::new(bar)
            .style(Style::default().fg(strength_color))
            .alignment(Alignment::Center);
        f.render_widget(strength_display, chunks[5]);
        
        if !strength.feedback.is_empty() {
            let feedback_text = format!("↳ {}", strength.feedback.join(", "));
            let feedback = Paragraph::new(feedback_text)
                .style(Style::default().fg(app.theme.gray()))
                .alignment(Alignment::Center)
                .wrap(Wrap { trim: true });
            f.render_widget(feedback, chunks[6]);
        }
    }
    
    let url_field = Paragraph::new(format!("URL (optional): {}", app.n_entry_url))
        .style(if app.add_fi == 3 { active_style } else { inactive_style })
        .block(
            Block::default()
                .borders(Borders::ALL)
                .border_type(BorderType::Rounded)
                .border_style(Style::default().fg(if app.add_fi == 3 { app.theme.green() } else { app.theme.gray() }))
        );
    f.render_widget(url_field, chunks[7]);

    let totp_field = Paragraph::new(format!("2FA Secret (optional): {}", app.n_entry_totp))
        .style(if app.add_fi == 4 { active_style } else { inactive_style })
        .block(
            Block::default()
                .borders(Borders::ALL)
                .border_type(BorderType::Rounded)
                .border_style(Style::default().fg(if app.add_fi == 4 { app.theme.green() } else { app.theme.gray() }))
        );
    f.render_widget(totp_field, chunks[8]);

    let tags_text = if app.add_fi == 5 {
        format!("Tags: {} ← Enter to add", app.tag_input)
    } else {
        "Tags: (Tab to focus)".to_string()
    };
    let tags_input = Paragraph::new(tags_text)
        .style(if app.add_fi == 5 { active_style } else { inactive_style })
        .wrap(Wrap { trim: true })
        .block(
            Block::default()
                .borders(Borders::ALL)
                .border_type(BorderType::Rounded)
                .border_style(Style::default().fg(if app.add_fi == 5 { app.theme.green() } else { app.theme.gray() }))
        );
    f.render_widget(tags_input, chunks[9]);
    
    if !app.n_entry_tags.is_empty() {
        let tags_display = app
            .n_entry_tags
            .iter()
            .enumerate()
            .map(|(i, tag)| format!("[{}]{} ", i + 1, tag))
            .collect::<Vec<_>>()
            .join(" ");
        let tags_widget = Paragraph::new(format!("Added: {tags_display}"))
            .style(Style::default().fg(app.theme.orange()))
            .wrap(Wrap { trim: true });
        f.render_widget(tags_widget, chunks[10]);
    }
    
    let notes_lines: Vec<Line> = if app.n_entry_notes.is_empty() {
        vec![Line::from("Notes:")]
    } else {
        let mut lines = vec![Line::from("Notes:")];
        for line in app.n_entry_notes.lines() {
            lines.push(Line::from(line.to_string()));
        }
        lines
    };
    
    let notes = Paragraph::new(notes_lines)
        .style(if app.add_fi == 6 { active_style } else { inactive_style })
        .wrap(Wrap { trim: false })
        .block(
            Block::default()
                .borders(Borders::ALL)
                .border_type(BorderType::Rounded)
                .border_style(Style::default().fg(if app.add_fi == 6 { app.theme.green() } else { app.theme.gray() }))
        );
    f.render_widget(notes, chunks[11]);
    
    if !app.msg.is_empty() {
        let msg_style = match app.msg_type {
            MessageType::Success => Style::default().fg(app.theme.green()),
            MessageType::Error => Style::default().fg(app.theme.red()),
            MessageType::Info => Style::default().fg(app.theme.blue()),
            MessageType::None => Style::default().fg(app.theme.fg()),
        };
        let msg = Paragraph::new(app.msg.as_str())
            .style(msg_style)
            .alignment(Alignment::Center);
        f.render_widget(msg, chunks[12]);
    }
}

#[allow(clippy::too_many_lines, clippy::cast_sign_loss)]
pub fn draw_edit_pwd(f: &mut Frame, size: Rect, app: &App) {
    let area = centered_rect(80, 85, size);
    let block = Block::default()
        .borders(Borders::ALL)
        .border_type(BorderType::Rounded)
        .border_style(Style::default().fg(app.theme.orange()))
        .title(Span::styled(
            " [ Edit Password ] ",
            Style::default()
                .fg(app.theme.orange())
                .add_modifier(Modifier::BOLD),
        ))
        .style(Style::default().bg(app.theme.bg0()));
    f.render_widget(block, area);

    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .margin(2)
        .constraints([
            Constraint::Length(2), // 0: title
            Constraint::Length(1), // 1: spacer
            Constraint::Length(3), // 2: name
            Constraint::Length(3), // 3: username
            Constraint::Length(3), // 4: pwd
            Constraint::Length(2), // 5: strength bar
            Constraint::Length(2), // 6: strength feedback
            Constraint::Length(3), // 7: url
            Constraint::Length(3), // 8: totp
            Constraint::Length(3), // 9: tags input
            Constraint::Length(3), // 10: tags display
            Constraint::Min(4),    // 11: notes
            Constraint::Length(3), // 12: msg
        ])
        .split(area);
        
    let title = Paragraph::new("Edit entry details (password changes are tracked)")
        .style(Style::default().fg(app.theme.yellow()))
        .alignment(Alignment::Center);
    f.render_widget(title, chunks[0]);
    
    let active_style = Style::default()
        .fg(app.theme.green())
        .add_modifier(Modifier::BOLD);
    let inactive_style = Style::default().fg(app.theme.gray());
    
    let name_field = Paragraph::new(format!("Name: {}", app.n_entry_name))
        .style(if app.add_fi == 0 { active_style } else { inactive_style })
        .block(
            Block::default()
                .borders(Borders::ALL)
                .border_type(BorderType::Rounded)
                .border_style(Style::default().fg(if app.add_fi == 0 { app.theme.green() } else { app.theme.gray() }))
        );
    f.render_widget(name_field, chunks[2]);
    
    let user_field = Paragraph::new(format!("Username: {}", app.n_entry_user))
        .style(if app.add_fi == 1 { active_style } else { inactive_style })
        .block(
            Block::default()
                .borders(Borders::ALL)
                .border_type(BorderType::Rounded)
                .border_style(Style::default().fg(if app.add_fi == 1 { app.theme.green() } else { app.theme.gray() }))
        );
    f.render_widget(user_field, chunks[3]);
    
    let pass_field = Paragraph::new(format!("Password: {}", app.n_entry_pass))
        .style(if app.add_fi == 2 { active_style } else { inactive_style })
        .block(
            Block::default()
                .borders(Borders::ALL)
                .border_type(BorderType::Rounded)
                .border_style(Style::default().fg(if app.add_fi == 2 { app.theme.green() } else { app.theme.gray() }))
        );
    f.render_widget(pass_field, chunks[4]);
    
    if !app.n_entry_pass.is_empty() && app.add_fi == 2 {
        let strength = crypto::calc_pwd_strength(&app.n_entry_pass);
        let strength_color = match strength.strength.as_str() {
            "Weak" => app.theme.red(),
            "Fair" => app.theme.orange(),
            "Good" => app.theme.yellow(),
            "Strong" => app.theme.green(),
            _ => app.theme.gray(),
        };
        let bar_width = (35 * strength.percentage) / 100;
        let empty_width = 35 - bar_width;
        let bar = format!(
            "[{}{}] {}% - {}",
            "█".repeat(bar_width as usize),
            "─".repeat(empty_width as usize),
            strength.percentage,
            strength.strength
        );
        let strength_display = Paragraph::new(bar)
            .style(Style::default().fg(strength_color))
            .alignment(Alignment::Center);
        f.render_widget(strength_display, chunks[5]);
        
        if !strength.feedback.is_empty() {
            let feedback_text = format!("↳ {}", strength.feedback.join(", "));
            let feedback = Paragraph::new(feedback_text)
                .style(Style::default().fg(app.theme.gray()))
                .alignment(Alignment::Center)
                .wrap(Wrap { trim: true });
            f.render_widget(feedback, chunks[6]);
        }
    }
    
    let url_field = Paragraph::new(format!("URL: {}", app.n_entry_url))
        .style(if app.add_fi == 3 { active_style } else { inactive_style })
        .block(
            Block::default()
                .borders(Borders::ALL)
                .border_type(BorderType::Rounded)
                .border_style(Style::default().fg(if app.add_fi == 3 { app.theme.green() } else { app.theme.gray() }))
        );
    f.render_widget(url_field, chunks[7]);

    let totp_field = Paragraph::new(format!("2FA Secret (optional): {}", app.n_entry_totp))
        .style(if app.add_fi == 4 { active_style } else { inactive_style })
        .block(
            Block::default()
                .borders(Borders::ALL)
                .border_type(BorderType::Rounded)
                .border_style(Style::default().fg(if app.add_fi == 4 { app.theme.green() } else { app.theme.gray() }))
        );
    f.render_widget(totp_field, chunks[8]);

    let tags_text = if app.add_fi == 5 {
        format!("Tags: {} ← Enter to add", app.tag_input)
    } else {
        "Tags: (Tab to focus)".to_string()
    };
    let tags_input = Paragraph::new(tags_text)
        .style(if app.add_fi == 5 { active_style } else { inactive_style })
        .wrap(Wrap { trim: true })
        .block(
            Block::default()
                .borders(Borders::ALL)
                .border_type(BorderType::Rounded)
                .border_style(Style::default().fg(if app.add_fi == 5 { app.theme.green() } else { app.theme.gray() }))
        );
    f.render_widget(tags_input, chunks[9]);
    
    if !app.n_entry_tags.is_empty() {
        let tags_display = app
            .n_entry_tags
            .iter()
            .enumerate()
            .map(|(i, tag)| format!("[{}]{} ", i + 1, tag))
            .collect::<Vec<_>>()
            .join(" ");
        let tags_widget = Paragraph::new(format!("Tags: {tags_display}"))
            .style(Style::default().fg(app.theme.orange()))
            .wrap(Wrap { trim: true });
        f.render_widget(tags_widget, chunks[10]);
    }
    
    let notes_lines: Vec<Line> = if app.n_entry_notes.is_empty() {
        vec![Line::from("Notes:")]
    } else {
        let mut lines = vec![Line::from("Notes:")];
        for line in app.n_entry_notes.lines() {
            lines.push(Line::from(line.to_string()));
        }
        lines
    };
    
    let notes = Paragraph::new(notes_lines)
        .style(if app.add_fi == 6 { active_style } else { inactive_style })
        .wrap(Wrap { trim: false })
        .block(
            Block::default()
                .borders(Borders::ALL)
                .border_type(BorderType::Rounded)
                .border_style(Style::default().fg(if app.add_fi == 6 { app.theme.green() } else { app.theme.gray() }))
        );
    f.render_widget(notes, chunks[11]);
    
    if !app.msg.is_empty() {
        let msg_style = match app.msg_type {
            MessageType::Success => Style::default().fg(app.theme.green()),
            MessageType::Error => Style::default().fg(app.theme.red()),
            MessageType::Info => Style::default().fg(app.theme.blue()),
            MessageType::None => Style::default().fg(app.theme.fg()),
        };
        let msg = Paragraph::new(app.msg.as_str())
            .style(msg_style)
            .alignment(Alignment::Center);
        f.render_widget(msg, chunks[12]);
    }
}

pub fn draw_history(f: &mut Frame, size: Rect, app: &App) {
    let block = Block::default()
        .borders(Borders::ALL)
        .border_type(BorderType::Rounded)
        .border_style(Style::default().fg(app.theme.purple()))
        .title(Span::styled(
            " [ Password History ] ",
            Style::default()
                .fg(app.theme.purple())
                .add_modifier(Modifier::BOLD),
        ))
        .style(Style::default().bg(app.theme.bg0()));
    f.render_widget(block, size);

    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .margin(2)
        .constraints([
            Constraint::Length(3),
            Constraint::Min(5),
        ])
        .split(size);
        
    if let Some(ref vault) = app.vault {
        if app.selected_entry < app.entry_disp.len() {
            let entry = &app.entry_disp[app.selected_entry];
            if let Some(vault_entry) = vault.e.iter().find(|e| e.id == entry.id) {
                let title = Paragraph::new(vec![
                    Line::from(vec![
                        Span::styled(
                            format!("History for: {}", vault_entry.n),
                            Style::default().fg(app.theme.yellow()).add_modifier(Modifier::BOLD)
                        ),
                    ]),
                    Line::from(""),
                    Line::from(vec![
                        Span::styled("Last 5 changes", Style::default().fg(app.theme.gray())),
                    ]),
                ])
                .alignment(Alignment::Center)
                .block(
                    Block::default()
                        .borders(Borders::ALL)
                        .border_type(BorderType::Rounded)
                        .border_style(Style::default().fg(app.theme.gray()))
                );
                f.render_widget(title, chunks[0]);
                
                if vault_entry.history.is_empty() {
                    let empty_area = centered_rect(60, 20, chunks[1]);
                    let empty = Paragraph::new(vec![
                        Line::from(""),
                        Line::from(Span::styled("No password changes recorded", Style::default().fg(app.theme.gray()).add_modifier(Modifier::BOLD))),
                    ])
                    .alignment(Alignment::Center)
                    .block(
                        Block::default()
                            .borders(Borders::ALL)
                            .border_type(BorderType::Rounded)
                            .border_style(Style::default().fg(app.theme.gray()))
                    );
                    f.render_widget(empty, empty_area);
                } else {
                    let items: Vec<ListItem> = vault_entry
                        .history
                        .iter()
                        .rev()
                        .enumerate()
                        .map(|(i, hist)| {
                            let time_ago = App::get_ta(hist.changed_at);
                            let lines = vec![
                                Line::from(vec![
                                    Span::styled(
                                        format!("[{}] ", i + 1),
                                        Style::default().fg(app.theme.purple()),
                                    ),
                                    Span::styled(
                                        &hist.password,
                                        Style::default().fg(app.theme.green()),
                                    ),
                                ]),
                                Line::from(vec![
                                    Span::raw("    "),
                                    Span::styled(
                                        format!("Changed: {time_ago}"),
                                        Style::default().fg(app.theme.gray()),
                                    ),
                                ]),
                                Line::from(""),
                            ];
                            ListItem::new(lines)
                        })
                        .collect();
                    let list = List::new(items).block(Block::default().borders(Borders::NONE));
                    f.render_widget(list, chunks[1]);
                }
            }
        }
    }
}

pub fn draw_del_pwd(f: &mut Frame, size: Rect, app: &App) {
    let block = Block::default()
        .borders(Borders::ALL)
        .border_type(BorderType::Rounded)
        .border_style(Style::default().fg(app.theme.red()))
        .title(Span::styled(
            " [ Delete Password ] ",
            Style::default()
                .fg(app.theme.red())
                .add_modifier(Modifier::BOLD),
        ))
        .style(Style::default().bg(app.theme.bg0()));
    f.render_widget(block, size);

    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .margin(2)
        .constraints([
            Constraint::Length(3),
            Constraint::Min(5),
            Constraint::Length(3),
        ])
        .split(size);
        
    let title = Paragraph::new("⚠ Enter the number of the entry to delete")
        .style(Style::default().fg(app.theme.orange()))
        .alignment(Alignment::Center)
        .block(
            Block::default()
                .borders(Borders::ALL)
                .border_type(BorderType::Rounded)
                .border_style(Style::default().fg(app.theme.orange()))
        );
    f.render_widget(title, chunks[0]);
    
    let empty_vec = Vec::new();
    let entries_to_display = if app.entry_disp.is_empty() {
        app.vault.as_ref().map_or(&empty_vec, |v| &v.e)
    } else {
        &app.entry_disp
    };
    
    if entries_to_display.is_empty() {
        let empty_area = centered_rect(60, 20, chunks[1]);
        let empty = Paragraph::new(vec![
            Line::from(""),
            Line::from(Span::styled("No passwords to delete", Style::default().fg(app.theme.gray()).add_modifier(Modifier::BOLD))),
        ])
        .alignment(Alignment::Center)
        .block(
            Block::default()
                .borders(Borders::ALL)
                .border_type(BorderType::Rounded)
                .border_style(Style::default().fg(app.theme.gray()))
        );
        f.render_widget(empty, empty_area);
    } else {
        let items: Vec<ListItem> = entries_to_display
            .iter()
            .enumerate()
            .map(|(i, entry)| {
                ListItem::new(vec![
                    Line::from(vec![
                        Span::styled(
                            format!("[{}] ", i + 1),
                            Style::default().fg(app.theme.red()),
                        ),
                        Span::styled(&entry.n, Style::default().fg(app.theme.fg())),
                    ]),
                    Line::from(""),
                ])
            })
            .collect();
        let list = List::new(items).block(Block::default().borders(Borders::NONE));
        f.render_widget(list, chunks[1]);
    }
    
    let input = Paragraph::new(format!("Entry number: {}", app.input_buffer))
        .style(
            Style::default()
                .fg(app.theme.red())
                .add_modifier(Modifier::BOLD),
        )
        .block(
            Block::default()
                .borders(Borders::ALL)
                .border_type(BorderType::Rounded)
                .border_style(Style::default().fg(app.theme.red()))
        );
    f.render_widget(input, chunks[2]);
}

pub fn draw_context_menu(f: &mut Frame, app: &App) {
    let menu_width = 24u16;
    let menu_height = 7u16;
    let x = app.context_menu_x;
    let y = app.context_menu_y;

    let area = Rect {
        x,
        y,
        width: menu_width,
        height: menu_height,
    };

    let entry_idx = app.context_menu_entry_idx;
    let has_url = if entry_idx < app.entry_disp.len() {
        app.entry_disp[entry_idx].url.is_some()
    } else {
        false
    };

    let selected = app.context_menu_selected;
    let orange = app.theme.orange();
    let fg = app.theme.fg();
    let gray = app.theme.gray();
    let bg1 = app.theme.bg1();

    let items = [
        ("󰆒", "Copy Password"),
        ("󰀄", "Copy Username"),
        ("󰖟", "Copy URL"),
        ("󰏫", "Edit Entry"),
        ("󰔩", "View History"),
    ];

    let menu_lines: Vec<Line> = items
        .iter()
        .enumerate()
        .map(|(idx, (icon, label))| {
            let is_selected = idx == selected;
            let is_disabled = idx == 2 && !has_url;

            let style = if is_selected {
                Style::default().fg(orange).add_modifier(Modifier::BOLD)
            } else if is_disabled {
                Style::default().fg(gray)
            } else {
                Style::default().fg(fg)
            };

            Line::from(vec![
                Span::raw(" "),
                Span::styled(*icon, style),
                Span::raw(" "),
                Span::styled(*label, style),
            ])
        })
        .collect();

    let block = Block::default()
        .borders(Borders::ALL)
        .border_type(BorderType::Rounded)
        .border_style(Style::default().fg(orange))
        .style(Style::default().bg(bg1));

    let paragraph = Paragraph::new(menu_lines).block(block);

    f.render_widget(paragraph, area);
}