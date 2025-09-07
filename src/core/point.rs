use std::collections::HashSet;
use std::io::{Error, ErrorKind};

#[derive(Clone, Copy)]
pub struct Point {
    x: i32,
    y: i32,
    value: f32,
}

impl Point {
    pub fn new(x: i32, y: i32, value: f32) -> Point {
        Self { x, y, value }
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

    pub fn find_link(&self, position_set: &HashSet<(i32, i32)>) -> Result<Link, Error> {
        if position_set.contains(&(self.x + 1, self.y)) {
            return Ok(Link::new(self.clone(), Point::new(self.x + 1, self.y, 2.0)));
        }
        if position_set.contains(&(self.x - 1, self.y)) {
            return Ok(Link::new(self.clone(), Point::new(self.x - 1, self.y, 2.0)));
        }
        if position_set.contains(&(self.x, self.y + 1)) {
            return Ok(Link::new(self.clone(), Point::new(self.x, self.y + 1, 2.0)));
        }
        if position_set.contains(&(self.x, self.y - 1)) {
            return Ok(Link::new(self.clone(), Point::new(self.x, self.y - 1, 2.0)));
        }
        Err(Error::new(
            ErrorKind::InvalidInput,
            "Can't find a valid point to create a Link",
        ))
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

    pub fn value(&self) -> f32 {
        self.value
    }

    pub fn set_value(&mut self, value: f32) {
        self.value = value;
    }
}

#[derive(Clone, Copy)]
pub struct Link {
    p1: Point,
    p2: Point,
}

impl Link {
    pub fn new(p1: Point, p2: Point) -> Self {
        Self { p1, p2 }
    }

    pub fn p1(&self) -> Point {
        self.p1
    }

    pub fn p2(&self) -> Point {
        self.p2
    }

    pub fn trace(&self) -> Vec<Point> {
        let mut points = vec![self.p1, self.p2];
        if self.p1.x == self.p2.x {
            let (y1, y2) = if self.p1.y <= self.p2.y {
                (self.p1.y, self.p2.y)
            } else {
                (self.p2.y, self.p1.y)
            };
            for y in y1..=y2 {
                points.push(Point::new(self.p1.x, y, 2.0));
            }
        } else if self.p1.y == self.p2.y {
            let (x1, x2) = if self.p1.x <= self.p2.x {
                (self.p1.x, self.p2.x)
            } else {
                (self.p2.x, self.p1.x)
            };

            for x in x1..=x2 {
                points.push(Point::new(x, self.p1.y, 2.0));
            }
        }
        points
    }
}
