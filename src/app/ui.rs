use std::time::Duration;

use symbols::line;
use tui::backend::Backend;
use tui::layout::{Alignment, Constraint, Direction, Layout, Rect};
use tui::style::{Color, Modifier, Style};
use tui::text::{Span, Spans, Text};
use tui::widgets::{Block, BorderType, Borders, LineGauge, Paragraph};
use tui::{symbols, Frame};
use tui_logger::TuiLoggerWidget;

use super::state::AppState;
use crate::app::App;
use crate::io::file::TypingFileDisplay;

pub fn draw<B>(rect: &mut Frame<B>, app: &App)
where
    B: Backend,
{
    let size = rect.size();
    check_size(&size);

    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints(
            [
                Constraint::Length(3),
                Constraint::Length(20),
                Constraint::Length(10),
                Constraint::Length(3),
                Constraint::Length(5),
            ]
            .as_ref(),
        )
        .split(size);

    let title = draw_title();
    rect.render_widget(title, chunks[0]);

    let body_chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Min(5), Constraint::Min(5)].as_ref())
        .split(chunks[1]);

    let typing_information = if let Some(typing_information) = app.state().typing_information() {
        TypingFileDisplay {
            from: format!("From: {}", typing_information.from),
            url: format!("Url: {}", typing_information.url),
            content: typing_information.content,
            words_count: typing_information.words_count,
        }
    } else {    
        TypingFileDisplay {
            from: "".to_owned(),
            content: "".to_owned(),
            url: "".to_owned(),
            words_count: 0
        }
    };

    let body = draw_typing_information(typing_information.from, typing_information.url);
    rect.render_widget(body, body_chunks[0]);

    let long_text = draw_typing_text(typing_information.content);
    rect.render_widget(long_text, body_chunks[1]);

    let typing_from_user = draw_typing_from_user(app.state());
    rect.render_widget(typing_from_user, chunks[2]);

    if let Some(duration) = app.state().duration() {
        let duration_block = draw_duration(duration);
        rect.render_widget(duration_block, chunks[3]);
    }

    let logs = draw_logs();
    rect.render_widget(logs, chunks[4]);
}

fn draw_title<'a>() -> Paragraph<'a> {
    Paragraph::new("Jackdull")
        .style(Style::default().fg(Color::LightCyan))
        .alignment(Alignment::Center)
        .block(
            Block::default()
                .borders(Borders::ALL)
                .style(Style::default().fg(Color::White))
                .border_type(BorderType::Plain),
        )
}

fn check_size(rect: &Rect) {
    if rect.width < 52 {
        panic!("Require width >= 52, (got {})", rect.width);
    }
    if rect.height < 28 {
        panic!("Require height >= 28, (got {})", rect.height);
    }
}

fn draw_typing_information<'a>(typing_information_from: String, typing_information_url: String) -> Paragraph<'a> {
    Paragraph::new(vec![
        Spans::from(Span::styled(typing_information_from, Style::default().fg(Color::Yellow).add_modifier(Modifier::ITALIC))),
        Spans::from(Span::styled(typing_information_url, Style::default().fg(Color::Blue).add_modifier(Modifier::ITALIC))),
    ])
    .style(Style::default().fg(Color::LightCyan))
    .alignment(Alignment::Left)
}

fn draw_typing_text<'a>(text: String) -> Paragraph<'a> {
    let long_text = Text::from(text);

    Paragraph::new(long_text)
        .style(Style::default().fg(Color::LightCyan))
        .alignment(Alignment::Left)
}

fn draw_typing_from_user<'a>(state: &AppState) -> Paragraph<'a> {
    let typing = if let Some(typed_text) = state.typed_text() {
        format!("{}", typed_text)
    } else {
        String::default()
    };

    Paragraph::new(typing)
        .style(Style::default().fg(Color::LightCyan))
        .alignment(Alignment::Left)
}

fn draw_duration(duration: &Duration) -> LineGauge {
    let sec = duration.as_secs();
    let label = format!("{}s", sec);
    let ratio = sec as f64 / 60.0;

    LineGauge::default()
        .block(
            Block::default()
                .borders(Borders::ALL)
                .title("Timer"),
        )
        .gauge_style(
            Style::default()
                .fg(Color::Cyan)
                .bg(Color::Black)
                .add_modifier(Modifier::BOLD),
        )
        .line_set(line::THICK)
        .label(label)
        .ratio(ratio)
}

fn draw_logs<'a>() -> TuiLoggerWidget<'a> {
    TuiLoggerWidget::default()
        .style_error(Style::default().fg(Color::Red))
        .style_debug(Style::default().fg(Color::Green))
        .style_warn(Style::default().fg(Color::Yellow))
        .style_trace(Style::default().fg(Color::Gray))
        .style_info(Style::default().fg(Color::Blue))
        .block(
            Block::default()
                .title("Logs")
                .border_style(Style::default().fg(Color::White).bg(Color::Black))
                .borders(Borders::ALL),
        )
        .style(Style::default().fg(Color::White).bg(Color::Black))
}
