mod error;
mod gui;
mod model;

pub use crate::error::Error;
use crate::gui::Win;
use crate::model::providers::TvSpielfilm;

use relm::Widget;

fn main() {
    Win::<TvSpielfilm>::run(()).expect("Could not spawn window");
}
