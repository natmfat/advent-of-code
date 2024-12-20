mod heap;
use heap::Heap;

use std::collections::HashMap;
use std::fs::read_to_string;

fn main() {
  println!("part 1 = {}", part1());
  println!("part 2 = {}", part2());
}

pub fn part1() -> i32 {
  let mut heap_a = Heap::new();
  let mut heap_b = Heap::new();

  // get all of the lines and add them to the corresponding list
  for line in read_to_string("./input.txt")
    .expect("expected file")
    .lines()
  {
    let components: Vec<&str> = line.split_whitespace().collect();
    heap_a.push(components.get(0).unwrap().parse::<i32>().unwrap());
    heap_b.push(components.get(1).unwrap().parse::<i32>().unwrap());
  }

  if heap_a.len() != heap_b.len() {
    panic!("invalid list or something");
  }

  let mut result = 0;
  for _ in 0..heap_a.len() {
    let dist = (heap_b.pop().unwrap() - heap_a.pop().unwrap()).abs();
    result += dist;
  }

  return result;
}

pub fn part2() -> i32 {
  let mut list_a: Vec<i32> = Vec::new();
  let mut freq: HashMap<i32, i32> = HashMap::new();

  for line in read_to_string("./input.txt")
    .expect("expected file")
    .lines()
  {
    let components: Vec<&str> = line.split_whitespace().collect();
    list_a.push(components.get(0).unwrap().parse::<i32>().unwrap());
    freq
      .entry(components.get(1).unwrap().parse::<i32>().unwrap())
      .and_modify(|x| *x += 1)
      .or_insert(1);
  }

  let mut result = 0;
  for item in list_a {
    result += item * *freq.get(&item).unwrap_or(&0);
  }

  return result;
}
