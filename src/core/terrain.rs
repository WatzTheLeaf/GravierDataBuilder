use crate::core::point::{Link, Point};
use crate::core::tile::Tile;
use crate::pixpal::uvs;
use rand::Rng;
use std::collections::HashSet;

const SIZE_FACTOR: i32 = 8;

pub struct Terrain {
    points: Vec<Point>,
    links: Vec<Link>,
    gsize: i32,
}

impl Terrain {
    pub fn new(gsize: i32) -> Self {
        println!("Generating a new terrain ...");

        Self {
            points: vec![],
            links: vec![],
            gsize: gsize * SIZE_FACTOR,
        }
    }

    pub fn init(&mut self) -> Self {
        println!("Applying initial ...");
        let mut random = rand::rng();
        let mut point_collection: Vec<Point> = Vec::new();
        let mut link_collection: Vec<Link> = Vec::new();
        let mut position_set: HashSet<(i32, i32)> = HashSet::new();

        let center_point = Point::new(self.gsize / 2, self.gsize / 2, 2.0);
        point_collection.push(center_point);
        position_set.insert((center_point.x(), center_point.y()));

        while (point_collection.len() as i32) < SIZE_FACTOR * 3 {
            let mut new_point = Point::new(
                random.random_range(0..SIZE_FACTOR),
                random.random_range(0..SIZE_FACTOR),
                2.0,
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
                new_point.set_value(2.0);
                link_collection.push(
                    new_point
                        .find_link(&position_set)
                        .expect("Link creation failed"),
                );
                point_collection.push(new_point);
                position_set.insert((new_point.x(), new_point.y()));
            }
        }

        println!("{} point(s) in terrain", point_collection.len());
        println!("{} link(s) in terrain", link_collection.len());
        Self {
            points: point_collection,
            links: link_collection,
            gsize: self.gsize,
        }
    }

    fn upscale(&self) -> Terrain {
        let mut point_collection: Vec<Point> = Vec::new();
        let mut link_collection: Vec<Link> = Vec::new();
        self.links.iter().for_each(|link| {
            let new_link: Link = Link::new(
                Point::new(link.p1().x() * 2, link.p1().y() * 2, 2.0),
                Point::new(link.p2().x() * 2, link.p2().y() * 2, 2.0),
            );
            let new_points = new_link.trace();
            link_collection.push(new_link);
            point_collection.append(&mut new_points.clone());
        });
        Self {
            points: point_collection,
            links: link_collection,
            gsize: self.gsize * 2,
        }
    }

    pub fn upscale_n(&mut self, n: i32) -> Terrain {
        let mut r: Terrain = self.upscale();
        for _ in 0..(n - 1) {
            r = r.upscale();
        }
        r
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

        println!("{} point(s) in terrain", points.len());

        Self {
            points,
            links: self.links.clone(),
            gsize: self.gsize,
        }
    }

    pub fn values_as_tile_vector(self) -> Vec<Tile> {
        println!("Converting points to tiles ...");

        let tiles: Vec<Tile> = self
            .points
            .iter()
            .map(|p| {
                Tile::new(
                    p.x(),
                    p.y(),
                    {
                        if p.value() > 1.0 {
                            uvs::GLOW_YELLOW.u
                        } else {
                            uvs::MIRROR_BLACK.u
                        }
                    },
                    {
                        if p.value() > 1.0 {
                            uvs::GLOW_YELLOW.v
                        } else {
                            uvs::MIRROR_BLACK.v
                        }
                    },
                    p.value(),
                )
            })
            .collect();

        println!("{} tiles were generated", tiles.len());
        tiles
    }
}
