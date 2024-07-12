use druid::kurbo::Line;
use druid::{Data, Lens};
use im::Vector;

#[derive(Clone, Data, Lens)]
pub struct AppState {
    pub current_line: Option<LineWrapper>,
    pub lines: WrappedVector,
    pub selected_line: Option<usize>,
}

impl AppState {
    pub fn new() -> Self {
        AppState {
            current_line: None,
            lines: WrappedVector { lines: Vector::new() },
            selected_line: None,
        }
    }
}

#[derive(Clone)]
pub struct LineWrapper(pub Line);

impl Data for LineWrapper {
    fn same(&self, other: &Self) -> bool {
        self.0.p0 == other.0.p0 && self.0.p1 == other.0.p1
    }
}

#[derive(Clone, Lens)]
pub struct WrappedVector {
    pub lines: Vector<LineWrapper>,
}

impl Data for WrappedVector {
    fn same(&self, other: &Self) -> bool {
        self.lines.len() == other.lines.len() && self.lines.iter().zip(other.lines.iter()).all(|(a, b)| a.same(b))
    }
}
