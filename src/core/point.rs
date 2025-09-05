use std::collections::HashSet;

#[derive(Clone, Copy)]
pub struct Point {
    x: i32,
    y: i32,
    height: f32,
}

impl Point {
    pub fn new(x: i32, y: i32, height: f32) -> Point {
        Self { x, y, height }
    }

    pub fn is_in_position_set(&self, position_set: &HashSet<(i32, i32)>) -> bool {
        position_set.contains(&(self.x, self.y))
    }

    pub fn is_touching_any(&self, position_set: &HashSet<(i32, i32)>) -> bool {
        position_set.contains(&(self.x + 1, self.y))
            || position_set.contains(&(self.x - 1, self.y))
            || position_set.contains(&(self.x, self.y + 1))
            || position_set.contains(&(self.x, self.y - 1))
    }

    pub fn add_x(&mut self, x: i32, cap: i32) {
        self.x += x;
        if self.x >= cap {
            self.x = 0
        } else if self.x < 0 {
            self.x = cap - 1
        }
    }

    pub fn add_y(&mut self, y: i32, cap: i32) {
        self.y += y;
        if self.y >= cap {
            self.y = 0
        } else if self.y < 0 {
            self.y = cap - 1
        }
    }

    pub fn x(&self) -> i32 {
        self.x
    }

    pub fn y(&self) -> i32 {
        self.y
    }

    pub fn height(&self) -> f32 {
        self.height
    }

    pub fn set_height(&mut self, height: f32) {
        self.height = height;
    }
}
