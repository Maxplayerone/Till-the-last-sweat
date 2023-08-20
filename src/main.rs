use macroquad::prelude::*;

mod blocks;
mod player;

#[macroquad::main("ttls")]
async fn main() {
    let mut player = player::Player {
        size: Vec2::new(40.0, 40.0),
        color: Color::new(0.52, 0.98, 1.0, 1.0),
        speed: 7.0,
        gravity: 5.0,
        current_pos: Vec2::new(screen_width() / 2.0 - 20.0, screen_height() / 2.0 - 20.0),
    };
    let block = blocks::Block {
        size: Vec2::new(screen_width() - 60.0, 70.0),
        color: WHITE,
        pos: Vec2::new(30.0, screen_height() - 70.0),
    };
    let mut blocks = Vec::new();
    blocks.push(block);
    loop {
        clear_background(Color::new(0.33, 0.33, 0.33, 1.0));

        player.update(&blocks);

        player.render();
        for block in blocks.iter() {
            block.render();
        }

        if is_key_down(KeyCode::Escape) {
            break;
        }
        next_frame().await
    }
}
