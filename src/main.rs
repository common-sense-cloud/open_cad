mod state;
mod drawing;
mod ui;

use crate::state::AppState;
use crate::ui::build_ui;
use druid::{AppLauncher, WindowDesc};

fn main() {
    let main_window = WindowDesc::new(build_ui)
        .title("OpenCAD")
        .window_size((800.0, 600.0));

    let initial_state = AppState::new();

    AppLauncher::with_window(main_window)
        .use_simple_logger()
        .launch(initial_state)
        .expect("Failed to launch application");
}
