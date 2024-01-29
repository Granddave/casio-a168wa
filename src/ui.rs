use ratatui::{
    prelude::{Alignment, Frame},
    style::{Color, Style},
    widgets::{Block, BorderType, Borders, Paragraph},
};

use crate::app::clock::datetime::DateTimeField;
pub use crate::app::{clock::datetime::HourFormat, clock::Mode, App};

fn hour_format(app: &App) -> (String, u8) {
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

    (hour_format.to_owned(), hour)
}

fn time(app: &App) -> Paragraph {
    let (hour_format, hour) = hour_format(app);
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

fn stopwatch(app: &App) -> Paragraph {
    let stopwatch = &app.clock.stopwatch;
    let (split_str, measurement) = match &stopwatch.split {
        Some(split) => ("SPLIT", split),
        None => ("", &stopwatch.measurement),
    };
    Paragraph::new(format!(
        "ST {}\n{:02}:{:02} {:02}",
        split_str, measurement.minutes, measurement.seconds, measurement.hundreds,
    ))
}

fn time_setting(app: &App) -> Paragraph {
    match app.clock.time_setting.selected_field {
        DateTimeField::Second | DateTimeField::Hour | DateTimeField::Minute => {
            Paragraph::new(format!(
                "  {} {}\n{:02}:{:02} {:02}",
                app.clock.datetime.day_of_week,
                app.clock.datetime.date,
                app.clock.datetime.hour,
                app.clock.datetime.minute,
                app.clock.datetime.second,
            ))
        }
        DateTimeField::Month | DateTimeField::Date | DateTimeField::DayOfWeek => {
            Paragraph::new(format!(
                "  {} {}\n {}",
                app.clock.datetime.day_of_week, app.clock.datetime.date, app.clock.datetime.month,
            ))
        }
    }
}

pub fn render(app: &mut App, f: &mut Frame) {
    let (fg, bg) = if app.clock.illuminator {
        (Color::Black, Color::Cyan)
    } else {
        (Color::White, Color::Black)
    };

    let widget = match app.clock.mode {
        Mode::Timekeeping => time(app),
        Mode::Stopwatch => stopwatch(app),
        Mode::TimeSetting => time_setting(app),
        _ => Paragraph::new(format!("{:?} not implemented", app.clock.mode)),
    };

    f.render_widget(
        widget
            .block(
                Block::default()
                    .title("CASIO")
                    .title_alignment(Alignment::Center)
                    .borders(Borders::ALL)
                    .border_type(BorderType::Rounded),
            )
            .style(Style::default().fg(fg).bg(bg))
            .alignment(Alignment::Left),
        f.size(),
    );
}
