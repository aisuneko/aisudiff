use crate::context::{Context, FocusMode, State};
use ratatui::{
    layout::{
        Constraint::{Fill, Length},
        Layout,
    },
    style::{Color, Style},
    widgets::{Block, Paragraph},
    Frame,
};
pub fn render(frame: &mut Frame, ctx: &mut Context) {
    let vertical = Layout::vertical([Fill(1), Length(1)]);
    let [input_area, status_area] = vertical.areas(frame.area());
    let horizontal = Layout::horizontal([Fill(1); 2]);
    let [diff_a_area, diff_b_area] = horizontal.areas(input_area);
    ctx.diff_a.set_block(
        Block::bordered()
            .title("Diff A")
            .border_style(match ctx.focus {
                FocusMode::B => Style::default(),
                FocusMode::A => Style::default().fg(Color::Yellow),
            }),
    );

    ctx.diff_b.set_block(
        Block::bordered()
            .title("Diff B")
            .border_style(match ctx.focus {
                FocusMode::A => Style::default(),
                FocusMode::B => Style::default().fg(Color::Yellow),
            }),
    );
    ctx.diff_a_result = Paragraph::new(ctx.diff_a_result_text.clone()).block(
        Block::bordered()
            .title("Diff A - Result")
            .border_style(match ctx.focus {
                FocusMode::B => Style::default(),
                FocusMode::A => Style::default().fg(Color::Blue),
            }),
    );

    ctx.diff_b_result = Paragraph::new(ctx.diff_b_result_text.clone()).block(
        Block::bordered()
            .title("Diff B - Result")
            .border_style(match ctx.focus {
                FocusMode::A => Style::default(),
                FocusMode::B => Style::default().fg(Color::Blue),
            }),
    );
    let status_bar_message = match ctx.state {
        State::Edit => "aisudiff alpha - Press Ctrl-s to see diff, Esc to quit",
        State::ShowDiff => "aisudiff alpha - Press Esc to return to editor",
    };
    let status_bar = Paragraph::new(status_bar_message);
    frame.render_widget(status_bar, status_area);
    if ctx.state == State::Edit {
        frame.render_widget(&ctx.diff_a, diff_a_area);
        frame.render_widget(&ctx.diff_b, diff_b_area);
    } else {
        frame.render_widget(&ctx.diff_a_result, diff_a_area);
        frame.render_widget(&ctx.diff_b_result, diff_b_area);
    }
    ctx.diff_a_area = diff_a_area;
    ctx.diff_b_area = diff_b_area;
}
