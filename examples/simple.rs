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
    let mut glyphs = BufferGlyphs::from_bytes(ROBOTO).unwrap();
    let format = TextFormat::new(50);
    let pane = Pane::new()
        .with_rect([0.0, 0.0, 400.0, 400.0])
        .with_orientation(Orientation::Horizontal)
        .with_panes(vec![
            Pane::new()
                .with_contents(Contents::Text(MESSAGE1.to_string(), format))
                .with_margin(5.0),
            Pane::new().with_panes(vec![
                Pane::new()
                    .with_contents(Contents::Text(MESSAGE2.to_string(), format.right()))
                    .with_margin(5.0),
                Pane::new()
                    .with_contents(Contents::Text(MESSAGE3.to_string(), format.centered()))
                    .with_margin(5.0),
            ]),
        ]).with_margin(10.0)
        .fit_text(&mut glyphs);
    let mut buffer = RenderBuffer::new(pane.rect().width() as u32, pane.rect().height() as u32);
    buffer.clear([0.0, 0.0, 0.0, 1.0]);
    if let Some(Contents::Text(text, format)) = pane[0].contents() {
        justified_text(
            text,
            pane[0].rect(),
            *format,
            &mut glyphs,
            identity(),
            &mut buffer,
        ).unwrap();
    }
    if let Some(Contents::Text(text, format)) = pane[1][0].contents() {
        justified_text(
            text,
            pane[1][0].rect(),
            *format,
            &mut glyphs,
            identity(),
            &mut buffer,
        ).unwrap();
    }
    if let Some(Contents::Text(text, format)) = pane[1][1].contents() {
        justified_text(
            text,
            pane[1][1].rect(),
            *format,
            &mut glyphs,
            identity(),
            &mut buffer,
        ).unwrap();
    }
    buffer.save("simple.png").unwrap();
}
