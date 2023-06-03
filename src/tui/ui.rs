use crate::tui::app::App;
use ratatui::{
    backend::Backend,
    layout::Alignment,
    style::{Color, Style},
    widgets::Paragraph,
    Frame,
};

/// Renders the user interface widgets.
pub fn render<B: Backend>(_app: &mut App, frame: &mut Frame<'_, B>) {
    frame.render_widget(
        Paragraph::new("binsider")
            .style(Style::default().fg(Color::Cyan).bg(Color::Black))
            .alignment(Alignment::Center),
        frame.size(),
    )
}
