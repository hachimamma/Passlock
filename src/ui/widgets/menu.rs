use super::super::app::App;
use super::super::colors::ThemeColors;
use super::super::screens::MessageType;
use crate::vault_ffi;
use ratatui::{
    layout::{Alignment, Constraint, Direction, Layout, Rect},
    style::{Modifier, Style},
    text::{Line, Span},
    widgets::{Block, BorderType, Borders, List, ListItem, Paragraph},
    Frame,
};

#[allow(clippy::too_many_lines)]
pub fn draw_main_menu(f: &mut Frame, size: Rect, app: &App) {
    let block = Block::default()
        .borders(Borders::NONE)
        .style(Style::default().bg(app.theme.bg0()));
    f.render_widget(block, size);

    let main_layout = Layout::default()
        .direction(Direction::Horizontal)
        .margin(2)
        .constraints([Constraint::Percentage(65), Constraint::Percentage(35)])
        .split(size);

    let has_vault = app.vault.is_some();
    let system_info_height = if has_vault { 8 } else { 4 };

    let left_layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(3),                  // title
            Constraint::Length(system_info_height), // system info and crypto
            Constraint::Min(20),                    // menu
        ])
        .split(main_layout[0]);

    let title = Paragraph::new(Line::from(vec![Span::styled(
        "PASSLOCK",
        Style::default()
            .fg(app.theme.red())
            .add_modifier(Modifier::BOLD),
    )]))
    .alignment(Alignment::Center);
    f.render_widget(title, left_layout[0]);

    let info_row_layout = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Percentage(55), // system info
            Constraint::Percentage(45), // crypto
        ])
        .split(left_layout[1]);

    if let Some(ref vault) = app.vault {
        let total = vault.e.len();
        let with_2fa = vault.e.iter().filter(|e| e.totp_secret.is_some()).count();
        let tags = app.all_tags.len();
        let cipher = vault_ffi::get_cipher();

        let info_lines = vec![
            Line::from(vec![Span::styled(
                "System Info",
                Style::default()
                    .fg(app.theme.aqua())
                    .add_modifier(Modifier::BOLD),
            )]),
            Line::from(""),
            Line::from(vec![
                Span::styled(
                    "Build: v",
                    Style::default()
                        .fg(app.theme.gray())
                        .add_modifier(Modifier::BOLD),
                ),
                Span::styled(
                    env!("CARGO_PKG_VERSION"),
                    Style::default().fg(app.theme.yellow()),
                ),
            ]),
            Line::from(vec![
                Span::styled(
                    "Cipher: ",
                    Style::default()
                        .fg(app.theme.gray())
                        .add_modifier(Modifier::BOLD),
                ),
                Span::styled(cipher, Style::default().fg(app.theme.purple())),
            ]),
            Line::from(vec![
                Span::styled(
                    "Entries: ",
                    Style::default()
                        .fg(app.theme.gray())
                        .add_modifier(Modifier::BOLD),
                ),
                Span::styled(format!("{total}"), Style::default().fg(app.theme.green())),
            ]),
            Line::from(vec![
                Span::styled(
                    "Stats: ",
                    Style::default()
                        .fg(app.theme.gray())
                        .add_modifier(Modifier::BOLD),
                ),
                Span::styled(
                    format!("{with_2fa} 2FA │ {tags} tags"),
                    Style::default().fg(app.theme.blue()),
                ),
            ]),
        ];

        let info_box = Paragraph::new(info_lines).block(
            Block::default()
                .borders(Borders::ALL)
                .border_type(BorderType::Rounded)
                .border_style(Style::default().fg(app.theme.aqua())),
        );
        f.render_widget(info_box, info_row_layout[0]);

        let is_aes = vault_ffi::aes_sup();
        let crypto_lines = vec![
            Line::from(vec![Span::styled(
                "Crypto",
                Style::default()
                    .fg(app.theme.red())
                    .add_modifier(Modifier::BOLD),
            )]),
            Line::from(""),
            Line::from(vec![Span::styled(
                "┌──────────┐",
                Style::default().fg(app.theme.gray()),
            )]),
            Line::from(vec![
                Span::styled("│  ", Style::default().fg(app.theme.gray())),
                Span::styled(
                    if is_aes { "AES-NI" } else { "ChaCha" },
                    Style::default()
                        .fg(app.theme.green())
                        .add_modifier(Modifier::BOLD),
                ),
                Span::styled("  │", Style::default().fg(app.theme.gray())),
            ]),
            Line::from(vec![Span::styled(
                "└──────────┘",
                Style::default().fg(app.theme.gray()),
            )]),
            Line::from(vec![Span::styled(
                "Hardware",
                Style::default().fg(if is_aes {
                    app.theme.green()
                } else {
                    app.theme.red()
                }),
            )]),
        ];

        let crypto_box = Paragraph::new(crypto_lines)
            .alignment(Alignment::Center)
            .block(
                Block::default()
                    .borders(Borders::ALL)
                    .border_type(BorderType::Rounded)
                    .border_style(Style::default().fg(app.theme.red())),
            );
        f.render_widget(crypto_box, info_row_layout[1]);
    }

    let all_items = [
        ("1", "View Passwords", "Browse all entries"),
        ("2", "Add Password", "Create new entry"),
        ("3", "Search", "Find passwords"),
        ("4", "Filter by Tags", "Sort entries"),
        ("5", "Generate Password", "Random secure"),
        ("6", "Delete Entry", "Remove password"),
        ("7", "Exit", "Lock vault"),
    ];

    let menu_list: Vec<ListItem> = all_items
        .iter()
        .enumerate()
        .map(|(i, (num, title, desc))| {
            let is_selected = i == app.selected_menu;

            let title_style = if is_selected {
                Style::default()
                    .fg(app.theme.yellow())
                    .add_modifier(Modifier::BOLD)
            } else {
                Style::default()
                    .fg(app.theme.fg())
                    .add_modifier(Modifier::BOLD)
            };

            let prefix = if is_selected { "▶ " } else { "  " };

            ListItem::new(vec![
                Line::from(vec![
                    Span::styled(prefix, Style::default().fg(app.theme.yellow())),
                    Span::styled(
                        format!("[{num}] "),
                        Style::default()
                            .fg(app.theme.orange())
                            .add_modifier(Modifier::BOLD),
                    ),
                    Span::styled(*title, title_style),
                ]),
                Line::from(vec![
                    Span::raw("     "),
                    Span::styled(*desc, Style::default().fg(app.theme.gray())),
                ]),
                Line::from(""),
            ])
        })
        .collect();

    let menu_block = Block::default()
        .borders(Borders::ALL)
        .border_type(BorderType::Rounded)
        .border_style(Style::default().fg(app.theme.green()))
        .title(Span::styled(
            " [ Menu ] ",
            Style::default()
                .fg(app.theme.green())
                .add_modifier(Modifier::BOLD),
        ));

    let menu = List::new(menu_list).block(menu_block);
    f.render_widget(menu, left_layout[2]);

    let right_layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(3), // status msg
            Constraint::Length(1), // spacer
            Constraint::Min(10),   // pwd stats
        ])
        .split(main_layout[1]);

    if !app.msg.is_empty() {
        let msg_style = match app.msg_type {
            MessageType::Success => Style::default().fg(app.theme.green()),
            MessageType::Error => Style::default().fg(app.theme.red()),
            MessageType::Info => Style::default().fg(app.theme.blue()),
            MessageType::None => Style::default().fg(app.theme.fg()),
        };

        let status_msg = Paragraph::new(app.msg.as_str())
            .style(msg_style.add_modifier(Modifier::BOLD))
            .alignment(Alignment::Center)
            .block(
                Block::default()
                    .borders(Borders::ALL)
                    .border_type(BorderType::Rounded)
                    .border_style(msg_style),
            );
        f.render_widget(status_msg, right_layout[0]);
    }

    if let Some(ref vault) = app.vault {
        let total = vault.e.len();
        let with_2fa = vault.e.iter().filter(|e| e.totp_secret.is_some()).count();
        let with_url = vault.e.iter().filter(|e| e.url.is_some()).count();
        let with_notes = vault
            .e
            .iter()
            .filter(|e| e.nt.as_ref().is_some_and(|n| !n.is_empty()))
            .count();

        let two_fa_pct = if total > 0 {
            (with_2fa * 100) / total
        } else {
            0
        };
        let url_pct = if total > 0 {
            (with_url * 100) / total
        } else {
            0
        };
        let notes_pct = if total > 0 {
            (with_notes * 100) / total
        } else {
            0
        };

        let create_bar = |pct: usize| -> String {
            let filled = pct / 10;
            let mut bar = String::new();
            for i in 0..10 {
                bar.push(if i < filled { '█' } else { '░' });
            }
            bar
        };

        let mut stats_lines = vec![
            Line::from(vec![Span::styled(
                "Password Statistics",
                Style::default()
                    .fg(app.theme.yellow())
                    .add_modifier(Modifier::BOLD),
            )]),
            Line::from(""),
            Line::from(vec![Span::styled(
                "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━",
                Style::default().fg(app.theme.gray()),
            )]),
            Line::from(""),
            Line::from(vec![Span::styled(
                "Total Entries",
                Style::default().fg(app.theme.gray()),
            )]),
            Line::from(vec![Span::styled(
                format!("   {total}"),
                Style::default()
                    .fg(app.theme.yellow())
                    .add_modifier(Modifier::BOLD),
            )]),
            Line::from(""),
            Line::from(vec![Span::styled(
                "2FA Protection",
                Style::default().fg(app.theme.gray()),
            )]),
            Line::from(vec![Span::styled(
                format!("   {}", create_bar(two_fa_pct)),
                Style::default().fg(app.theme.green()),
            )]),
            Line::from(vec![Span::styled(
                format!("   {with_2fa} entries ({two_fa_pct}%)"),
                Style::default().fg(app.theme.green()),
            )]),
            Line::from(""),
            Line::from(vec![Span::styled(
                "With URLs",
                Style::default().fg(app.theme.gray()),
            )]),
            Line::from(vec![Span::styled(
                format!("   {}", create_bar(url_pct)),
                Style::default().fg(app.theme.blue()),
            )]),
            Line::from(vec![Span::styled(
                format!("   {with_url} entries ({url_pct}%)"),
                Style::default().fg(app.theme.blue()),
            )]),
            Line::from(""),
            Line::from(vec![Span::styled(
                "With Notes",
                Style::default().fg(app.theme.gray()),
            )]),
            Line::from(vec![Span::styled(
                format!("   {}", create_bar(notes_pct)),
                Style::default().fg(app.theme.purple()),
            )]),
            Line::from(vec![Span::styled(
                format!("   {with_notes} entries ({notes_pct}%)"),
                Style::default().fg(app.theme.purple()),
            )]),
        ];

        if !app.all_tags.is_empty() {
            stats_lines.push(Line::from(""));
            stats_lines.push(Line::from(vec![Span::styled(
                "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━",
                Style::default().fg(app.theme.gray()),
            )]));
            stats_lines.push(Line::from(""));
            stats_lines.push(Line::from(vec![Span::styled(
                "Top Tags",
                Style::default()
                    .fg(app.theme.aqua())
                    .add_modifier(Modifier::BOLD),
            )]));

            for (tag, count) in app.all_tags.iter().take(3) {
                stats_lines.push(Line::from(vec![
                    Span::styled(
                        format!("   {tag} "),
                        Style::default().fg(app.theme.purple()),
                    ),
                    Span::styled(format!("({count})"), Style::default().fg(app.theme.gray())),
                ]));
            }
        }

        let stats_box = Paragraph::new(stats_lines).block(
            Block::default()
                .borders(Borders::ALL)
                .border_type(BorderType::Rounded)
                .border_style(Style::default().fg(app.theme.yellow())),
        );
        f.render_widget(stats_box, right_layout[2]);
    }
}
