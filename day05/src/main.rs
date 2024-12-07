use std::collections::HashMap;

fn main() {
  let input = std::fs::read_to_string("./input.txt").expect("expected input data");
  let mut ordering: HashMap<i32, Vec<i32>> = HashMap::new();
  let mut valid_updates = 0;
  let mut invalid_updates = 0;
  let mut parsing_order = true;
  for line in input.lines() {
    // toggle mode (either parsing orders or parsing instructions)
    if line.is_empty() {
      parsing_order = false;
      continue;
    }

    // currently parsing page orders
    // we store this in a hashmap of pages that need to come before -> valid pages that can come after
    if parsing_order {
      let components: Vec<i32> = line
        .split('|')
        .map(|x| {
          x.parse::<i32>()
            .expect("should be able to parse each item in ordering section")
        })
        .collect();

      let item_before = *components.get(0).expect("expected first item");
      let item_after = *components.get(1).expect("expected second item");
      ordering
        .entry(item_before)
        .and_modify(|orders| orders.push(item_after))
        .or_insert(vec![item_after]);
    } else {
      // currently parsing update line to see if they are valid
      let mut update: Vec<i32> = line
        .split(',')
        .map(|x| {
          x.parse::<i32>()
            .expect("should be able to parse each item in instruction section")
        })
        .collect();
      if is_valid(&ordering, &update) {
        valid_updates += update.get(update.len() / 2).expect("expected middle value");
      } else {
        fix_update(&ordering, &mut update);
        invalid_updates += update.get(update.len() / 2).expect("expected middle value");
      }
    }
  }

  println!("part 1 = {}", valid_updates);
  println!("part 2 = {}", invalid_updates);
}

fn fix_update(ordering: &HashMap<i32, Vec<i32>>, update: &mut Vec<i32>) {
  let mut swaps = false;

  for i in 0..(update.len() - 1) {
    let current = update.get(i).expect("expected to get current");
    let valid_after = ordering.get(current); // what pages can come after?
    if let Some(valid_after) = valid_after {
      // loop through remaining elements
      for j in (i + 1)..update.len() {
        let comparison = update
          .get(j)
          .expect("expected to get remaining el to compare to");
        // the comparison we are examining is not a page that can come after
        // so this instruction must be invalid
        if valid_after
          .iter()
          .position(|&item| item == *comparison)
          .is_none()
        {
          // swap the current and comparison cause it's wrong
          update.swap(i, j);
          swaps = true;
        }
      }
    } else {
      // nothing comes after, so we we stick it at the end of the list
      let current = update.remove(i);
      update.push(current);
    }
  }

  // swaps were made, so keep fixing list
  if swaps {
    fix_update(ordering, update);
  }
}

fn is_valid(ordering: &HashMap<i32, Vec<i32>>, update: &Vec<i32>) -> bool {
  for i in 0..(update.len() - 1) {
    let current = update.get(i).expect("expected to get current");
    let valid_after = ordering.get(current); // what pages can come after?
    if let Some(valid_after) = valid_after {
      // loop through remaining elements
      for j in (i + 1)..update.len() {
        let comparison = update
          .get(j)
          .expect("expected to get remaining el to compare to");
        // the comparison we are examining is not a page that can come after
        // so this instruction must be invalid
        if valid_after
          .iter()
          .position(|&item| item == *comparison)
          .is_none()
        {
          return false;
        }
      }
    } else {
      // nothing is valid after, so we know current should actually be at the END of the update list
      return false;
    }
  }
  return true;
}
