use crate::{
    context::{Context, FocusMode, State},
    ui::render,
};
use crossterm::{
    event::{self, Event, MouseButton, MouseEventKind},
    execute,
};
use ratatui::{
    layout::Position,
    style::{Color, Style},
    text::{Line, Span},
};
use similar::{ChangeTag, TextDiff};
use std::io;
use tui_textarea::{Input, Key};
fn handle_diff(ctx: &mut Context) {
    let diff_a_text = ctx.diff_a.lines().join("\n");
    let diff_b_text = ctx.diff_b.lines().join("\n");
    if !diff_a_text.is_empty() && !diff_b_text.is_empty() {
        let diff = TextDiff::from_lines(&diff_a_text, &diff_b_text);
        ctx.diff_a_result_text = Vec::new();
        ctx.diff_b_result_text = Vec::new();
        for change in diff.iter_all_changes() {
            match change.tag() {
                ChangeTag::Delete => {
                    ctx.diff_a_result_text.push(Line::from(Span::styled(
                        format!("- {}\n", change),
                        Style::default().fg(Color::Red),
                    )));
                }
                ChangeTag::Insert => {
                    ctx.diff_b_result_text.push(Line::from(Span::styled(
                        format!("+ {}\n", change),
                        Style::default().fg(Color::Green),
                    )));
                }
                ChangeTag::Equal => {
                    // Add to both results (unchanged)
                    ctx.diff_a_result_text
                        .push(Line::from(format!("  {}\n", change)));
                    ctx.diff_b_result_text
                        .push(Line::from(format!("  {}\n", change)));
                }
            }
        }
    }
}
fn handle_events(ctx: &mut Context) -> io::Result<bool> {
    match event::read()? {
        Event::Key(key) => match key.into() {
            Input {
                key: Key::Char('s'),
                ctrl: true,
                ..
            } => {
                ctx.state = State::ShowDiff;
                handle_diff(ctx);
                Ok(false)
            }
            Input { key: Key::Esc, .. } => match ctx.state {
                State::Edit => Ok(true),
                State::ShowDiff => {
                    ctx.state = State::Edit;
                    Ok(false)
                }
            },
            input => {
                if ctx.focus == FocusMode::A {
                    ctx.diff_a.input(input);
                } else {
                    ctx.diff_b.input(input);
                }
                Ok(false)
            }
        },
        Event::Mouse(mouse) if mouse.kind == MouseEventKind::Down(MouseButton::Left) => {
            let pos = Position::new(mouse.column, mouse.row);
            if ctx
                .diff_a
                .block()
                .is_some_and(|k| k.inner(ctx.diff_a_area).contains(pos))
            {
                ctx.focus = FocusMode::A;
            }
            if ctx
                .diff_b
                .block()
                .is_some_and(|k| k.inner(ctx.diff_b_area).contains(pos))
            {
                ctx.focus = FocusMode::B;
            }
            Ok(false)
        }
        _ => Ok(false),
    }
}
pub fn run(terminal: &mut ratatui::DefaultTerminal) -> io::Result<()> {
    execute!(
        std::io::stdout(),
        event::EnableFocusChange,
        event::EnableMouseCapture
    )?;
    let mut ctx = Context::new();
    loop {
        terminal.draw(|frame| render(frame, &mut ctx))?;
        if handle_events(&mut ctx)? {
            execute!(
                std::io::stdout(),
                event::DisableFocusChange,
                event::DisableMouseCapture
            )?;
            break Ok(());
        }
    }
}
