use macroquad::prelude::*;

mod animate_cars;
mod brain;
mod cars;
mod graphics;
mod types;

use brain::*;
use graphics::*;
use types::*;

fn window_conf() -> Conf {
    Conf {
        window_title: "Road Intersection".to_string(),
        window_width: G_WIDTH as i32,
        window_height: G_HEIGHT as i32,
        window_resizable: false,
        fullscreen: false,
        ..Default::default()
    }
}

#[macroquad::main(window_conf)]
async fn main() {
    let grass: Texture2D = load_texture("assets/background.png").await.unwrap();
    let sprites = Sprites::load().await;
    let mut lanes = Lane::create_all_lanes();

    loop {
        draw_window(&grass);
        let dt = get_frame_time();

        handle_input(&mut lanes);
        update_draw_lanes(&mut lanes, dt, &sprites);
        remove_out_of_bounds_vehicles(&mut lanes);

        next_frame().await
    }
}
