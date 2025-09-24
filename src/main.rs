use macroquad::prelude::*;

mod types;

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
    grass.set_filter(FilterMode::Nearest);
    loop {
        clear_background(BLACK);
        draw_texture_ex(
            &grass,
            0.0,
            0.0,
            WHITE,
            DrawTextureParams {
                dest_size: Some(vec2(screen_width(), screen_height())),
                ..Default::default()
            },
        );
        // North : Up
        draw_rectangle(
            G_HEIGHT / 2.0, // x
            0.0,            // y
            ROAD_WIDTH,     // width
            G_HEIGHT,       // width
            BLACK,
        );
        // South : Down
        draw_rectangle(
            G_HEIGHT / 2.0 - ROAD_WIDTH, // x
            0.0,                         // y
            ROAD_WIDTH,                  // width
            G_HEIGHT,                    // width
            DARKGRAY,
        );
        // East : Left
        draw_rectangle(
            0.0,                        // y
            G_WIDTH / 2.0 - ROAD_WIDTH, // x
            G_WIDTH,                    // width
            ROAD_WIDTH,                 // width
            DARKGRAY,
        );
        // West : Right
        draw_rectangle(
            0.0,           // y
            G_WIDTH / 2.0, // x
            G_WIDTH,       // width
            ROAD_WIDTH,    // width
            BLACK,
        );

        draw_horizontal_dashed_line(
            G_HEIGHT / 2.0 - 5.0, // y position
            0.0,                  // start x
            screen_width(),       // end x
            30.0,                 // dash length
            30.0,                 // gap
            6.0,                  // thickness
            WHITE,
        );

        // Vertical dashed line
        draw_vertical_dashed_line(
            G_WIDTH / 2.0 - 5.0, // x position
            0.0,                 // start y
            screen_height(),     // end y
            30.0,                // dash length
            30.0,                // gap
            6.0,                 // thickness
            WHITE,
        );
        // finish frame
        next_frame().await
    }
}

fn draw_horizontal_dashed_line(
    y: f32,
    x_start: f32,
    x_end: f32,
    dash_len: f32,
    gap: f32,
    thickness: f32,
    color: Color,
) {
    let mut x = x_start;
    let inter_left = G_WIDTH / 2.0 - ROAD_WIDTH;
    let inter_right = G_WIDTH / 2.0 + ROAD_WIDTH;

    while x < x_end {
        let dash_right = x + dash_len;

        // Only draw if the dash is fully outside the intersection zone
        if dash_right < inter_left || x > inter_right {
            draw_rectangle(x, y, dash_len, thickness, color);
        }

        x += dash_len + gap;
    }
}

fn draw_vertical_dashed_line(
    x: f32,
    y_start: f32,
    y_end: f32,
    dash_len: f32,
    gap: f32,
    thickness: f32,
    color: Color,
) {
    let mut y = y_start;
    let inter_top = G_HEIGHT / 2.0 - ROAD_WIDTH;
    let inter_bottom = G_HEIGHT / 2.0 + ROAD_WIDTH;

    while y < y_end {
        let dash_bottom = y + dash_len;

        // Only draw if the dash is fully outside the intersection zone
        if dash_bottom < inter_top || y > inter_bottom {
            draw_rectangle(x, y, thickness, dash_len, color);
        }

        y += dash_len + gap;
    }
}
