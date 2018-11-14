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
    // Initialize a format
    let format = TextFormat::new(50);

    // Create a pane
    let pane = Pane::new()
        .with_rect([0.0, 0.0, 400.0, 300.0])
        .with_orientation(Orientation::Horizontal)
        .with_color([1.0; 4])
        // Add some sub-panes
        .with_panes(vec![
            // This pane will be on the left
            Pane::new()
                .with_contents(Contents::Text(MESSAGE1.to_string(), format))
                .with_margin(5.0),
            // This pane will be on the right, but it is split by more sub-panes
            Pane::new().with_panes(vec![
                // This pane will be on the top-right
                Pane::new()
                    .with_contents(Contents::Text(MESSAGE2.to_string(), format.right()))
                    .with_margin(5.0),
                // This pane will be on the bottom-right
                Pane::new()
                    .with_contents(Contents::Text(MESSAGE3.to_string(), format.centered()))
                    .with_margin(5.0),
            ]),
        ])
        .with_margin(10.0)
        .fit_text(&mut glyphs);

    // Create a RenderBuffer with the same size as the pane
    let mut buffer = RenderBuffer::new(pane.rect().width() as u32, pane.rect().height() as u32);
    buffer.clear([1.0, 1.0, 1.0, 1.0]);

    // Draw the pane to the buffer
    pane.draw(&mut glyphs, identity(), &mut buffer).unwrap();

    // Save the buffer
    buffer.save("simple.png").unwrap();
}
