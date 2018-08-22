extern crate piston_window;
extern crate selenium;

use piston_window::*;
use selenium::*;

fn main() {
    let mut window: PistonWindow = WindowSettings::new("Simple", (800, 600))
        .exit_on_esc(true)
        .build()
        .unwrap();
    let mut glyphs = Glyphs::from_bytes(
        include_bytes!("roboto.ttf"),
        window.factory.clone(),
        TextureSettings::new(),
    ).unwrap();

    let mut ui: Widget<usize> = Widget::from(vec![
        Widget::label("Hello there Alex!"),
        Widget::from(vec![
            Widget::label("How").with_background_color([1.0, 0.0, 0.0, 1.0]),
            Widget::label("Are").with_background_color([0.0, 1.0, 0.0, 1.0]),
            Widget::label("You?").with_background_color([0.0, 0.0, 1.0, 1.0]),
        ]),
    ]);

    while let Some(event) = window.next() {
        let window_size = [window.size().width as f64, window.size().height as f64];
        window.draw_2d(&event, |c, g| {
            ui.draw(window_size, c.transform, g, &mut glyphs);
        });
    }
}
