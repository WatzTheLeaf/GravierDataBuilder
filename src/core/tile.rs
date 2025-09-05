#[repr(C)]
pub struct Tile {
    posx: i32,
    posy: i32,
    u: f32,
    v: f32,
    height: f32,
}

#[allow(dead_code)]
impl Tile {
    pub fn new(posx: i32, posy: i32, u: f32, v: f32, height: f32) -> Self {
        Self {
            posx,
            posy,
            u,
            v,
            height,
        }
    }
    pub fn posx(&self) -> i32 {
        self.posx
    }
    pub fn posy(&self) -> i32 {
        self.posy
    }
    pub fn u(&self) -> f32 {
        self.u
    }
    pub fn v(&self) -> f32 {
        self.v
    }
    pub fn height(&self) -> f32 {
        self.height
    }
}
