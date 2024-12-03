use aoc::Heap;
use std::fs::read_to_string;

fn main() {
    let mut heap_a = Heap::new();
    let mut heap_b = Heap::new();

    // get all of the lines and add them to the corresponding list
    for line in read_to_string("./inputs/day1-part1.txt")
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

    let mut total_dist = 0;
    for _ in 0..heap_a.len() {
        // dbg!(list_b[0]);
        let dist = (heap_b.pop().unwrap() - heap_a.pop().unwrap()).abs();
        total_dist += dist;
    }
    println!("{total_dist}");
}

// #[cfg(test)]
// mod tests {
//     use super::*;

//     #[test]
//     fn heap_add() {}

//     #[test]
//     fn heap_pop() {}

//     #[test]
//     fn heap_sort() {}
// }
