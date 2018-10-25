extern crate pane;

use pane::prelude::*;

fn main() {
    let pane: Pane = Pane::new()
        .with_size([100.0, 100.0])
        .with_margin(10.0)
        .split_in_half();
    println!("{:#?}", pane);

    let pane: Pane = Pane::new()
        .with_orientation(Orientation::Horizontal)
        .with_size([100.0, 100.0])
        .split_weighted_named(vec![("Tree", 0.2), ("Editor", 0.6), ("Git", 0.2)]);
    println!("{:#?}", pane["Editor"]);
}
