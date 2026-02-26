use super::super::app::App;
use super::super::colors::ThemeColors;
use ratatui::{
    layout::{Alignment, Constraint, Direction, Layout, Rect},
    style::{Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, List, ListItem, Paragraph, Wrap},
    Frame,
};

pub fn centered_rect(percent_x: u16, percent_y: u16, r: Rect) -> Rect {
    let popup_layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Percentage((100 - percent_y) / 2),
            Constraint::Percentage(percent_y),
            Constraint::Percentage((100 - percent_y) / 2),
        ])
        .split(r);
    Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Percentage((100 - percent_x) / 2),
            Constraint::Percentage(percent_x),
            Constraint::Percentage((100 - percent_x) / 2),
        ])
        .split(popup_layout[1])[1]
}

pub fn draw_search_pwd(f: &mut Frame, size: Rect, app: &App) {
    let block = Block::default()
        .borders(Borders::ALL)
        .border_type(ratatui::widgets::BorderType::Rounded)
        .border_style(Style::default().fg(app.theme.blue()))
        .title(Span::styled(
            " [ Search Passwords ] ",
            Style::default()
                .fg(app.theme.blue())
                .add_modifier(Modifier::BOLD),
        ))
        .style(Style::default().bg(app.theme.bg0()));
    f.render_widget(block, size);

    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .margin(2)
        .constraints([
            Constraint::Length(3),  // search bar
            Constraint::Length(1),  // spacer
            Constraint::Min(5),     // results
        ])
        .split(size);

    let search_area = chunks[0];
    let search_block = Block::default()
        .borders(Borders::ALL)
        .border_type(ratatui::widgets::BorderType::Rounded)
        .border_style(Style::default().fg(if app.search_query.is_empty() {
            app.theme.gray()
        } else {
            app.theme.green()
        }))
        .title(Span::styled(
            " Search ",
            Style::default().fg(app.theme.yellow()),
        ));
    
    let search_text = if app.search_query.is_empty() {
        Span::styled("Type to search...", Style::default().fg(app.theme.gray()))
    } else {
        Span::styled(&app.search_query, Style::default().fg(app.theme.green()).add_modifier(Modifier::BOLD))
    };
    
    let search_widget = Paragraph::new(search_text)
        .block(search_block)
        .alignment(Alignment::Left);
    f.render_widget(search_widget, search_area);

    if app.search_query.is_empty() {
        let tips_area = centered_rect(70, 30, chunks[2]);
        let tips = Paragraph::new(vec![
            Line::from(""),
            Line::from(Span::styled("Quick Search Tips", Style::default().fg(app.theme.yellow()).add_modifier(Modifier::BOLD))),
            Line::from(""),
            Line::from(vec![
                Span::styled("• ", Style::default().fg(app.theme.orange())),
                Span::raw("Search by "),
                Span::styled("name, username, URL, or tags", Style::default().fg(app.theme.green())),
            ]),
            Line::from(vec![
                Span::styled("• ", Style::default().fg(app.theme.orange())),
                Span::raw("Results update "),
                Span::styled("as you type", Style::default().fg(app.theme.blue())),
            ]),
            Line::from(vec![
                Span::styled("• ", Style::default().fg(app.theme.orange())),
                Span::raw("Press "),
                Span::styled("Enter", Style::default().fg(app.theme.purple())),
                Span::raw(" to view detailed results"),
            ]),
        ])
        .alignment(Alignment::Left)
        .block(
            Block::default()
                .borders(Borders::ALL)
                .border_type(ratatui::widgets::BorderType::Rounded)
                .border_style(Style::default().fg(app.theme.aqua()))
        );
        f.render_widget(tips, tips_area);
    } else if app.entry_disp.is_empty() {
        let empty_area = centered_rect(60, 20, chunks[2]);
        let empty = Paragraph::new(vec![
            Line::from(""),
            Line::from(Span::styled("No matches found", Style::default().fg(app.theme.gray()).add_modifier(Modifier::BOLD))),
            Line::from(""),
            Line::from(Span::styled("Try a different search term", Style::default().fg(app.theme.gray()))),
        ])
        .alignment(Alignment::Center)
        .block(
            Block::default()
                .borders(Borders::ALL)
                .border_type(ratatui::widgets::BorderType::Rounded)
                .border_style(Style::default().fg(app.theme.gray()))
        );
        f.render_widget(empty, empty_area);
    } else if !app.entry_disp.is_empty() {
        let items: Vec<ListItem> = app
            .entry_disp
            .iter()
            .map(|entry| {
                let mut lines = vec![
                    Line::from(vec![
                        Span::styled("▶ ", Style::default().fg(app.theme.orange())),
                        Span::styled(
                            &entry.n,
                            Style::default()
                                .fg(app.theme.yellow())
                                .add_modifier(Modifier::BOLD),
                        ),
                    ]),
                    Line::from(vec![
                        Span::styled("  Username: ", Style::default().fg(app.theme.gray())),
                        Span::styled(&entry.u, Style::default().fg(app.theme.blue())),
                    ]),
                ];
                
                if let Some(ref url) = entry.url {
                    lines.push(Line::from(vec![
                        Span::styled("  URL: ", Style::default().fg(app.theme.gray())),
                        Span::styled(url, Style::default().fg(app.theme.aqua())),
                    ]));
                }
                
                if !entry.tags.is_empty() {
                    lines.push(Line::from(vec![
                        Span::styled("  Tags: ", Style::default().fg(app.theme.gray())),
                        Span::styled(
                            entry.tags.join(", "),
                            Style::default().fg(app.theme.purple()),
                        ),
                    ]));
                }
                
                lines.push(Line::from(""));
                ListItem::new(lines)
            })
            .collect();
        
        let list = List::new(items).block(
            Block::default()
                .borders(Borders::ALL)
                .border_type(ratatui::widgets::BorderType::Rounded)
                .border_style(Style::default().fg(app.theme.green()))
                .title(Span::styled(
                    format!(" [ {} Results ] ", app.entry_disp.len()),
                    Style::default().fg(app.theme.green()).add_modifier(Modifier::BOLD),
                ))
        );
        f.render_widget(list, chunks[2]);
    } else {
        let tips_area = centered_rect(70, 30, chunks[2]);
        let tips = Paragraph::new(vec![
            Line::from(""),
            Line::from(Span::styled("Quick Search Tips", Style::default().fg(app.theme.yellow()).add_modifier(Modifier::BOLD))),
            Line::from(""),
            Line::from(vec![
                Span::styled("• ", Style::default().fg(app.theme.orange())),
                Span::raw("Search by "),
                Span::styled("name, username, URL, or tags", Style::default().fg(app.theme.green())),
            ]),
            Line::from(vec![
                Span::styled("• ", Style::default().fg(app.theme.orange())),
                Span::raw("Results update "),
                Span::styled("as you type", Style::default().fg(app.theme.blue())),
            ]),
            Line::from(vec![
                Span::styled("• ", Style::default().fg(app.theme.orange())),
                Span::raw("Press "),
                Span::styled("Enter", Style::default().fg(app.theme.purple())),
                Span::raw(" to view detailed results"),
            ]),
        ])
        .alignment(Alignment::Left)
        .block(
            Block::default()
                .borders(Borders::ALL)
                .border_type(ratatui::widgets::BorderType::Rounded)
                .border_style(Style::default().fg(app.theme.aqua()))
        );
        f.render_widget(tips, tips_area);
    }
}

pub fn draw_gen_pwd(f: &mut Frame, size: Rect, app: &App) {
    let area = centered_rect(60, 50, size);
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .margin(2)
        .constraints([
            Constraint::Length(3),
            Constraint::Length(3),
            Constraint::Length(6),
            Constraint::Min(1),
            Constraint::Length(3),
        ])
        .split(area);
    let block = Block::default()
        .borders(Borders::ALL)
        .border_style(Style::default().fg(app.theme.aqua()))
        .title("═══ GENERATE PASSWORD ═══")
        .title_alignment(Alignment::Center)
        .style(Style::default().bg(app.theme.bg0()));
    f.render_widget(block, area);
    let title = Paragraph::new("Enter password length (4-64)")
        .style(Style::default().fg(app.theme.yellow()))
        .alignment(Alignment::Center);
    f.render_widget(title, chunks[0]);
    let length_input = Paragraph::new(format!(
        "Length: {}",
        if app.input_buffer.is_empty() {
            "16"
        } else {
            &app.input_buffer
        }
    ))
    .style(
        Style::default()
            .fg(app.theme.green())
            .add_modifier(Modifier::BOLD),
    );
    f.render_widget(length_input, chunks[1]);
    if !app.gen_pwd.is_empty() {
        let generated = Paragraph::new(vec![
            Line::from(""),
            Line::from(Span::styled(
                "Generated Password:",
                Style::default().fg(app.theme.gray()),
            )),
            Line::from(""),
            Line::from(Span::styled(
                &app.gen_pwd,
                Style::default()
                    .fg(app.theme.green())
                    .add_modifier(Modifier::BOLD),
            )),
        ])
        .alignment(Alignment::Center);
        f.render_widget(generated, chunks[2]);
    }
    let help = Paragraph::new("| Enter: Generate │ Esc: Back |")
        .style(Style::default().fg(app.theme.gray()))
        .alignment(Alignment::Center);
    f.render_widget(help, chunks[4]);
}

pub fn draw_filter_tags(f: &mut Frame, size: Rect, app: &App) {
    let block = Block::default()
        .borders(Borders::ALL)
        .border_type(ratatui::widgets::BorderType::Rounded)
        .border_style(Style::default().fg(app.theme.purple()))
        .title(Span::styled(
            " [ Filter by Tags ] ",
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
            Constraint::Length(3),  // status or active filter
            Constraint::Length(1),  // spacer
            Constraint::Min(5),     // tags list
        ])
        .split(size);

    if let Some(ref tag) = app.active_tf {
        let status = Paragraph::new(vec![
            Line::from(vec![
                Span::styled("Active Filter: ", Style::default().fg(app.theme.gray())),
                Span::styled(tag, Style::default().fg(app.theme.purple()).add_modifier(Modifier::BOLD)),
                Span::styled(format!(" ({} entries)", app.entry_disp.len()), Style::default().fg(app.theme.aqua())),
            ]),
        ])
        .block(
            Block::default()
                .borders(Borders::ALL)
                .border_type(ratatui::widgets::BorderType::Rounded)
                .border_style(Style::default().fg(app.theme.green()))
        )
        .alignment(Alignment::Center);
        f.render_widget(status, chunks[0]);
    } else {
        let status = Paragraph::new("No active filter")
            .style(Style::default().fg(app.theme.gray()))
            .block(
                Block::default()
                    .borders(Borders::ALL)
                    .border_type(ratatui::widgets::BorderType::Rounded)
                    .border_style(Style::default().fg(app.theme.gray()))
            )
            .alignment(Alignment::Center);
        f.render_widget(status, chunks[0]);
    }

    if app.all_tags.is_empty() {
        let empty_area = centered_rect(60, 30, chunks[2]);
        let empty = Paragraph::new(vec![
            Line::from(""),
            Line::from(Span::styled("No tags available", Style::default().fg(app.theme.gray()).add_modifier(Modifier::BOLD))),
            Line::from(""),
            Line::from(Span::styled("Add tags to your passwords first", Style::default().fg(app.theme.gray()))),
        ])
        .alignment(Alignment::Center)
        .block(
            Block::default()
                .borders(Borders::ALL)
                .border_type(ratatui::widgets::BorderType::Rounded)
                .border_style(Style::default().fg(app.theme.gray()))
        );
        f.render_widget(empty, empty_area);
    } else {
        let mut items = vec![
            ListItem::new(vec![Line::from("")]),
            ListItem::new(vec![
                Line::from(vec![
                    Span::styled(
                        if app.select_tf == 0 { "▶ " } else { "  " },
                        Style::default().fg(app.theme.yellow()),
                    ),
                    Span::styled(
                        "[ALL]",
                        if app.select_tf == 0 {
                            Style::default()
                                .fg(app.theme.green())
                                .add_modifier(Modifier::BOLD)
                        } else {
                            Style::default().fg(app.theme.fg())
                        },
                    ),
                    Span::raw("  "),
                    Span::styled(
                        format!("({} total)", app.vault.as_ref().map_or(0, |v| v.e.len())),
                        Style::default().fg(app.theme.gray()),
                    ),
                ]),
                Line::from(""),
            ])
        ];

        for (idx, (tag, count)) in app.all_tags.iter().enumerate() {
            let is_selected = idx + 1 == app.select_tf;
            let is_active = app.active_tf.as_ref().map_or(false, |t| t == tag);
            
            let prefix = if is_selected { "▶ " } else { "  " };
            let tag_style = if is_selected {
                Style::default()
                    .fg(app.theme.yellow())
                    .add_modifier(Modifier::BOLD)
            } else if is_active {
                Style::default()
                    .fg(app.theme.purple())
                    .add_modifier(Modifier::BOLD)
            } else {
                Style::default().fg(app.theme.fg())
            };
            
            items.push(ListItem::new(vec![
                Line::from(vec![
                    Span::styled(prefix, Style::default().fg(app.theme.yellow())),
                    Span::styled(format!("[{}]", tag), tag_style),
                    Span::raw("  "),
                    Span::styled(
                        format!("({} entries)", count),
                        Style::default().fg(app.theme.gray()),
                    ),
                    if is_active {
                        Span::styled("  ✓", Style::default().fg(app.theme.green()))
                    } else {
                        Span::raw("")
                    },
                ]),
                Line::from(""),
            ]));
        }

        let list = List::new(items).block(
            Block::default()
                .borders(Borders::ALL)
                .border_type(ratatui::widgets::BorderType::Rounded)
                .border_style(Style::default().fg(app.theme.purple()))
                .title(Span::styled(
                    format!(" [ {} Tags Available ] ", app.all_tags.len()),
                    Style::default().fg(app.theme.aqua()).add_modifier(Modifier::BOLD),
                ))
        );
        f.render_widget(list, chunks[2]);
    }
}

#[allow(clippy::too_many_lines)]
pub fn draw_theme_selector(f: &mut Frame, area: Rect, app: &App) {
    use super::super::colors::{Theme, ThemeColors};

    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .margin(2)
        .constraints([
            Constraint::Length(3),
            Constraint::Min(10),
            Constraint::Length(3),
        ])
        .split(area);

    let title = Paragraph::new("THEME SELECTOR")
        .style(
            Style::default()
                .fg(app.theme.yellow())
                .add_modifier(Modifier::BOLD),
        )
        .alignment(Alignment::Center)
        .block(
            Block::default()
                .borders(Borders::ALL)
                .border_style(Style::default().fg(app.theme.aqua())),
        );
    f.render_widget(title, chunks[0]);

    let content_chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(40), Constraint::Percentage(60)])
        .split(chunks[1]);

    let themes = Theme::all();
    let items: Vec<ListItem> = themes
        .iter()
        .enumerate()
        .map(|(i, theme)| {
            let is_selected = i == app.theme_selector_index;
            let is_current = theme == &app.theme;

            let content = if is_current {
                format!(
                    "{}● {}",
                    if is_selected { "> " } else { "  " },
                    theme.name()
                )
            } else {
                format!(
                    "{}  {}",
                    if is_selected { "> " } else { "  " },
                    theme.name()
                )
            };

            let style = if is_selected {
                Style::default()
                    .fg(theme.yellow())
                    .add_modifier(Modifier::BOLD)
            } else if is_current {
                Style::default().fg(theme.green())
            } else {
                Style::default().fg(app.theme.fg())
            };

            ListItem::new(content).style(style)
        })
        .collect();

    let list = List::new(items).block(
        Block::default()
            .borders(Borders::ALL)
            .border_style(Style::default().fg(app.theme.blue()))
            .title(" Themes "),
    );
    f.render_widget(list, content_chunks[0]);

    let preview_theme = themes[app.theme_selector_index];
    let preview_lines = vec![
        Line::from(vec![
            Span::styled("Theme: ", Style::default().fg(app.theme.gray())),
            Span::styled(
                preview_theme.name(),
                Style::default()
                    .fg(preview_theme.yellow())
                    .add_modifier(Modifier::BOLD),
            ),
        ]),
        Line::from(""),
        Line::from(vec![Span::styled(
            "Colors:",
            Style::default().fg(app.theme.gray()),
        )]),
        Line::from(vec![
            Span::raw("  Red:    "),
            Span::styled("███", Style::default().fg(preview_theme.red())),
        ]),
        Line::from(vec![
            Span::raw("  Green:  "),
            Span::styled("███", Style::default().fg(preview_theme.green())),
        ]),
        Line::from(vec![
            Span::raw("  Yellow: "),
            Span::styled("███", Style::default().fg(preview_theme.yellow())),
        ]),
        Line::from(vec![
            Span::raw("  Blue:   "),
            Span::styled("███", Style::default().fg(preview_theme.blue())),
        ]),
        Line::from(vec![
            Span::raw("  Purple: "),
            Span::styled("███", Style::default().fg(preview_theme.purple())),
        ]),
        Line::from(vec![
            Span::raw("  Aqua:   "),
            Span::styled("███", Style::default().fg(preview_theme.aqua())),
        ]),
    ];

    let preview = Paragraph::new(preview_lines)
        .block(
            Block::default()
                .borders(Borders::ALL)
                .border_style(Style::default().fg(app.theme.purple()))
                .title(" Preview "),
        )
        .alignment(Alignment::Left);
    f.render_widget(preview, content_chunks[1]);

    let help_text = vec![
        Span::styled("Enter", Style::default().fg(app.theme.green())),
        Span::raw(": Apply  "),
        Span::styled("Esc", Style::default().fg(app.theme.red())),
        Span::raw(": Cancel"),
    ];

    let help = Paragraph::new(Line::from(help_text))
        .style(Style::default().fg(app.theme.gray()))
        .alignment(Alignment::Center)
        .block(
            Block::default()
                .borders(Borders::ALL)
                .border_style(Style::default().fg(app.theme.gray())),
        );
    f.render_widget(help, chunks[2]);
}

#[allow(clippy::too_many_lines)]
pub fn draw_options_menu(f: &mut Frame, area: Rect, app: &App) {
    use super::super::colors::ThemeColors;

    let block = Block::default()
        .borders(Borders::NONE)
        .style(Style::default().bg(app.theme.bg0()));
    f.render_widget(block, area);

    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .margin(0)
        .constraints([
            Constraint::Percentage(10), // top padding
            Constraint::Length(7),      // Pass title
            Constraint::Length(4),      // spacing
            Constraint::Length(3),      // options
            Constraint::Length(3),      // spacing
            Constraint::Length(3),      // help
            Constraint::Length(3),      // spacing
            Constraint::Length(3),      // quit
            Constraint::Percentage(10), // bottom padding
        ])
        .split(area);

    let title_lines = vec![
        Line::from(vec![Span::styled(
            "██████╗  █████╗ ███████╗███████╗",
            Style::default()
                .fg(app.theme.red())
                .add_modifier(Modifier::BOLD),
        )]),
        Line::from(vec![Span::styled(
            "██╔══██╗██╔══██╗██╔════╝██╔════╝",
            Style::default()
                .fg(app.theme.red())
                .add_modifier(Modifier::BOLD),
        )]),
        Line::from(vec![Span::styled(
            "██████╔╝███████║███████╗███████╗",
            Style::default()
                .fg(app.theme.red())
                .add_modifier(Modifier::BOLD),
        )]),
        Line::from(vec![Span::styled(
            "██╔═══╝ ██╔══██║╚════██║╚════██║",
            Style::default()
                .fg(app.theme.red())
                .add_modifier(Modifier::BOLD),
        )]),
        Line::from(vec![Span::styled(
            "██║     ██║  ██║███████║███████║",
            Style::default()
                .fg(app.theme.red())
                .add_modifier(Modifier::BOLD),
        )]),
        Line::from(vec![Span::styled(
            "╚═╝     ╚═╝  ╚═╝╚══════╝╚══════╝",
            Style::default()
                .fg(app.theme.red())
                .add_modifier(Modifier::BOLD),
        )]),
        Line::from(vec![Span::styled(
            "           v2.3.4",
            Style::default().fg(app.theme.gray()),
        )]),
    ];

    let title = Paragraph::new(title_lines).alignment(Alignment::Center);
    f.render_widget(title, chunks[1]);

    let options_style = if app.options_menu_index == 0 {
        Style::default()
            .fg(app.theme.orange())
            .add_modifier(Modifier::BOLD)
    } else {
        Style::default().fg(app.theme.gray())
    };

    let options_lines = vec![
        Line::from(vec![Span::styled(
            "▄▀▀▄ █▀▀█ ▀▀█▀▀ ▀█▀ ▄▀▀▄ █▀▀▄ █▀▀",
            options_style,
        )]),
        Line::from(vec![Span::styled(
            "█  █ █  █   █    █  █  █ █  █ ▀▀█",
            options_style,
        )]),
        Line::from(vec![Span::styled(
            " ▀▀  █▀▀▀   ▀   ▀▀▀  ▀▀  ▀  ▀ ▀▀▀",
            options_style,
        )]),
    ];

    let options = Paragraph::new(options_lines).alignment(Alignment::Center);
    f.render_widget(options, chunks[3]);

    let help_style = if app.options_menu_index == 1 {
        Style::default()
            .fg(app.theme.yellow())
            .add_modifier(Modifier::BOLD)
    } else {
        Style::default().fg(app.theme.gray())
    };

    let help_lines = vec![
        Line::from(vec![Span::styled("█  █ █▀▀▀ █   █▀▀█", help_style)]),
        Line::from(vec![Span::styled("█▀▀█ █▀▀  █   █  █", help_style)]),
        Line::from(vec![Span::styled("▀  ▀ ▀▀▀▀ ▀▀▀ █▀▀▀", help_style)]),
    ];

    let help = Paragraph::new(help_lines).alignment(Alignment::Center);
    f.render_widget(help, chunks[5]);

    let quit_style = if app.options_menu_index == 2 {
        Style::default()
            .fg(app.theme.yellow())
            .add_modifier(Modifier::BOLD)
    } else {
        Style::default().fg(app.theme.gray())
    };

    let quit_lines = vec![
        Line::from(vec![Span::styled("▄▀▀▄ █  █ ▀█▀ ▀▀█▀▀", quit_style)]),
        Line::from(vec![Span::styled("█  █ █  █  █    █  ", quit_style)]),
        Line::from(vec![Span::styled(" ▀▀   ▀▀  ▀▀▀   ▀  ", quit_style)]),
    ];

    let quit = Paragraph::new(quit_lines).alignment(Alignment::Center);
    f.render_widget(quit, chunks[7]);
}

pub fn draw_settings_screen(f: &mut Frame, area: Rect, app: &App) {
    use super::super::colors::ThemeColors;

    let centered_area = centered_rect(60, 50, area);

    let block = Block::default()
        .borders(Borders::ALL)
        .border_style(Style::default().fg(app.theme.blue()))
        .title("═══ SETTINGS ═══")
        .title_alignment(Alignment::Center)
        .style(Style::default().bg(app.theme.bg0()));

    let inner = block.inner(centered_area);
    f.render_widget(block, centered_area);

    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .margin(2)
        .constraints([Constraint::Length(3), Constraint::Min(5), Constraint::Length(3)])
        .split(inner);

    let title = Paragraph::new("Preferences")
        .style(
            Style::default()
                .fg(app.theme.yellow())
                .add_modifier(Modifier::BOLD),
        )
        .alignment(Alignment::Center);
    f.render_widget(title, chunks[0]);

    let current_theme = app.theme.name();
    let clipboard_timeout_display = if app.clipboard_timeout == 0 {
        "Never".to_string()
    } else {
        format!("{}s", app.clipboard_timeout)
    };
    
    let items = [
        ("1", "Theme Selector", format!("Current: {current_theme}")),
        ("2", "Clipboard Timeout", format!("Current: {clipboard_timeout_display}")),
        ("3", "Future Feature", "Coming soon".to_string()),
    ];

    let list_items: Vec<ListItem> = items
        .iter()
        .enumerate()
        .map(|(i, (key, title, value))| {
            let is_selected = i == app.settings_menu_index;

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
                    Span::styled(format!("[{key}] "), Style::default().fg(app.theme.orange())),
                    Span::styled(*title, style),
                ]),
                Line::from(vec![
                    Span::raw("     "),
                    Span::styled(value, Style::default().fg(app.theme.gray())),
                ]),
                Line::from(""),
            ];
            ListItem::new(lines)
        })
        .collect();

    let list = List::new(list_items);
    f.render_widget(list, chunks[1]);
    
    let help_text = "Press number or Enter to select | ↑↓: Navigate | Esc: Back";
    let help = Paragraph::new(help_text)
        .style(Style::default().fg(app.theme.gray()))
        .alignment(Alignment::Center);
    f.render_widget(help, chunks[2]);
}

#[allow(clippy::too_many_lines)]
pub fn draw_help_screen(f: &mut Frame, area: Rect, app: &App) {
    use super::super::colors::ThemeColors;

    let block = Block::default()
        .borders(Borders::ALL)
        .border_style(Style::default().fg(app.theme.green()))
        .title("═══ PASSLOCK HELP ═══")
        .title_alignment(Alignment::Center)
        .style(Style::default().bg(app.theme.bg0()));

    let inner = block.inner(area);
    f.render_widget(block, area);

    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .margin(2)
        .constraints([Constraint::Min(0), Constraint::Length(3)])
        .split(inner);

    let help_text = vec![
        Line::from(vec![Span::styled(
            "MAIN MENU",
            Style::default()
                .fg(app.theme.yellow())
                .add_modifier(Modifier::BOLD),
        )]),
        Line::from(vec![
            Span::styled("  1-7       ", Style::default().fg(app.theme.orange())),
            Span::raw("Quick select option"),
        ]),
        Line::from(vec![
            Span::styled("  ↑/↓       ", Style::default().fg(app.theme.orange())),
            Span::raw("Navigate options"),
        ]),
        Line::from(vec![
            Span::styled("  ←/→       ", Style::default().fg(app.theme.orange())),
            Span::raw("Switch section (left/right)"),
        ]),
        Line::from(vec![
            Span::styled("  Enter     ", Style::default().fg(app.theme.orange())),
            Span::raw("Select option"),
        ]),
        Line::from(vec![
            Span::styled("  T/8       ", Style::default().fg(app.theme.orange())),
            Span::raw("Theme selector"),
        ]),
        Line::from(vec![
            Span::styled("  Esc       ", Style::default().fg(app.theme.orange())),
            Span::raw("Options menu"),
        ]),
        Line::from(""),
        Line::from(vec![Span::styled(
            "PASSWORDS VIEW",
            Style::default()
                .fg(app.theme.yellow())
                .add_modifier(Modifier::BOLD),
        )]),
        Line::from(vec![
            Span::styled("  ↑/↓       ", Style::default().fg(app.theme.orange())),
            Span::raw("Navigate entries"),
        ]),
        Line::from(vec![
            Span::styled("  E         ", Style::default().fg(app.theme.orange())),
            Span::raw("Edit selected entry"),
        ]),
        Line::from(vec![
            Span::styled("  H         ", Style::default().fg(app.theme.orange())),
            Span::raw("View password history"),
        ]),
        Line::from(vec![
            Span::styled("  F         ", Style::default().fg(app.theme.orange())),
            Span::raw("Clear filters"),
        ]),
        Line::from(vec![
            Span::styled("  V         ", Style::default().fg(app.theme.orange())),
            Span::raw("View filtered passwords (when in Filter Tags screen)"),
        ]),
        Line::from(vec![
            Span::styled("  Esc       ", Style::default().fg(app.theme.orange())),
            Span::raw("Back to main menu"),
        ]),
        Line::from(""),
        Line::from(vec![Span::styled(
            "ADD/EDIT PASSWORD",
            Style::default()
                .fg(app.theme.yellow())
                .add_modifier(Modifier::BOLD),
        )]),
        Line::from(vec![
            Span::styled("  Tab       ", Style::default().fg(app.theme.orange())),
            Span::raw("Next field"),
        ]),
        Line::from(vec![
            Span::styled("  Shift+Tab ", Style::default().fg(app.theme.orange())),
            Span::raw("Previous field"),
        ]),
        Line::from(vec![
            Span::styled("  Ctrl+S    ", Style::default().fg(app.theme.orange())),
            Span::raw("Save entry (from any field)"),
        ]),
        Line::from(vec![
            Span::styled("  Enter     ", Style::default().fg(app.theme.orange())),
            Span::raw("Save (except in Tags/Notes)"),
        ]),
        Line::from(vec![
            Span::styled("  Esc       ", Style::default().fg(app.theme.orange())),
            Span::raw("Cancel"),
        ]),
        Line::from(""),
        Line::from(vec![Span::styled(
            "SEARCH",
            Style::default()
                .fg(app.theme.yellow())
                .add_modifier(Modifier::BOLD),
        )]),
        Line::from(vec![
            Span::styled("  Type      ", Style::default().fg(app.theme.orange())),
            Span::raw("Search query"),
        ]),
        Line::from(vec![
            Span::styled("  Enter     ", Style::default().fg(app.theme.orange())),
            Span::raw("View results"),
        ]),
        Line::from(vec![
            Span::styled("  Esc       ", Style::default().fg(app.theme.orange())),
            Span::raw("Back to main menu"),
        ]),
        Line::from(""),
        Line::from(vec![Span::styled(
            "THEME SELECTOR",
            Style::default()
                .fg(app.theme.yellow())
                .add_modifier(Modifier::BOLD),
        )]),
        Line::from(vec![
            Span::styled("  ↑↓/j/k    ", Style::default().fg(app.theme.orange())),
            Span::raw("Navigate themes"),
        ]),
        Line::from(vec![
            Span::styled("  ←/→       ", Style::default().fg(app.theme.orange())),
            Span::raw("Quick switch themes"),
        ]),
        Line::from(vec![
            Span::styled("  Enter     ", Style::default().fg(app.theme.orange())),
            Span::raw("Apply theme"),
        ]),
        Line::from(vec![
            Span::styled("  Esc       ", Style::default().fg(app.theme.orange())),
            Span::raw("Cancel"),
        ]),
    ];

    let help = Paragraph::new(help_text)
        .alignment(Alignment::Left)
        .wrap(Wrap { trim: false });
    f.render_widget(help, chunks[0]);

    let footer = Paragraph::new("Press Esc or Q to close")
        .style(Style::default().fg(app.theme.gray()))
        .alignment(Alignment::Center);
    f.render_widget(footer, chunks[1]);
}