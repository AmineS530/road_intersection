use macroquad::prelude::*;

use crate::types::{G_HEIGHT, G_WIDTH, ROAD_WIDTH};

pub fn draw_horizontal_dashed_line(
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

pub fn draw_vertical_dashed_line(
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

pub fn draw_window(grass: &Texture2D) {
    draw_texture_ex(
        grass,
        0.0,
        0.0,
        WHITE,
        DrawTextureParams {
            dest_size: Some(vec2(screen_width(), screen_height())),
            ..Default::default()
        },
    );

    // North : Up
    draw_rectangle(G_HEIGHT / 2.0, 0.0, ROAD_WIDTH, G_HEIGHT, BLACK);
    // South : Down
    draw_rectangle(
        G_HEIGHT / 2.0 - ROAD_WIDTH,
        0.0,
        ROAD_WIDTH,
        G_HEIGHT,
        BLACK,
    );
    // East : Left
    draw_rectangle(0.0, G_WIDTH / 2.0 - ROAD_WIDTH, G_WIDTH, ROAD_WIDTH, BLACK);
    // West : Right
    draw_rectangle(0.0, G_WIDTH / 2.0, G_WIDTH, ROAD_WIDTH, BLACK);

    draw_horizontal_dashed_line(
        G_HEIGHT / 2.0 - 5.0,
        0.0,
        screen_width(),
        30.0,
        30.0,
        6.0,
        WHITE,
    );

    draw_vertical_dashed_line(
        G_WIDTH / 2.0 - 5.0,
        0.0,
        screen_height(),
        30.0,
        30.0,
        6.0,
        WHITE,
    );
}
