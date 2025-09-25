use macroquad::prelude::*;

mod animate_cars;
mod brain;
mod cars;
mod graphics;
mod traffic_lights;
mod types;

use brain::*;
use graphics::*;
// use traffic_lights::*;
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
        for lane in lanes.values_mut() {
            lane.traffic_light.update(dt);

            let (x, y) = match lane.direction {
                Direction::North => (G_WIDTH / 2.0 - 10.0, 0.0),
                Direction::South => (G_WIDTH / 2.0 - 10.0, G_HEIGHT - 50.0),
                Direction::East => (G_WIDTH - 50.0, G_HEIGHT / 2.0 - 25.0),
                Direction::West => (0.0, G_HEIGHT / 2.0 - 25.0),
            };
            lane.traffic_light.draw(x, y);
            // lane.update_capacity_from_light();
        }
        next_frame().await
    }
}
