use ratatui::{
    prelude::{Alignment, Frame},
    style::{Color, Style},
    widgets::{Block, BorderType, Borders, Paragraph},
};

use crate::app::{App, Mode, clock::HourFormat};

fn time(app: &App) -> Paragraph {
    let hour = app.clock.datetime.hour;
    let (hour_format, hour) = match app.clock.hour_format {
        HourFormat::Format12 => {
            if hour > 12 {
                ("PM ", hour - 12)
            } else {
                ("AM ", hour)
            }
        }
        HourFormat::Format24 => ("24h", hour),
    };
    Paragraph::new(format!(
        "{} {} {}\n{:02}:{:02} {:02}",
        hour_format,
        app.clock.datetime.day_of_week,
        app.clock.datetime.date,
        hour,
        app.clock.datetime.minute,
        app.clock.datetime.second,
    ))
}

pub fn render(app: &mut App, f: &mut Frame) {
    let background = if app.clock.illuminator {
        Color::Green
    } else {
        Color::Black
    };

    let widget = match app.clock.mode {
        Mode::Timekeeping => time(app),
        _ => time(app),
    };

    f.render_widget(
        widget
            .block(
                Block::default()
                    .title("CASIO Alarm Chrono")
                    .title_alignment(Alignment::Center)
                    .borders(Borders::ALL)
                    .border_type(BorderType::Rounded),
            )
            .style(Style::default().fg(Color::White).bg(background))
            .alignment(Alignment::Left),
        f.size(),
    );
}
