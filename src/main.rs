mod events;
mod puzzle;
mod ui;
use ui::UI;

fn main() {
    let mut ui = UI::new();
    ui.run();
}
