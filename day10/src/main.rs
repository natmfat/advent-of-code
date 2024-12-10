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

  println!("part 1 {}", grid.get_paths());
}

struct Grid {
  values: Vec<Vec<u32>>,
  width: usize,
  height: usize,
}

impl Grid {
  fn new(values: Vec<Vec<u32>>) -> Grid {
    let height = values.len();
    let width: usize = values.get(0).expect("first row").len();

    return Grid {
      values,
      width,
      height,
    };
  }

  fn get(&self, point: (usize, usize)) -> Option<u32> {
    let row = self.values.get(point.1);
    if let Some(row) = row {
      return row.get(point.0).cloned();
    }
    return None;
  }

  fn get_trailheads(&self) -> Vec<(usize, usize)> {
    // get all of the trailheads (anywhere that starts w/ 0)
    let mut trailheads: Vec<(usize, usize)> = Vec::new();
    for y in 0..self.height {
      for x in 0..self.width {
        let cell = self.get((x, y)).expect("cell at x,y");
        if cell == 0 {
          trailheads.push((x, y));
        }
      }
    }
    return trailheads;
  }

  fn get_paths(&self) -> i32 {
    let trailheads = self.get_trailheads();
    let mut score = 0;
    for trailhead in trailheads {
      let mut stack: Vec<(usize, usize)> = vec![trailhead];
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
