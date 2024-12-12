use std::collections::HashMap;

fn main() {
  let mut numbers: HashMap<u64, u64> = std::fs::read_to_string("./input.txt")
    .expect("input data")
    .split(" ")
    .map(|number| number.parse::<u64>().expect("parse string into u64"))
    .fold(HashMap::new(), |mut acc: HashMap<u64, u64>, number| {
      acc.entry(number).and_modify(|n| *n += 1).or_insert(1);
      acc
    });

  for _ in 0..25 {
    numbers = blink(&mut numbers);
  }
  println!("part 1 = {}", get_num_stones(&numbers));

  for _ in 0..50 {
    numbers = blink(&mut numbers);
  }
  println!("part 2 = {}", get_num_stones(&numbers));
}

fn get_num_stones(numbers: &HashMap<u64, u64>) -> u64 {
  let mut num_stones = 0;
  for frequency in numbers.values() {
    num_stones += frequency;
  }
  return num_stones;
}

fn blink(numbers: &mut HashMap<u64, u64>) -> HashMap<u64, u64> {
  let mut next_numbers: HashMap<u64, u64> = HashMap::new();
  for (number, frequency) in numbers.into_iter() {
    let number = number.clone();
    let frequency = frequency.clone();
    if number == 0 {
      next_numbers
        .entry(1)
        .and_modify(|n| *n += frequency)
        .or_insert(frequency);
      continue;
    }

    let num_digits: i64 = f64::log10(number as f64) as i64 + 1;
    if num_digits % 2 == 0 {
      let power = 10_f64.powi((num_digits / 2_i64) as i32) as u64;
      let left_num = number / power;
      let right_num = number - left_num * power;
      next_numbers
        .entry(left_num)
        .and_modify(|n| *n += frequency)
        .or_insert(frequency);
      next_numbers
        .entry(right_num)
        .and_modify(|n| *n += frequency)
        .or_insert(frequency);
    } else {
      next_numbers
        .entry(number * 2024)
        .and_modify(|n| *n += frequency)
        .or_insert(frequency);
    }
  }
  return next_numbers;
}
