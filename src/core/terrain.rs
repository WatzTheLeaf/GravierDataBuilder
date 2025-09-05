use crate::core::point::Point;
use crate::core::tile::Tile;
use rand::Rng;
use std::collections::HashSet;

pub struct Terrain {
    points: Vec<Point>,
    gsize: i32,
}

impl Terrain {
    pub fn new(gsize: i32) -> Terrain {
        println!("Generating a new terrain ...");
        Self {
            points: vec![],
            gsize,
        }
    }

    pub fn dla(&mut self, range: i32) -> Terrain {
        println!("Applying rpattern ...");

        let mut random = rand::rng();
        let mut point_collection: Vec<Point> = Vec::new();
        let mut position_set: HashSet<(i32, i32)> = HashSet::new();

        let center_point = Point::new(self.gsize / 2, self.gsize / 2, 50.0);
        point_collection.push(center_point);
        position_set.insert((center_point.x(), center_point.y()));

        while point_collection.len() < range as usize {
            let mut new_point = Point::new(
                random.random_range(0..self.gsize),
                random.random_range(0..self.gsize),
                100.0,
            );

            while !new_point.is_touching_any(&position_set)
                && !new_point.is_in_position_set(&position_set)
            {
                match random.random_range(0..4) {
                    0 => new_point.add_x(1, self.gsize),
                    1 => new_point.add_x(-1, self.gsize),
                    2 => new_point.add_y(1, self.gsize),
                    _ => new_point.add_y(-1, self.gsize),
                }
            }

            if !new_point.is_in_position_set(&position_set) {
                new_point.set_height(200.0);

                point_collection.push(new_point);
                position_set.insert((new_point.x(), new_point.y()));
            }
        }

        println!("{} Points were generated", point_collection.len());
        Self {
            points: point_collection,
            gsize: self.gsize,
        }
    }

    pub fn complete_pattern(&mut self) -> Terrain {
        println!("Applying complete pattern ...");

        let mut position_set: HashSet<(i32, i32)> = HashSet::new();
        for point in &self.points {
            position_set.insert((point.x(), point.y()));
        }

        let mut points: Vec<Point> = self.points.clone();

        for i in 0..self.gsize {
            for j in 0..self.gsize {
                if !position_set.contains(&(i, j)) {
                    points.push(Point::new(i, j, 1.0));
                }
            }
        }

        Self {
            points,
            gsize: self.gsize,
        }
    }

    pub fn values_as_tile_vector(self) -> Vec<Tile> {
        println!("Converting points to tiles ...");

        let tiles: Vec<Tile> = self
            .points
            .iter()
            .map(|p| Tile::new(p.x(), p.y(), 0.0, 0.0, p.height()))
            .collect();

        println!("{} tiles were generated", tiles.len());
        tiles
    }
}
