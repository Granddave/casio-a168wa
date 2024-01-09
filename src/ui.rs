use ratatui::{
    prelude::{Alignment, Frame},
    style::{Color, Style},
    widgets::{Block, BorderType, Borders, Paragraph},
};

use crate::app::App;

pub fn render(app: &mut App, f: &mut Frame) {
    f.render_widget(
        Paragraph::new(format!(
            "{} {}\n{:02}:{:02} {:02}",
            app.datetime.day_of_week,
            // app.datetime.month,
            app.datetime.date,
            app.datetime.hour,
            app.datetime.minute,
            app.datetime.second,
        ))
        .block(
            Block::default()
                .title("CASIO Alarm Chrono")
                .title_alignment(Alignment::Center)
                .borders(Borders::ALL)
                .border_type(BorderType::Rounded),
        )
        .style(Style::default().fg(Color::Yellow))
        .alignment(Alignment::Left),
        f.size(),
    )
}
