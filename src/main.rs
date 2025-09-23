use macroquad::prelude::*;

fn window_conf() -> Conf {
    Conf {
        window_title: "My Car Game".to_owned(),
        window_width: 1024,
        window_height: 768,
        high_dpi: true,
        fullscreen: false,
        ..Default::default()
    }
}
#[macroquad::main(window_conf)]
async fn main() {
    loop {
        clear_background(LIGHTGRAY);

        draw_text("Hello, Macroquad!", 20.0, 20.0, 30.0, DARKBLUE);

        // finish frame
        next_frame().await
    }
}