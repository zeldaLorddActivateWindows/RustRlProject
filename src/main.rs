use raylib::color::Color;
use raylib::RaylibHandle;
use raylib::RaylibThread;

fn main() {
    let (mut rl, thread): (RaylibHandle, RaylibThread) = raylib::init()
        .size(640, 480)
        .title("Hello, World")
        .build();

    while !rl.window_should_close() {
        let mut d = rl.begin_drawing(&thread);

        d.clear_background(Color::WHITE);
        d.draw_text("Hello, world!", 12, 12, 20, Color::BLACK);
    }
}