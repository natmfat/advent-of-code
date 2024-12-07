use std::rc::Rc;

fn main() {
  // more efficient would just be to convert x & y into a single index
  // something like y * width + x probably
  let input: Vec<Vec<char>> = std::fs::read_to_string("./input.txt")
    .expect("expected input data")
    .lines()
    .map(|line| line.chars().collect())
    .collect();
  let mut grid = Grid::new(input);
  println!("part 1 = {}", grid.find_xmas());
  println!("part 2 = {}", grid.find_x());
}

/// list of characters to find
const XMAS: [char; 4] = ['X', 'M', 'A', 'S'];

struct Grid {
  values: Vec<Vec<char>>,
}

impl Grid {
  fn new(values: Vec<Vec<char>>) -> Grid {
    return Grid { values };
  }

  fn width(&self) -> i32 {
    return self
      .values
      .get(0)
      .expect("expected row")
      .len()
      .try_into()
      .expect("expected to parse width into i32");
  }

  fn height(&self) -> i32 {
    return self
      .values
      .len()
      .try_into()
      .expect("expected to parse height into i32");
  }

  fn get(&self, x: i32, y: i32) -> Option<char> {
    if !(x >= 0 && x < self.width() && y >= 0 && y < self.height()) {
      return None;
    }

    let row = self.values.get(y as usize);
    if row != None {
      let value = row?.get(x as usize);
      return value.copied();
    }
    return None;
  }

  /// PART 2:
  /// Find MAS in the shape of an X

  fn find_x(&mut self) -> i32 {
    let pattern: Rc<Vec<char>> = Rc::new(vec!['M', 'A', 'S']);
    let mut count = 0;
    for y in 0..self.height() {
      for x in 0..self.width() {
        if self.get(x, y) == Some('A') {
          if (Grid::check(1, 1, Rc::clone(&pattern))(self, x - 1, y - 1) // top left -> bottom right
          || Grid::check(-1, -1, Rc::clone(&pattern))(self, x + 1, y + 1)) // bottom right -> top left
          && (Grid::check(-1, 1, Rc::clone(&pattern))(self, x + 1, y - 1) // top right -> bottom left
          || Grid::check(1, -1, Rc::clone(&pattern))(self, x - 1, y + 1))
          // bottom left -> top right
          {
            count += 1;
          }
        }
      }
    }
    return count;
  }

  /// PART 1:
  /// Find all XMAS instances like a crossword puzzle

  /// xmas factory - search for XMAS in the provided direction at a current location
  fn check(dx: i32, dy: i32, pattern: Rc<Vec<char>>) -> Box<dyn Fn(&Grid, i32, i32) -> bool> {
    let check_direction = move |grid: &Grid, x: i32, y: i32| {
      for i in 0..pattern.len() {
        if grid.get(x + (i as i32) * dx, y + (i as i32) * dy) != Some(pattern[i]) {
          return false;
        }
      }
      return true;
    };
    return Box::new(check_direction);
  }

  /// find XMAS like a cross word puzzle
  fn find_xmas(&mut self) -> i32 {
    // array of methods to check for each grid value
    let pattern: Rc<Vec<char>> = Rc::new(vec!['X', 'M', 'A', 'S']);
    let checks = vec![
      Grid::check(1, 0, Rc::clone(&pattern)), // check right horizontal
      Grid::check(-1, 0, Rc::clone(&pattern)), // check left horizontal
      Grid::check(0, 1, Rc::clone(&pattern)), // check bottom vertical
      Grid::check(0, -1, Rc::clone(&pattern)), // check top vertical
      Grid::check(1, -1, Rc::clone(&pattern)), // check top right diagonal
      Grid::check(1, 1, Rc::clone(&pattern)), // check bottom right diagonal
      Grid::check(-1, -1, Rc::clone(&pattern)), // check top left diagonal
      Grid::check(-1, 1, Rc::clone(&pattern)), // check bottom left diagonal
    ];

    let mut count: i32 = 0;
    for y in 0..self.height() {
      for x in 0..self.width() {
        if self.get(x, y) == Some('X') {
          count += checks
            .iter()
            .map(|procedure| match procedure(&self, x, y) {
              true => 1,
              false => 0,
            })
            .sum::<i32>();
        }
      }
    }
    return count;
  }
}
