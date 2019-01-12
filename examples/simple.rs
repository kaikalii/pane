use graphics_buffer::*;
use pane::prelude::*;

static ROBOTO: &'static [u8] = include_bytes!("roboto.ttf");

const MESSAGE1: &str =
    "Somebody once told me the world is gonna role me. I ain't the sharpest tool in the shed.";

const MESSAGE2: &str = "She was lookin' kinda dumb with her finger and her thumb";

const MESSAGE3: &str = "in the shape of an 'L' on her forehead.";

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
                .with_contents(Contents::text(MESSAGE1, format))
                .with_margin(5.0),
            // This pane will be on the right, but it is split into more sub-panes
            Pane::new()
                .with_color(color::WHITE)
                .with_margin(5.0)
                .with_panes(vec![
                    // This pane will be in the top-right
                    Pane::new()
                        .with_color(color::GREEN)
                        .with_contents(Contents::text(MESSAGE2, format.right()))
                        .with_margin(5.0),
                    // This pane will be in the bottom-right
                    Pane::new()
                        .with_color(color::BLUE)
                        .with_contents(Contents::text(MESSAGE3, format.centered()))
                        .with_margin(5.0),
                ]),
        ])
        // Call this at the end
        .fit_text(&mut glyphs);

    // Create a RenderBuffer with the same size as the pane
    let mut buffer = RenderBuffer::new(pane.rect().width() as u32, pane.rect().height() as u32);
    buffer.clear([1.0, 1.0, 1.0, 1.0]);

    // Draw the pane to the buffer
    pane.draw(&mut glyphs, IDENTITY, &mut buffer).unwrap();

    // Save the buffer
    buffer.save("simple.png").unwrap();
}
