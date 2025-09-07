use crate::core::point::{Link, Point};
use crate::core::tile::Tile;
use crate::pixpal::uvs;
use rand::Rng;
use rand::prelude::ThreadRng;
use std::collections::{HashMap, HashSet, VecDeque};

const SIZE_FACTOR: i32 = 8;

pub struct Terrain {
    points: Vec<Point>,
    links: Vec<Link>,
    pub(crate) gsize: i32,
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

    pub fn init(&mut self) {
        println!("Applying initial ...");
        let mut random = rand::rng();
        let mut point_collection: Vec<Point> = Vec::new();
        let mut link_collection: Vec<Link> = Vec::new();
        let mut position_set: HashSet<(i32, i32)> = HashSet::new();

        let center_point = Point::new(self.gsize / 2, self.gsize / 2, 2.0);
        point_collection.push(center_point);
        position_set.insert((center_point.x(), center_point.y()));

        while (point_collection.len() as i32) < SIZE_FACTOR * 2 {
            let new_point = Point::new(
                random.random_range(0..SIZE_FACTOR),
                random.random_range(0..SIZE_FACTOR),
                2.0,
            );

            self.dla_pixel_move(
                &mut random,
                &mut point_collection,
                &mut link_collection,
                &mut position_set,
                new_point,
            );
        }

        println!("{} point(s) in terrain", point_collection.len());
        println!("{} link(s) in terrain", link_collection.len());

        self.points = point_collection;
        self.links = link_collection;
    }

    fn upscale(&mut self) {
        println!("Upscaling ...");

        let mut point_collection: Vec<Point> = Vec::new();
        let mut link_collection: Vec<Link> = Vec::new();
        let mut position_set: HashSet<(i32, i32)> = HashSet::new();

        self.links.iter().for_each(|link| {
            let new_link: Link = Link::new(
                Point::new(link.p1().x() * 2, link.p1().y() * 2, 2.0),
                Point::new(link.p2().x() * 2, link.p2().y() * 2, 2.0),
            );
            let new_points = new_link.trace();
            link_collection.push(new_link);
            new_points.iter().for_each(|p| {
                if !p.is_in_position_set(&position_set) {
                    position_set.insert((p.x(), p.y()));
                    point_collection.push(p.clone());
                }
            });
        });

        println!("{} points(s) were generated", point_collection.len());

        self.links = link_collection;
        self.points = point_collection;
        self.gsize = self.gsize * 2;
    }

    pub fn upscale_n(&mut self, n: i32) {
        for _ in 0..n {
            self.upscale();

            let (point_collection, link_collection) = self.add_crisp_layer();

            println!("{} point(s) in terrain", point_collection.len());
            println!("{} link(s) in terrain", link_collection.len());

            self.points = point_collection;
            self.links = link_collection;
        }
    }

    fn add_crisp_layer(&mut self) -> (Vec<Point>, Vec<Link>) {
        println!("Adding crisp layer ...");

        let mut random = rand::rng();
        let mut point_collection: Vec<Point> = self.points.clone();
        let mut link_collection: Vec<Link> = self.links.clone();
        let mut position_set: HashSet<(i32, i32)> = HashSet::new();

        point_collection.iter().for_each(|p| {
            position_set.insert((p.x(), p.y()));
        });

        let target: i32 = point_collection.len() as i32;

        while (point_collection.len() as i32) < ((target + 1) * 2) {
            let new_point = Point::new(
                random.random_range(0..self.gsize),
                random.random_range(0..self.gsize),
                2.0,
            );

            self.dla_pixel_move(
                &mut random,
                &mut point_collection,
                &mut link_collection,
                &mut position_set,
                new_point,
            );
        }
        (point_collection, link_collection)
    }

    fn dla_pixel_move(
        &mut self,
        random: &mut ThreadRng,
        point_collection: &mut Vec<Point>,
        link_collection: &mut Vec<Link>,
        position_set: &mut HashSet<(i32, i32)>,
        mut new_point: Point,
    ) {
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

    pub fn complete_pattern(&mut self) {
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

        self.points = points;
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

    pub fn evaluate_height(&mut self, center: (i32, i32)) {
        println!("Evaluating height values ...");

        let mut point_map: HashMap<(i32, i32), i32> = HashMap::new();

        for point in &self.points {
            point_map.insert((point.x(), point.y()), -1);
        }

        let mut queue = VecDeque::new();
        let (cx, cy) = center;

        if let Some(v) = point_map.get_mut(&(cx, cy)) {
            *v = 0;
        }
        queue.push_back((cx, cy));

        let directions = [(1, 0), (-1, 0), (0, 1), (0, -1)];
        while let Some((x, y)) = queue.pop_front() {
            let current_dist = point_map[&(x, y)];
            for (dx, dy) in directions {
                let nx = x + dx;
                let ny = y + dy;
                if let Some(v) = point_map.get_mut(&(nx, ny)) {
                    if *v == -1 {
                        *v = current_dist + 1;
                        queue.push_back((nx, ny));
                    }
                }
            }
        }

        let mut new_points: Vec<Point> = Vec::new();
        for (&point, &dist) in &point_map {
            if dist >= 0 {
                let height: f32 = ((-(1.0 / 8.0) * dist as f32) + 32.0).max(1.0);
                new_points.push(Point::new(point.0, point.1, height));
            }
        }

        self.points = new_points;
    }
}
