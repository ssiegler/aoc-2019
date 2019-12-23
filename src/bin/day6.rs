use std::collections::{BinaryHeap, HashMap};
use std::convert::TryFrom;
use std::io::{stdin, BufRead};
use std::iter::FromIterator;

fn main() {
    let orbits = read_orbits();
    println!("Checksum: {}", orbits.checksum());
    let source = orbits.center("YOU").expect("YOU not on map");
    let destination = orbits.center("SAN").expect("SAN not on map");
    println!(
        "Required transfers: {}",
        orbits.distance(source, destination).expect("No path")
    );
}

fn read_orbits() -> OrbitMap {
    stdin()
        .lock()
        .lines()
        .map(|line| line.expect("Error reading line"))
        .map(Orbit::try_from)
        .collect::<Result<OrbitMap, _>>()
        .expect("Error reading map")
}

#[derive(Debug)]
enum Error {
    InvalidOrbit { orbit: String },
}

#[derive(Debug)]
struct Orbit {
    center: String,
    satellite: String,
}

impl TryFrom<String> for Orbit {
    type Error = Error;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        let parts: Vec<&str> = value.splitn(2, ')').collect();
        if parts.len() == 2 {
            Ok(Orbit {
                center: parts[0].to_string(),
                satellite: parts[1].to_string(),
            })
        } else {
            Err(Error::InvalidOrbit {
                orbit: value.to_string(),
            })
        }
    }
}

#[derive(Default)]
struct OrbitMap {
    outside: HashMap<String, Vec<String>>,
    inside: HashMap<String, String>,
}

impl FromIterator<Orbit> for OrbitMap {
    fn from_iter<T: IntoIterator<Item = Orbit>>(iter: T) -> Self {
        let mut orbits = OrbitMap::default();
        for orbit in iter {
            orbits.add(&orbit);
        }
        orbits
    }
}

impl<'a> OrbitMap {
    const EMPTY: &'a [String] = &[];

    fn center(&self, object: &str) -> Option<&String> {
        self.inside.get(object)
    }

    fn around(&self, object: &str) -> &[String] {
        self.outside
            .get(object)
            .map(Vec::as_slice)
            .unwrap_or(Self::EMPTY)
    }

    fn add(&mut self, orbit: &Orbit) {
        self.outside
            .entry(orbit.center.clone())
            .or_default()
            .push(orbit.satellite.clone());
        self.inside
            .insert(orbit.satellite.clone(), orbit.center.clone());
    }

    fn neighbours(&self, object: &str) -> impl Iterator<Item = &String> {
        self.center(object)
            .into_iter()
            .chain(self.around(object).iter())
    }

    fn distance(&self, source: &str, destination: &str) -> Option<usize> {
        let mut distances: HashMap<&str, usize> = HashMap::new();
        let mut queue = BinaryHeap::new();
        queue.push((0, source));
        while let Some((distance, object)) = queue.pop() {
            if destination == object {
                return Some(distance);
            }
            if distance > *distances.get(object).unwrap_or(&std::usize::MAX) {
                continue;
            }

            let distance = distance + 1;
            for object in self.neighbours(object) {
                if distance < *distances.get(object.as_str()).unwrap_or(&std::usize::MAX) {
                    queue.push((distance, object));
                    distances.insert(object, distance);
                }
            }
        }
        None
    }

    fn checksum(&self) -> usize {
        let mut queue: Vec<(usize, &str)> = vec![(0, "COM")];
        let mut checksum = 0;
        while let Some((level, object)) = queue.pop() {
            let satellites = self.around(&object);
            let level = level + 1;
            checksum += level * satellites.len();
            for satellite in satellites {
                queue.push((level, satellite.as_str()))
            }
        }
        checksum
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn read_lines(lines: &str) -> OrbitMap {
        lines
            .lines()
            .map(|line| line.to_string())
            .map(Orbit::try_from)
            .collect::<Result<OrbitMap, _>>()
            .unwrap()
    }

    #[test]
    fn calculates_checksum() {
        let orbits = read_lines(
            "COM)B
B)C
C)D
D)E
E)F
B)G
G)H
D)I
E)J
J)K
K)L
",
        );
        assert_eq!(orbits.checksum(), 42);
    }

    #[test]
    fn calculates_distance() {
        let orbits = read_lines(
            "COM)B
B)C
C)D
D)E
E)F
B)G
G)H
D)I
E)J
J)K
K)L
K)YOU
I)SAN",
        );
        let source = orbits.center("YOU").unwrap();
        let destination = orbits.center("SAN").unwrap();
        assert_eq!(orbits.distance(source, destination), Some(4));
    }
}
