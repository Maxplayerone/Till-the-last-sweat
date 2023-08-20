use macroquad::prelude::*;

use crate::blocks;

pub struct Player {
    pub size: Vec2,
    pub color: Color,
    pub speed: f32,
    pub current_pos: Vec2,
    pub gravity: f32,
}
//(TODO): add collission checking for low y
fn is_colliding_with_blocks(player_pos: &Vec2, blocks: &Vec<blocks::Block>) -> bool {
    for block in blocks.iter() {
        if player_pos.x > block.pos.x
            && player_pos.x < block.pos.x + block.size.x
            && player_pos.y > block.pos.y - block.size.y / 2.0
        {
            return true;
        }
    }
    //println!("player pos {:?} block pos {:?} and size {:?}", player_pos, blocks[0].pos, blocks[0].size);
    false
}

impl Player {
    pub fn update(&mut self, blocks: &Vec<blocks::Block>) {
        //horizontal movement
        if is_key_down(KeyCode::A) {
            self.current_pos.x -= self.speed;
        }
        if is_key_down(KeyCode::D) {
            self.current_pos.x += self.speed;
        }
        //vertical movement
        //gravity
        if !is_colliding_with_blocks(
            &Vec2::new(self.current_pos.x, self.current_pos.y + self.gravity),
            blocks,
        ) {
            self.current_pos.y += self.gravity;
        }
    }

    pub fn render(&self) {
        draw_rectangle(
            self.current_pos.x,
            self.current_pos.y,
            self.size.x,
            self.size.y,
            self.color,
        );
    }
}
