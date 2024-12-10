use std::collections::HashSet;

fn main() {
  let input = std::fs::read_to_string("./input.txt").expect("input data");
  let grid = Grid::new(
    input
      .lines()
      .map(|line| {
        line
          .chars()
          .map(|pos| char::to_digit(pos, 10).expect("parse char into u32"))
          .collect()
      })
      .collect(),
  );

  println!("part 1 {}", grid.get_trailheads_score());
  println!("part 2 {}", grid.get_trailheads_rating());
}

struct Grid {
  values: Vec<Vec<u32>>,
  trailheads: Vec<(usize, usize)>,
  width: usize,
  height: usize,
}

impl Grid {
  fn new(values: Vec<Vec<u32>>) -> Grid {
    let height = values.len();
    let width: usize = values.get(0).expect("first row").len();
    let mut grid = Grid {
      values,
      width,
      height,
      trailheads: Vec::new(),
    };
    grid.get_trailheads();
    return grid;
  }

  fn get(&self, point: (usize, usize)) -> Option<u32> {
    let row = self.values.get(point.1);
    if let Some(row) = row {
      return row.get(point.0).cloned();
    }
    return None;
  }

  /// get all of the trailheads (anywhere that starts w/ 0)
  fn get_trailheads(&mut self) {
    for y in 0..self.height {
      for x in 0..self.width {
        let cell = self.get((x, y)).expect("cell at x,y");
        if cell == 0 {
          self.trailheads.push((x, y));
        }
      }
    }
  }

  fn get_trailheads_rating(&self) -> i32 {
    let mut rating = 0;
    for trailhead in &self.trailheads {
      let mut stack: Vec<(usize, usize)> = vec![*trailhead];
      while stack.len() > 0 {
        let curr_point = stack.pop().expect("item in stack");
        let curr_value = self.get(curr_point).expect("stack item in grid");

        // did we reach endpoints
        if curr_value == 9 {
          rating += 1;
          continue;
        }

        for i in -1..=1 {
          for j in -1..=1 {
            if (i == 0) ^ (j == 0) {
              // if direct neighbor within bounds
              let x = (curr_point.0 as i32) + i;
              let y = (curr_point.1 as i32) + j;
              if x >= 0 && x < self.width as i32 && y >= 0 && y < self.height as i32 {
                let neighbor_point = (x as usize, y as usize);
                let neighbor_value = self.get(neighbor_point).expect("neighbor within bounds");
                // if neighbor is within 1 unit of distance
                if (neighbor_value as i32) - (curr_value as i32) == 1 {
                  stack.push(neighbor_point);
                }
              }
            }
          }
        }
      }
    }
    return rating;
  }

  fn get_trailheads_score(&self) -> i32 {
    let mut score = 0;
    for trailhead in &self.trailheads {
      let mut stack: Vec<(usize, usize)> = vec![*trailhead];
      let mut visited: HashSet<(usize, usize)> = HashSet::new();
      while stack.len() > 0 {
        let curr_point = stack.pop().expect("item in stack");
        let curr_value = self.get(curr_point).expect("stack item in grid");
        visited.insert(curr_point);

        // did we reach endpoints
        if curr_value == 9 {
          score += 1;
          continue;
        }

        for i in -1..=1 {
          for j in -1..=1 {
            if (i == 0) ^ (j == 0) {
              // if direct neighbor within bounds
              let x = (curr_point.0 as i32) + i;
              let y = (curr_point.1 as i32) + j;
              if x >= 0 && x < self.width as i32 && y >= 0 && y < self.height as i32 {
                let neighbor_point = (x as usize, y as usize);
                let neighbor_value = self.get(neighbor_point).expect("neighbor within bounds");
                // if neighbor is within 1 unit of distance
                if (neighbor_value as i32) - (curr_value as i32) == 1
                  && !visited.contains(&neighbor_point)
                {
                  stack.push(neighbor_point);
                }
              }
            }
          }
        }
      }
    }
    return score;
  }
}
