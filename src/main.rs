mod ui;
mod events;
use ui::UI;

fn main() {
    let mut ui = UI::new();
    ui.run();
}