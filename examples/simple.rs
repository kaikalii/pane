extern crate graphics_buffer;
extern crate pane;

use graphics_buffer::*;
use pane::prelude::*;

static ROBOTO: &'static [u8] = include_bytes!("roboto.ttf");

static MESSAGE: &'static str = "Nice weather we are having, isn't it? \
                                It's such a beautiful day. \
                                The air is so fresh and the temperature is just right. \
                                This line would not normally fit.";

fn main() {
    let mut glyphs = BufferGlyphs::from_bytes(ROBOTO).unwrap();
    let mut rect = [0.0, 0.0, 250.0, 200.0];
    let mut format = TextFormat::new(30);
    rect[2] = glyphs.fit_min_width(MESSAGE, rect, format, 1.0);
    // format.font_size = glyphs.fit_max_font_size(MESSAGE, rect, format);
    let mut buffer = RenderBuffer::new(rect[2] as u32, rect[3] as u32);
    buffer.clear([0.0, 0.0, 0.0, 1.0]);
    justified_text(MESSAGE, rect, format, &mut glyphs, identity(), &mut buffer).unwrap();
    buffer.save("simple.png").unwrap();
}
