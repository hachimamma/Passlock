use super::super::app::App;
use super::super::colors::ThemeColors;
use super::super::screens::{InputField, MessageType};
use super::utility::centered_rect;
use crate::crypto;
use ratatui::{
    layout::{Alignment, Constraint, Direction, Layout, Rect},
    style::{Modifier, Style},
    widgets::{Block, Borders, Clear, Paragraph, Wrap},
    Frame,
};

pub fn draw_loading(f: &mut Frame, size: Rect, app: &App) {
    let area = centered_rect(50, 30, size);
    let block = Block::default()
        .borders(Borders::ALL)
        .border_style(Style::default().fg(app.theme.yellow()))
        .title(" PASSLOCK ")
        .title_alignment(Alignment::Center)
        .style(Style::default().bg(app.theme.bg0()));
    let text = Paragraph::new("Initializing vault...")
        .block(block)
        .alignment(Alignment::Center)
        .style(Style::default().fg(app.theme.fg()));
    f.render_widget(Clear, area);
    f.render_widget(text, area);
}

#[allow(clippy::cast_sign_loss)]
pub fn draw_create_vault(f: &mut Frame, size: Rect, app: &App) {
    let area = centered_rect(65, 70, size);
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .margin(2)
        .constraints([
            Constraint::Length(3),
            Constraint::Length(1),
            Constraint::Length(3),
            Constraint::Length(3),
            Constraint::Length(3),
            Constraint::Length(1),
            Constraint::Length(3),
            Constraint::Min(2),
            Constraint::Length(3),
        ])
        .split(area);

    let block = Block::default()
        .borders(Borders::ALL)
        .border_style(Style::default().fg(app.theme.orange()))
        .title("═══ CREATE VAULT ═══")
        .title_alignment(Alignment::Center)
        .style(Style::default().bg(app.theme.bg0()));
    f.render_widget(block, area);

    let title = Paragraph::new("Create your master password")
        .style(Style::default().fg(app.theme.yellow()))
        .alignment(Alignment::Center);
    f.render_widget(title, chunks[0]);

    let pwd_text = format!("Password: {}", "•".repeat(app.input_buffer.len()));
    let pwd_style = if app.input_field == InputField::Password {
        Style::default()
            .fg(app.theme.green())
            .add_modifier(Modifier::BOLD)
    } else {
        Style::default().fg(app.theme.gray())
    };
    let password_input = Paragraph::new(pwd_text).style(pwd_style);
    f.render_widget(password_input, chunks[2]);

    if !app.input_buffer.is_empty() && app.input_field == InputField::Password {
        let strength = crypto::calc_pwd_strength(&app.input_buffer);
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
        f.render_widget(strength_display, chunks[3]);

        if !strength.feedback.is_empty() {
            let feedback_text = format!("↳ {}", strength.feedback.join(", "));
            let feedback = Paragraph::new(feedback_text)
                .style(Style::default().fg(app.theme.gray()))
                .alignment(Alignment::Center)
                .wrap(Wrap { trim: true });
            f.render_widget(feedback, chunks[4]);
        }
    }

    let confirm_text = format!("Confirm: {}", "•".repeat(app.input_buffer2.len()));
    let confirm_style = if app.input_field == InputField::PasswordConfirm {
        Style::default()
            .fg(app.theme.green())
            .add_modifier(Modifier::BOLD)
    } else {
        Style::default().fg(app.theme.gray())
    };
    let confirm_input = Paragraph::new(confirm_text).style(confirm_style);
    f.render_widget(confirm_input, chunks[6]);

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
        f.render_widget(msg, chunks[7]);
    }

    let help = Paragraph::new("| Tab: Switch | Enter: Create | Esc: Quit |")
        .style(Style::default().fg(app.theme.gray()))
        .alignment(Alignment::Center);
    f.render_widget(help, chunks[8]);
}

pub fn draw_unlock_vault(f: &mut Frame, size: Rect, app: &App) {
    let area = centered_rect(60, 40, size);
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .margin(2)
        .constraints([
            Constraint::Length(3),
            Constraint::Length(1),
            Constraint::Length(3),
            Constraint::Min(2),
            Constraint::Length(3),
        ])
        .split(area);

    let block = Block::default()
        .borders(Borders::ALL)
        .border_style(Style::default().fg(app.theme.aqua()))
        .title("═══ UNLOCK VAULT ═══")
        .title_alignment(Alignment::Center)
        .style(Style::default().bg(app.theme.bg0()));
    f.render_widget(block, area);

    let title = Paragraph::new("Enter your master password")
        .style(Style::default().fg(app.theme.yellow()))
        .alignment(Alignment::Center);
    f.render_widget(title, chunks[0]);

    let pwd_text = format!("Password: {}", "•".repeat(app.input_buffer.len()));
    let password_input = Paragraph::new(pwd_text).style(
        Style::default()
            .fg(app.theme.green())
            .add_modifier(Modifier::BOLD),
    );
    f.render_widget(password_input, chunks[2]);

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
        f.render_widget(msg, chunks[3]);
    }

    let help = Paragraph::new("| Enter: Unlock | Esc: Quit |")
        .style(Style::default().fg(app.theme.gray()))
        .alignment(Alignment::Center);
    f.render_widget(help, chunks[4]);
}
