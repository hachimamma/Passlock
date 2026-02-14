use super::super::app::App;
use super::super::colors::ThemeColors;
use super::super::screens::MessageType;
use ratatui::{
    layout::{Alignment, Constraint, Direction, Layout, Rect},
    style::{Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, List, ListItem, Paragraph, Wrap},
    Frame,
};

#[allow(clippy::too_many_lines)]
pub fn draw_main_menu(f: &mut Frame, size: Rect, app: &App) {
    let main_layout = Layout::default()
        .direction(Direction::Vertical)
        .margin(1)
        .constraints([
            Constraint::Length(5),
            Constraint::Min(10),
            Constraint::Length(3),
        ])
        .split(size);

    let header_block = Block::default()
        .borders(Borders::ALL)
        .border_style(Style::default().fg(app.theme.yellow()))
        .style(Style::default().bg(app.theme.bg0()));

    let vault_info = if let Some(ref vault) = app.vault {
        let tag_count = app.all_tags.len();
        let disp_count = app.entry_disp.len();
        let total_count = vault.e.len();
        let filter_indicator = if disp_count == total_count {
            String::new()
        } else {
            format!(" (Filtered: {disp_count})")
        };
        vec![
            Line::from(vec![Span::styled(
                "█▓▒░ PASSLOCK ░▒▓█",
                Style::default()
                    .fg(app.theme.orange())
                    .add_modifier(Modifier::BOLD),
            )]),
            Line::from(""),
            Line::from(vec![
                Span::styled("Vault: ", Style::default().fg(app.theme.gray())),
                Span::styled(
                    "UNLOCKED ",
                    Style::default()
                        .fg(app.theme.green())
                        .add_modifier(Modifier::BOLD),
                ),
                Span::styled("│ ", Style::default().fg(app.theme.gray())),
                Span::styled(
                    format!("{total_count} passwords{filter_indicator} "),
                    Style::default().fg(app.theme.blue()),
                ),
                Span::styled("│ ", Style::default().fg(app.theme.gray())),
                Span::styled(
                    format!("{tag_count} tags"),
                    Style::default().fg(app.theme.purple()),
                ),
            ]),
        ]
    } else {
        vec![
            Line::from(Span::styled(
                "PASSLOCK",
                Style::default().fg(app.theme.red()),
            )),
            Line::from(""),
            Line::from("No vault loaded"),
        ]
    };

    let header = Paragraph::new(vault_info)
        .block(header_block)
        .alignment(Alignment::Center);
    f.render_widget(header, main_layout[0]);

    let content_layout = Layout::default()
        .direction(Direction::Horizontal)
        .margin(0)
        .constraints([Constraint::Percentage(50), Constraint::Percentage(50)])
        .split(main_layout[1]);

    let left_block = Block::default()
        .borders(Borders::ALL)
        .border_style(Style::default().fg(app.theme.green()))
        .title("═══ PASSWORDS ═══")
        .title_alignment(Alignment::Center)
        .style(Style::default().bg(app.theme.bg0()));

    let left_items = [
        ("1", "View All", "Browse vault"),
        ("2", "Add New", "Create entry"),
        ("3", "Search", "Find passwords"),
    ];

    let left_list: Vec<ListItem> = left_items
        .iter()
        .enumerate()
        .map(|(i, (num, title, desc))| {
            let is_selected = app.selected_section == 0 && i == app.selected_menu;
            let style = if is_selected {
                Style::default()
                    .fg(app.theme.yellow())
                    .add_modifier(Modifier::BOLD)
            } else {
                Style::default().fg(app.theme.fg())
            };
            let prefix = if is_selected { "▶ " } else { "  " };
            let lines = vec![
                Line::from(vec![
                    Span::styled(prefix, Style::default().fg(app.theme.yellow())),
                    Span::styled(format!("[{num}] "), Style::default().fg(app.theme.orange())),
                    Span::styled(*title, style),
                ]),
                Line::from(vec![
                    Span::raw("     "),
                    Span::styled(*desc, Style::default().fg(app.theme.gray())),
                ]),
                Line::from(""),
            ];
            ListItem::new(lines)
        })
        .collect();

    let left = List::new(left_list).block(left_block);
    f.render_widget(left, content_layout[0]);

    let right_block = Block::default()
        .borders(Borders::ALL)
        .border_style(Style::default().fg(app.theme.purple()))
        .title("═══ TOOLS ═══")
        .title_alignment(Alignment::Center)
        .style(Style::default().bg(app.theme.bg0()));

    let right_items = [
        ("4", "Filter Tags", "Sort by tags"),
        ("5", "Generate", "Random password"),
        ("6", "Delete", "Remove entry"),
        ("7", "Exit", "Lock & quit"),
    ];

    let right_list: Vec<ListItem> = right_items
        .iter()
        .enumerate()
        .map(|(i, (num, title, desc))| {
            let is_selected = app.selected_section == 1 && i == app.selected_menu - 3;
            let style = if is_selected {
                Style::default()
                    .fg(app.theme.yellow())
                    .add_modifier(Modifier::BOLD)
            } else {
                Style::default().fg(app.theme.fg())
            };
            let prefix = if is_selected { "▶ " } else { "  " };
            let lines = vec![
                Line::from(vec![
                    Span::styled(prefix, Style::default().fg(app.theme.yellow())),
                    Span::styled(format!("[{num}] "), Style::default().fg(app.theme.orange())),
                    Span::styled(*title, style),
                ]),
                Line::from(vec![
                    Span::raw("     "),
                    Span::styled(*desc, Style::default().fg(app.theme.gray())),
                ]),
                Line::from(""),
            ];
            ListItem::new(lines)
        })
        .collect();

    let right = List::new(right_list).block(right_block);
    f.render_widget(right, content_layout[1]);

    if !app.msg.is_empty() {
        let msg_area = Layout::default()
            .direction(Direction::Vertical)
            .constraints([Constraint::Min(0), Constraint::Length(3)])
            .split(main_layout[1])[1];
        let msg_style = match app.msg_type {
            MessageType::Success => Style::default().fg(app.theme.green()),
            MessageType::Error => Style::default().fg(app.theme.red()),
            MessageType::Info => Style::default().fg(app.theme.blue()),
            MessageType::None => Style::default().fg(app.theme.fg()),
        };
        let msg = Paragraph::new(app.msg.as_str())
            .style(msg_style)
            .alignment(Alignment::Center)
            .wrap(Wrap { trim: true });
        f.render_widget(msg, msg_area);
    }

    let help_block = Block::default()
        .borders(Borders::ALL)
        .border_style(Style::default().fg(app.theme.gray()))
        .style(Style::default().bg(app.theme.bg0()));
    let help =
        Paragraph::new("↑/↓: Navigate  │  ←/→: Switch section  │  Enter: Select  │  Esc: Exit")
            .block(help_block)
            .style(Style::default().fg(app.theme.gray()))
            .alignment(Alignment::Center);
    f.render_widget(help, main_layout[2]);
}
