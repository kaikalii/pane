extern crate pane;

use pane::prelude::*;

fn main() {
    let pane: Pane = Pane::new()
        .with_size([100.0, 100.0])
        .with_margin(10.0)
        .split_in_half();
    println!("{:#?}", pane);
}
