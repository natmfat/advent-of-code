use std::sync::{Arc, Mutex};

use rayon::prelude::*;
fn main() {
  let mut numbers: Vec<u64> = std::fs::read_to_string("./input.txt")
    .expect("input data")
    .split(" ")
    .map(|number| number.parse::<u64>().expect("parse string into u64"))
    .collect();

  for _ in 0..25 {
    numbers = blink(&numbers);
  }
  println!("part 1 = {}", numbers.len());

  for i in 0..50 {
    numbers = blink(&numbers);
    println!("finished {i}")
  }
  println!("part 2 = {}", numbers.len());
}

fn blink(numbers: &Vec<u64>) -> Vec<u64> {
  let blink_split: Arc<Mutex<Vec<u64>>> = Arc::new(Mutex::new(Vec::new()));
  let mut blink_numbers: Vec<u64> = numbers
    .par_iter()
    .map(|curr| {
      let curr = curr.clone();
      if curr == 0 {
        return 1;
      } else {
        let num_digits: i64 = f64::log10(curr as f64) as i64 + 1;
        if num_digits % 2 == 0 {
          let power = 10_f64.powi((num_digits / 2_i64) as i32) as u64;
          let left_num = curr / power;
          let right_num = curr - left_num * power;
          blink_split.lock().unwrap().push(right_num);
          return left_num;
        } else {
          return curr * 2024;
        }
      }
    })
    .collect();

  blink_numbers.extend(blink_split.lock().unwrap().to_vec());
  return blink_numbers;
}
