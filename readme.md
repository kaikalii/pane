
[on crates.io](https://crates.io/crates/pane)

[API Documentation](https://docs.rs/pane/)

### Description

This crate provides a data structure for text alignment. Rectangular `Pane`s, which may have smaller child `Pane`s, can be defined, and the positions of characters of text within them can be calculated.

The `graphics` feature, which is on by default, allows the direct rendering of a `Pane` with the `piston2d-graphics` crate.

### Example

The following example creates a simple `Pane` tree where some nodes contain formatted text. The `Pane` is then drawn using my [`graphics_buffer`](https://github.com/kaikalii/graphics_buffer) crate, and the image is saved to a file.

```rust
extern crate graphics_buffer;
extern crate pane;

use graphics_buffer::*;
use pane::prelude::*;

static ROBOTO: &'static [u8] = include_bytes!("roboto.ttf");

static MESSAGE1: &'static str =
    "Somebody once told me the world is gonna role me. I ain't the sharpest tool in the shed.";

static MESSAGE2: &'static str = "She was lookin' kinda dumb with her finger and her thumb";

static MESSAGE3: &'static str = "in the shape of an 'L' on her forehead.";

fn main() {
    // Initialize the glyphs
    let mut glyphs = BufferGlyphs::from_bytes(ROBOTO).unwrap();

    // Initialize a text format
    let format = TextFormat::new(50).color(color::WHITE);

    // Create a pane
    let pane = Pane::new()
        .with_rect([0.0, 0.0, 400.0, 300.0])
        .with_color(color::BLACK)
        .with_margin(10.0)
        .with_orientation(Orientation::Horizontal)
        // Add some sub-panes
        .with_panes(vec![
            // This pane will be on the left
            Pane::new()
                .with_color(color::RED)
                .with_contents(Contents::Text(MESSAGE1.to_string(), format))
                .with_margin(5.0),
            // This pane will be on the right, but it is split into more sub-panes
            Pane::new()
                .with_color(color::WHITE)
                .with_margin(5.0)
                .with_panes(vec![
                    // This pane will be in the top-right
                    Pane::new()
                        .with_color(color::GREEN)
                        .with_contents(Contents::Text(MESSAGE2.to_string(), format.right()))
                        .with_margin(5.0),
                    // This pane will be in the bottom-right
                    Pane::new()
                        .with_color(color::BLUE)
                        .with_contents(Contents::Text(MESSAGE3.to_string(), format.centered()))
                        .with_margin(5.0),
                ]),
        ])
        .fit_text(&mut glyphs);

    // Create a RenderBuffer with the same size as the pane
    let mut buffer = RenderBuffer::new(pane.rect().width() as u32, pane.rect().height() as u32);
    buffer.clear([1.0, 1.0, 1.0, 1.0]);

    // Draw the pane to the buffer
    pane.draw(&mut glyphs, identity(), &mut buffer).unwrap();

    // Save the buffer
    buffer.save("simple.png").unwrap();
}

```

This example creates the image below. Notice the different text justifications.

![Hey now!](https://github.com/kaikalii/pane/blob/master/simple.png)
