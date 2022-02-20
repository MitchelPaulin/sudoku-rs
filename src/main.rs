mod events;
mod puzzle;
mod ui;
use ui::UI;
mod themes;
mod puzzle_transformer;

fn main() {
    let mut ui = UI::new();
    ui.run();
}
