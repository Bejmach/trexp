use ratatui::text::{Line, Span};

use crate::{app::App, theme::GaugeState};

pub fn build_gauge<'a>(app: &App, label_left: String, label_right: String, ratio: f32, width: u16, state: GaugeState) -> Line<'a>{
    let bar_width = width - app.theme.gauge_style.margin_left - app.theme.gauge_style.margin_right - 4;
    let filled_chars = (ratio * bar_width as f32) as usize;
    let empty_chars = bar_width as usize - filled_chars;

    let style = match state {
        GaugeState::Focus => app.theme.gauge_style.focus,
        GaugeState::FadedFocus => app.theme.gauge_style.faded_focus,
        GaugeState::Passive => app.theme.gauge_style.passive,
        GaugeState::FadedPassive => app.theme.gauge_style.faded_passive,
    };

    let span_vec = vec![
        Span::styled(format!("{:<1$}", label_left, app.theme.gauge_style.margin_left as usize), style),
        Span::styled(format!("{}", app.theme.gauge_style.border_char), style),
        Span::styled(app.theme.gauge_style.fill_char.to_string().repeat(filled_chars), style),
        Span::styled(app.theme.gauge_style.empty_char.to_string().repeat(empty_chars), style),
        Span::styled(format!("{}", app.theme.gauge_style.border_char), style),
        Span::styled(format!("{:>1$}", label_right, app.theme.gauge_style.margin_right as usize), style),
    ];

    Line::from(span_vec)
}
