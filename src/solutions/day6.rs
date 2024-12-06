pub fn part1() {
  // convert input into a 2d grid of cells
  let mut grid = Grid::new(
    std::fs::read_to_string("./inputs/day6-part1.txt")
      .expect("please provide day 6 input")
      .lines()
      .map(|line| line.chars().collect())
      .collect(),
  );

  // locate guard
  let mut pos = Vector { x: 0, y: 0 };
  for y in 0..grid.height() {
    for x in 0..grid.width() {
      if grid.get(x, y).expect("expected grid value") == '^' {
        pos.x = x as i32;
        pos.y = y as i32;
      }
    }
  }

  // run guard routine, marking visited squares with an X
  let mut vel = Vector { x: 0, y: -1 };
  loop {
    // visit current location
    grid.visit(&pos);

    // peak next square, if obstacle, change vel
    match grid.get_vec(pos.clone().add(&vel)) {
      Some('#') => vel = change_dir(&vel),
      None => break,
      _ => (),
    }

    // otherwise move
    pos.add(&vel);
  }

  // println!("{}", grid.to_string());
  println!("{}", grid.visited);

  // when guard leaves, we stop
}

fn change_dir(vel: &Vector) -> Vector {
  let cycle = vec![
    Vector::new(0, -1),
    Vector::new(1, 0),
    Vector::new(0, 1),
    Vector::new(-1, 0),
  ];
  for i in 0..cycle.len() {
    if cycle.get(i).expect("expected to get a vector").equals(vel) {
      return cycle
        .get((i + 1) % cycle.len())
        .expect("where is the next vector???")
        .clone();
    }
  }

  panic!("vel vector must be invalid");
}

#[derive(Debug)]
struct Grid {
  visited: i32,
  values: Vec<Vec<char>>,
}

impl Grid {
  fn new(values: Vec<Vec<char>>) -> Grid {
    return Grid { visited: 0, values };
  }

  fn within_bounds(&self, x: i32, y: i32) -> bool {
    return x >= 0 && x < self.width() && y >= 0 && y < self.height();
  }

  fn get_vec(&self, vector: &Vector) -> Option<char> {
    return self.get(vector.x, vector.y);
  }

  fn set(&mut self, x: i32, y: i32, value: char) {
    if self.within_bounds(x, y) {
      self.values[y as usize][x as usize] = value;
    }
  }

  fn visit(&mut self, pos: &Vector) {
    let prev = self.get_vec(pos);
    self.set(pos.x, pos.y, 'X');
    if prev != Some('X') {
      self.visited += 1;
    }
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

#[derive(Debug)]
struct Vector {
  x: i32,
  y: i32,
}

impl Vector {
  pub fn new(x: i32, y: i32) -> Vector {
    return Vector { x, y };
  }

  pub fn clone(&self) -> Vector {
    return Vector::new(self.x, self.y);
  }

  pub fn add(&mut self, vector: &Vector) -> &mut Vector {
    self.x += vector.x;
    self.y += vector.y;
    return self;
  }

  pub fn equals(&self, vector: &Vector) -> bool {
    return self.x == vector.x && self.y == vector.y;
  }
}
