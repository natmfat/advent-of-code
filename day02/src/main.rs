fn main() {
  println!("part 1 = {}", part(0));
  println!("part 2 = {}", part(1));
}

pub fn part(tolerance: u32) -> i32{
  let mut safe = 0;
  for line in std::fs::read_to_string("./input.txt")
    .expect("day2 inputs should be provided")
    .lines()
  {
    if is_safe(line.split_whitespace().collect(), tolerance) {
      safe += 1;
    }
  }

  return safe;
}

/// determine if a line is "safe" (monotonic & within a range)
fn is_safe(components: Vec<&str>, tolerance: u32) -> bool {
  let parsed_components = components
    .iter()
    .map(|&x| x.parse::<i32>().expect("parse string into int"))
    .collect::<Vec<i32>>();

  let mut direction_flag = false; // false is decreasing, true is increasing
  let mut unsafe_flags = 0;
  for i in 0..(components.len() - 1) {
    let curr = *parsed_components.get(i).expect("value within range");
    let next = *parsed_components.get(i + 1).expect("value within range");

    // determine direction
    if i == 0 && next > curr {
      direction_flag = true;
    }

    // distance must be within [1, 3]
    let distance: i32 = (next - curr).abs();
    if distance < 1
      || distance > 3
      // supposed to be decreasing, but we're not
      || (!direction_flag && next > curr) 
      // supposed to be increasing, but we're not
      || (direction_flag && next < curr)
    
    {
      unsafe_flags += 1;
    }

    // stop early if too many "unsafe" levels
    if unsafe_flags > tolerance {
      return false;
    }
  }

  // make sure we're within tolerance
  return unsafe_flags <= tolerance;
}
