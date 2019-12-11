use core::iter;
use std::collections::HashSet;
use std::io;
use std::io::BufRead;
use std::iter::FromIterator;
use std::num::ParseIntError;
use std::str::FromStr;

#[derive(Eq, PartialEq, Hash, Default, Clone, Debug)]
struct Point {
    x: i32,
    y: i32,
}

impl Point {
    fn distance(&self) -> i32 {
        self.x.abs() + self.y.abs()
    }

    fn draw(&self, movement: &Movement) -> Vec<Point> {
        let to_point = |(x, y)| Point { x, y };
        match movement {
            Movement::Up(distance) => iter::repeat(self.x)
                .zip(self.y..=self.y + *distance as i32)
                .map(to_point)
                .collect(),
            Movement::Down(distance) => iter::repeat(self.x)
                .zip((self.y - *distance as i32..=self.y).rev())
                .map(to_point)
                .collect(),
            Movement::Left(distance) => (self.x - *distance as i32..=self.x)
                .rev()
                .zip(iter::repeat(self.y))
                .map(to_point)
                .collect(),
            Movement::Right(distance) => (self.x..=self.x + *distance as i32)
                .zip(iter::repeat(self.y))
                .map(to_point)
                .collect(),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct ParseMovementError {
    kind: MovementErrorKind,
}

#[derive(Debug, Clone, PartialEq, Eq)]
enum MovementErrorKind {
    InvalidDirection,
    ParseIntError(ParseIntError),
}

impl From<ParseIntError> for ParseMovementError {
    fn from(err: ParseIntError) -> Self {
        ParseMovementError {
            kind: MovementErrorKind::ParseIntError(err),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
enum Movement {
    Up(u32),
    Down(u32),
    Left(u32),
    Right(u32),
}

impl FromStr for Movement {
    type Err = ParseMovementError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match &s[..1] {
            "U" => Ok(Movement::Up(s[1..].parse()?)),
            "D" => Ok(Movement::Down(s[1..].parse()?)),
            "L" => Ok(Movement::Left(s[1..].parse()?)),
            "R" => Ok(Movement::Right(s[1..].parse()?)),
            _ => Err(ParseMovementError {
                kind: MovementErrorKind::InvalidDirection,
            }),
        }
    }
}

#[derive(Debug, PartialEq, Eq)]
struct Wire(HashSet<Point>);

impl Wire {
    fn new() -> Self {
        Wire(HashSet::new())
    }

    fn closest_intersection(&self, other: &Self) -> Option<Point> {
        let mut intersections: Vec<Point> = self.0.intersection(&other.0).cloned().collect();
        intersections.sort_by_key(Point::distance);
        intersections.get(1).cloned()
    }
}

impl FromIterator<Movement> for Wire {
    fn from_iter<T: IntoIterator<Item = Movement>>(iter: T) -> Self {
        let mut wire = Wire::new();
        let mut point = Point::default();

        for movement in iter {
            let points = point.draw(&movement);
            wire.0.extend(points.iter().cloned());
            point = points.last().cloned().unwrap();
        }

        wire
    }
}

impl FromStr for Wire {
    type Err = ParseMovementError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        s.split(',')
            .map(|movement| movement.parse::<Movement>())
            .collect()
    }
}

fn main() {
    let wires = io::stdin()
        .lock()
        .lines()
        .map(|line| line.expect("Unable to read line").parse::<Wire>())
        .collect::<Result<Vec<_>, _>>()
        .expect("Unable to read wires");

    println!(
        "Distance to closest intersection: {}",
        wires[0]
            .closest_intersection(&wires[1])
            .expect("No intersection")
            .distance()
    );
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_1() {
        assert_eq!(
            Wire::from_str("R8,U5,L5,D3")
                .unwrap()
                .closest_intersection(&Wire::from_str("U7,R6,D4,L4").unwrap()),
            Some(Point { x: 3, y: 3 })
        );
    }

    #[test]
    fn example_2() {
        let wires = "R75,D30,R83,U83,L12,D49,R71,U7,L72
U62,R66,U55,R34,D71,R55,D58,R83"
            .lines()
            .map(Wire::from_str)
            .collect::<Result<Vec<_>, _>>()
            .unwrap();
        assert_eq!(
            wires[0].closest_intersection(&wires[1]).unwrap().distance(),
            159
        )
    }

    #[test]
    fn example_3() {
        let wires = "R98,U47,R26,D63,R33,U87,L62,D20,R33,U53,R51
U98,R91,D20,R16,D67,R40,U7,R15,U6,R7"
            .lines()
            .map(Wire::from_str)
            .collect::<Result<Vec<_>, _>>()
            .unwrap();
        assert_eq!(
            wires[0].closest_intersection(&wires[1]).unwrap().distance(),
            135
        )
    }
}
