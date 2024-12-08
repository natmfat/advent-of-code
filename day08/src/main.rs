use std::collections::{HashMap, HashSet};

fn main() {
  let input = std::fs::read_to_string("./input.txt").expect("expected input data");
  let mut grid = Grid::new(input.lines().map(|line| line.chars().collect()).collect());
  println!("part 1 = {}", grid.core());
}

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
struct Point(i32, i32);

impl Point {
  fn new(x: i32, y: i32) -> Point {
    return Point(x, y);
  }

  fn add(point_a: &Point, point_b: &Point) -> Point {
    return Point::new(point_a.0 + point_b.0, point_a.1 + point_b.1);
  }

  fn sub(point_a: &Point, point_b: &Point) -> Point {
    return Point::new(point_a.0 - point_b.0, point_a.1 - point_b.1);
  }
}

#[derive(Debug)]
struct Grid {
  // frequency to a list of where they are located
  frequencies: HashMap<char, Vec<Point>>,
  values: Vec<Vec<char>>,
}

impl Grid {
  fn new(values: Vec<Vec<char>>) -> Grid {
    return Grid {
      frequencies: HashMap::new(),
      values,
    };
  }

  fn to_string(&self) -> String {
    return self
      .values
      .iter()
      .map(|row| {
        row
          .iter()
          .map(|c| c.to_string())
          .collect::<Vec<String>>()
          .join("")
      })
      .collect::<Vec<String>>()
      .join("\n");
  }

  fn core(&mut self) -> i32 {
    // get all antennas
    for y in 0..self.height() {
      for x in 0..self.width() {
        let frequency = self
          .get(x, y)
          .expect("should have gotten a grid value at x & y");
        if frequency != '.' {
          // we have a antenna with a "frequency" being represented by a char
          self
            .frequencies
            .entry(frequency)
            .and_modify(|locations| locations.push(Point::new(x, y)))
            .or_insert(vec![Point::new(x, y)]);
        }
      }
    }

    let mut used_positions: HashSet<Point> = HashSet::new();

    for (_, positions) in self.frequencies.iter() {
      for i in 0..(positions.len() - 1) {
        let pos_a = positions.get(i).expect("expected position at i");
        for j in (i + 1)..positions.len() {
          let pos_b = positions.get(j).expect("expected position at j");

          // compute slope-ish (how line is changing, but we really want this as a point/vec)
          let vel = Point::sub(pos_b, pos_a);
          let possible_antinodes = vec![Point::sub(pos_a, &vel), Point::add(pos_b, &vel)];

          for antinode in possible_antinodes {
            if self.point_within_bounds(&antinode) {
              used_positions.insert(antinode.clone());
              self.values[antinode.1 as usize][antinode.0 as usize] = '#';
            }
          }
        }
      }
    }

    return used_positions.len() as i32;
  }

  fn within_bounds(&self, x: i32, y: i32) -> bool {
    return x >= 0 && x < self.width() && y >= 0 && y < self.height();
  }

  fn point_within_bounds(&self, point: &Point) -> bool {
    return self.within_bounds(point.0, point.1);
  }

  fn get(&self, x: i32, y: i32) -> Option<char> {
    if self.within_bounds(x, y) {
      return self
        .values
        .get::<usize>(y as usize)
        .and_then(|row| row.get(x as usize).cloned());
    }

    return None;
  }

  fn width(&self) -> i32 {
    return self
      .values
      .get(0)
      .expect("expected a row")
      .len()
      .try_into()
      .expect("parse usize into i32");
  }

  fn height(&self) -> i32 {
    return self.values.len().try_into().expect("parse usize into i32");
  }
}
