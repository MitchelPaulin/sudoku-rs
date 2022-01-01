mod events;
mod puzzle;
mod ui;
use ui::UI;
mod themes;

fn main() {
    let mut ui = UI::new();
    ui.run();
}
