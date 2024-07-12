use crate::drawing::create_painter;
use crate::state::{AppState, LineWrapper};
use druid::kurbo::{Line, Point};
use druid::widget::{Button, Flex};
use druid::{Env, Event, EventCtx, Widget, WidgetExt};
use std::time::{Duration, Instant};

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
        .with_flex_child(painter.controller(MouseController::new()), 1.0)
        .with_child(delete_button.fix_height(50.0).padding(10.0))
}

struct MouseController {
    last_click: Option<Instant>,
}

impl MouseController {
    pub fn new() -> Self {
        Self { last_click: None }
    }

    fn handle_mouse_down(&mut self, ctx: &mut EventCtx, data: &mut AppState, mouse_event: &druid::MouseEvent) {
        let pos = mouse_event.pos;

        // Check for double-click
        let double_click = self.last_click.map_or(false, |last| last.elapsed() < Duration::from_millis(500));

        if double_click {
            // Attach to nearest point if within 2 pixels
            for (_i, line) in data.lines.lines.iter().enumerate() {
                if line_contains_point(&line, pos, 2.0) {
                    data.selected_point = Some(closest_point(&line, pos));
                    break;
                }
            }
        } else if let Some(selected_point) = data.selected_point {
            // Start a new line from the selected point
            if selected_point.distance(pos) <= 5.0 {
                data.current_line = Some(LineWrapper(Line::new(selected_point, pos)));
            } else {
                data.selected_point = None;
            }
        } else {
            // Start a new line if no double-click
            data.selected_line = None;
            data.selected_point = None;
            for (_i, line) in data.lines.lines.iter().enumerate() {
                if line_contains_point(&line, pos, 5.0) {
                    data.selected_line = Some(_i);
                    break;
                }
            }
            if data.selected_line.is_none() {
                data.current_line = Some(LineWrapper(Line::new(pos, pos)));
            }
        }

        self.last_click = Some(Instant::now());
        ctx.request_paint();
    }

    fn handle_mouse_move(&mut self, ctx: &mut EventCtx, data: &mut AppState, mouse_event: &druid::MouseEvent) {
        let pos = mouse_event.pos;

        if let Some(ref mut line) = data.current_line {
            line.0.p1 = pos;
            data.selected_point = None;

            // Check if within snapping distance
            for (i, existing_line) in data.lines.lines.iter().enumerate() {
                if i != data.selected_line.unwrap_or(usize::MAX) {
                    if let Some(point) = closest_point_if_near(existing_line, pos, 2.0) {
                        line.0.p1 = point;
                        data.selected_point = Some(point);
                        break;
                    }
                }
            }
            ctx.request_paint();
        }
    }

    fn handle_mouse_up(&mut self, ctx: &mut EventCtx, data: &mut AppState, _mouse_event: &druid::MouseEvent) {
        if let Some(line) = data.current_line.take() {
            let completed_line = LineWrapper(Line::new(line.0.p0, line.0.p1));
            data.lines.lines.push_back(completed_line);
            data.selected_point = None;
        }
        ctx.request_paint();
    }
}

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
            Event::MouseDown(mouse_event) => self.handle_mouse_down(ctx, data, mouse_event),
            Event::MouseMove(mouse_event) => self.handle_mouse_move(ctx, data, mouse_event),
            Event::MouseUp(mouse_event) => self.handle_mouse_up(ctx, data, mouse_event),
            _ => {}
        }
        child.event(ctx, event, data, _env);
    }
}

fn line_contains_point(line: &LineWrapper, point: Point, tolerance: f64) -> bool {
    let dist = line.0.p0.distance(point) + line.0.p1.distance(point) - line.0.p0.distance(line.0.p1);
    dist.abs() < tolerance
}

fn closest_point(line: &LineWrapper, point: Point) -> Point {
    if line.0.p0.distance(point) < line.0.p1.distance(point) {
        line.0.p0
    } else {
        line.0.p1
    }
}

fn closest_point_if_near(line: &LineWrapper, point: Point, tolerance: f64) -> Option<Point> {
    let p0_distance = line.0.p0.distance(point);
    let p1_distance = line.0.p1.distance(point);
    if p0_distance < tolerance {
        Some(line.0.p0)
    } else if p1_distance < tolerance {
        Some(line.0.p1)
    } else {
        None
    }
}
