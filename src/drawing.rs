use crate::state::AppState;
use druid::kurbo::{Line, Point, Size};
use druid::piet::{Color, RenderContext};
use druid::widget::Painter;

pub fn create_painter() -> Painter<AppState> {
    Painter::new(|ctx, data: &AppState, _env| {
        let size = ctx.size();
        draw_grid(ctx, size);
        draw_lines(ctx, data);
    })
}

fn draw_grid(ctx: &mut druid::piet::Piet, size: Size) {
    let grid_spacing = 20.0;
    let grid_color = Color::rgba(0.78, 0.78, 0.78, 0.5);

    let width = size.width;
    let height = size.height;

    let mut x = 0.0;
    while x <= width {
        let line = Line::new(Point::new(x, 0.0), Point::new(x, height));
        ctx.stroke(line, &grid_color, 1.0);
        x += grid_spacing;
    }

    let mut y = 0.0;
    while y <= height {
        let line = Line::new(Point::new(0.0, y), Point::new(width, y));
        ctx.stroke(line, &grid_color, 1.0);
        y += grid_spacing;
    }
}

fn draw_lines(ctx: &mut druid::piet::Piet, data: &AppState) {
    let stroke_color = Color::rgb8(255, 255, 0);

    for (i, line) in data.lines.lines.iter().enumerate() {
        let color = if Some(i) == data.selected_line {
            Color::rgb8(255, 0, 0)
        } else {
            stroke_color.clone()
        };
        ctx.stroke(line.0, &color, 2.0);
    }

    if let Some(line) = &data.current_line {
        ctx.stroke(line.0, &stroke_color, 2.0);
    }
}
