extern crate graphics_buffer;
extern crate pane;

use graphics_buffer::*;
use pane::prelude::*;

static ROBOTO: &'static [u8] = include_bytes!("roboto.ttf");

fn main() {
    let mut glyphs = BufferGlyphs::from_bytes(ROBOTO).unwrap();
    let mut buffer = RenderBuffer::new(250, 200);
    buffer.clear([0.0, 0.0, 0.0, 1.0]);
    justified_text(
        "Nice weather we are having, isn't it? \
         It's such a beautiful day. \
         The air is so fresh and the temperature is just right.",
        [0.0, 0.0, 250.0, 200.0],
        TextFormat::new(30),
        &mut glyphs,
        identity(),
        &mut buffer,
    ).unwrap();
    buffer.save("simple.png").unwrap();
}
