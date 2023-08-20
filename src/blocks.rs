use macroquad::prelude::*;

pub struct Block {
    pub size: Vec2,
    pub color: Color,
    pub pos: Vec2,
}

impl Block {
    pub fn render(&self) {
        draw_rectangle(self.pos.x, self.pos.y, self.size.x, self.size.y, self.color);
    }
}
