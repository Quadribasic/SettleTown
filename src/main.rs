use macroquad::prelude::*;

fn window_config() -> Conf {
    Conf {
        window_title: "SettleTown".to_owned(),
        fullscreen: false,
        // high_dpi: true,
        window_width: 1200,
        window_height: 1200,
        ..Default::default()
    }
}

#[macroquad::main(window_config)]
async fn main() {
    // Load the texture data we want from assets/
    let desert_tile_texture = Texture2D::from_file_with_format(
            include_bytes!("../assets/desert-tiles/plain-desert-tile-1.png"),
            None,
    );

    'gameloop: loop {
        clear_background(BLACK);

        let texture_width = 64.;
        let texture_height = 64.;

        for i in 0..19 {
            for j in 0..19 {
            draw_texture_ex(
                &desert_tile_texture,
                (i as f32) * texture_width,
                (j as f32) * texture_height,
                WHITE,
                DrawTextureParams {
                    dest_size: Some(vec2(texture_width, texture_height)),
                    ..Default::default()
                },
            );

            }
        }
        // draw_texture(&desert_tile_texture, 0., 0., WHITE);

        next_frame().await;

        if is_key_down(KeyCode::Escape) {
            break 'gameloop;
        }
    }
}