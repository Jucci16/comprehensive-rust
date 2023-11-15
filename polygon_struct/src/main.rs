// TODO: remove this when you're done with your implementation.
#![allow(unused_variables, dead_code)]

use std::{f64::consts::PI, ops::Add, slice::Iter};

#[derive(PartialEq, Eq, Debug, Clone, Copy)]
pub struct Point {
    x: i32,
    y: i32,
}

impl Point {
    fn new(x: i32, y: i32) -> Point {
        Point { x, y }
    }

    // add methods
    fn magnitude(&self) -> f64 {
        let c = self.x.pow(2) + self.y.pow(2);
        (c as f64).sqrt()
    }

    fn dist(&self, right: Self) -> f64 {
        let a = self.x - right.x;
        let b = self.y - right.y;
        let c = a.pow(2) + b.pow(2);
        (c as f64).sqrt()
    }
}

impl Add for Point {
    type Output = Point;

    fn add(self, rhs: Self) -> Self::Output {
        Point::new(self.x + rhs.x, self.y + rhs.y)
    }
}

pub struct Polygon {
    points: Vec<Point>,
}

impl Polygon {
    fn new() -> Polygon {
        Polygon { points: Vec::new() }
    }

    fn left_most_point(&self) -> Option<Point> {
        self.points.iter().min_by_key(|x| x.x).copied()
    }

    fn add_point(&mut self, point: Point) {
        self.points.push(point);
    }

    fn iter(&mut self) -> Iter<Point> {
        self.points.iter()
    }

    fn perimeter(&self) -> f64 {
        let mut previous: Option<Point> = None;
        let mut perimeter: f64 = 0_f64;
        for point in &self.points {
            let current = *point;
            if previous == None {
                previous = Some(current);
                continue;
            }

            perimeter += previous.unwrap().dist(current);
            previous = Some(current);
        }

        perimeter += previous.unwrap().dist(self.points.first().unwrap().clone());
        perimeter
    }
}

impl From<Polygon> for Shape {
    fn from(value: Polygon) -> Self {
        Shape::Polygon(value)
    }
}

impl From<Circle> for Shape {
    fn from(value: Circle) -> Self {
        Shape::Circle(value)
    }
}

pub struct Circle {
    point: Point,
    radius: i32,
}

impl Circle {
    fn new(point: Point, radius: i32) -> Circle {
        Circle { point, radius }
    }

    fn circumference(&self) -> f64 {
        2_f64 * PI * (self.radius as f64)
    }
}

pub enum Shape {
    Polygon(Polygon),
    Circle(Circle),
}

impl Shape {
    fn perimeter(&self) -> f64 {
        match self {
            Shape::Circle(c) => c.circumference(),
            Shape::Polygon(p) => p.perimeter(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn round_two_digits(x: f64) -> f64 {
        (x * 100.0).round() / 100.0
    }

    #[test]
    fn test_point_magnitude() {
        let p1 = Point::new(12, 13);
        assert_eq!(round_two_digits(p1.magnitude()), 17.69);
    }

    #[test]
    fn test_point_dist() {
        let p1 = Point::new(10, 10);
        let p2 = Point::new(14, 13);
        assert_eq!(round_two_digits(p1.dist(p2)), 5.00);
    }

    #[test]
    fn test_point_add() {
        let p1 = Point::new(16, 16);
        let p2 = p1 + Point::new(-4, 3);
        assert_eq!(p2, Point::new(12, 19));
    }

    #[test]
    fn test_polygon_left_most_point() {
        let p1 = Point::new(12, 13);
        let p2 = Point::new(16, 16);

        let mut poly = Polygon::new();
        poly.add_point(p1);
        poly.add_point(p2);
        assert_eq!(poly.left_most_point(), Some(p1));
    }

    #[test]
    fn test_polygon_iter() {
        let p1 = Point::new(12, 13);
        let p2 = Point::new(16, 16);

        let mut poly = Polygon::new();
        poly.add_point(p1);
        poly.add_point(p2);

        let points = poly.iter().cloned().collect::<Vec<_>>();
        assert_eq!(points, vec![Point::new(12, 13), Point::new(16, 16)]);
    }

    #[test]
    fn test_shape_perimeters() {
        let mut poly = Polygon::new();
        poly.add_point(Point::new(12, 13));
        poly.add_point(Point::new(17, 11));
        poly.add_point(Point::new(16, 16));
        let shapes = vec![
            Shape::from(poly),
            Shape::from(Circle::new(Point::new(10, 20), 5)),
        ];
        let perimeters = shapes
            .iter()
            .map(Shape::perimeter)
            .map(round_two_digits)
            .collect::<Vec<_>>();
        assert_eq!(perimeters, vec![15.48, 31.42]);
    }
}

#[allow(dead_code)]
fn main() {}
