use std::time::Duration;

use symbols::line;
use tui::backend::Backend;
use tui::layout::{Alignment, Constraint, Direction, Layout, Rect};
use tui::style::{Color, Modifier, Style};
use tui::text::{Span, Spans};
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
                Constraint::Min(10),
                Constraint::Length(3),
                Constraint::Length(12),
            ]
            .as_ref(),
        )
        .split(size);

    let title = draw_title();
    rect.render_widget(title, chunks[0]);

    let body_chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Min(20), Constraint::Length(32)].as_ref())
        .split(chunks[1]);

    let body = draw_body(app.is_loading(), app.state());
    rect.render_widget(body, body_chunks[0]);

    if let Some(duration) = app.state().duration() {
        let duration_block = draw_duration(duration);
        rect.render_widget(duration_block, chunks[2]);
    }

    let logs = draw_logs();
    rect.render_widget(logs, chunks[3]);
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

fn draw_body<'a>(loading: bool, state: &AppState) -> Paragraph<'a> {
    let initialized_text = if state.is_initialized() {
        ""
    } else {
        "Not Initialized !"
    };
    let loading_text = if loading { "Loading..." } else { "" };

    let typing = if let Some(typed_text) = state.typed_text() {
        format!("{}", typed_text)
    } else {
        String::default()
    };

    let typing_information = if let Some(typing_information) = state.typing_information() {
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

    Paragraph::new(vec![
        Spans::from(Span::raw(initialized_text)),
        Spans::from(Span::raw(loading_text)),
        Spans::from(Span::styled(typing_information.from, Style::default().fg(Color::Yellow).add_modifier(Modifier::ITALIC))),
        Spans::from(Span::styled(typing_information.url, Style::default().fg(Color::Blue).add_modifier(Modifier::ITALIC))),
        Spans::from(Span::raw("")),
        Spans::from(Span::raw(typing_information.content)),
        Spans::from(Span::raw("")),
        Spans::from(Span::raw(typing)),
    ])
    .style(Style::default().fg(Color::LightCyan))
    .alignment(Alignment::Left)
    .block(
        Block::default()
            .borders(Borders::ALL)
            .style(Style::default().fg(Color::White))
            .border_type(BorderType::Plain),
    )
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
