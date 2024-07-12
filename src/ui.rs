use crate::drawing::create_painter;
use crate::state::{AppState, LineWrapper};
use druid::kurbo::Line;
use druid::kurbo::Point;
use druid::widget::{Button, Flex};
use druid::{Env, Event, EventCtx, Widget, WidgetExt};

pub fn build_ui() -> impl Widget<AppState> {
    let painter = create_painter();

    let delete_button = Button::new(|data: &AppState, _env: &Env| {
            if data.selected_line.is_some() {
                "Delete Selected Line".to_string()
            } else {
                "No Line Selected".to_string()
            }
        })
        .on_click(|_ctx, data: &mut AppState, _env| {
            if let Some(selected) = data.selected_line {
                data.lines.lines.remove(selected);
                data.selected_line = None;
            }
        });

    Flex::column()
        .with_flex_child(painter.controller(MouseController), 1.0)
        .with_child(delete_button.fix_height(50.0).padding(10.0))
}

struct MouseController;

impl<W: Widget<AppState>> druid::widget::Controller<AppState, W> for MouseController {
    fn event(
        &mut self,
        child: &mut W,
        ctx: &mut EventCtx,
        event: &Event,
        data: &mut AppState,
        _env: &Env,
    ) {
        match event {
            Event::MouseDown(mouse_event) => {
                let pos = mouse_event.pos;
                data.selected_line = None;
                for (i, line) in data.lines.lines.iter().enumerate() {
                    if line_contains_point(line, pos) {
                        data.selected_line = Some(i);
                        break;
                    }
                }
                if data.selected_line.is_none() {
                    data.current_line = Some(LineWrapper(Line::new(pos, pos)));
                }
                ctx.request_paint();
            }
            Event::MouseMove(mouse_event) => {
                if let Some(ref mut line) = data.current_line {
                    line.0.p1 = mouse_event.pos;
                    ctx.request_paint();
                } else if data.selected_line.is_some() {
                    ctx.set_cursor(&druid::Cursor::Crosshair);
                } else {
                    ctx.set_cursor(&druid::Cursor::Arrow);
                }
            }
            Event::MouseUp(mouse_event) => {
                if let Some(line) = data.current_line.take() {
                    let completed_line = LineWrapper(Line::new(line.0.p0, mouse_event.pos));
                    data.lines.lines.push_back(completed_line);
                }
                ctx.request_paint();
            }
            _ => {}
        }
        child.event(ctx, event, data, _env);
    }
}

fn line_contains_point(line: &LineWrapper, point: Point) -> bool {
    let tolerance = 5.0;
    let dist = line.0.p0.distance(point) + line.0.p1.distance(point) - line.0.p0.distance(line.0.p1);
    dist.abs() < tolerance
}
