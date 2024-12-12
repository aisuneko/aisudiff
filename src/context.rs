use ratatui::{layout::Rect, text::Line, widgets::Paragraph};
use tui_textarea::TextArea;

#[derive(PartialEq)]
pub enum FocusMode {
    A,
    B,
}
#[derive(PartialEq)]
pub enum State {
    Edit,
    ShowDiff,
}
pub struct Context<'a> {
    pub state: State,
    pub focus: FocusMode,
    pub diff_a: TextArea<'a>,
    pub diff_b: TextArea<'a>,
    pub diff_a_result: Paragraph<'a>,
    pub diff_b_result: Paragraph<'a>,
    pub diff_a_area: Rect,
    pub diff_b_area: Rect,
    pub diff_a_result_text: Vec<Line<'a>>,
    pub diff_b_result_text: Vec<Line<'a>>,
}

impl Context<'_> {
    pub fn new() -> Self {
        Context {
            state: State::Edit,
            focus: FocusMode::A,
            diff_a: TextArea::default(),
            diff_b: TextArea::default(),
            diff_a_result: Paragraph::default(),
            diff_b_result: Paragraph::default(),
            diff_a_area: Rect::default(),
            diff_b_area: Rect::default(),
            diff_a_result_text: Vec::new(),
            diff_b_result_text: Vec::new(),
        }
    }
}
