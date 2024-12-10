use std::collections::{HashMap, HashSet};

fn main() {
  let input: Vec<u32> = std::fs::read_to_string("./input.txt")
    .expect("input data")
    .to_string()
    .chars()
    .filter(|x| char::is_digit(*x, 10))
    .map(|x| x.to_digit(10).expect("parse char into u32") as u32)
    .collect();

  let mut block_sizes: HashMap<usize, u32> = HashMap::new();
  let mut blocks: Vec<Block> = Vec::new();
  for i in (0..input.len()).step_by(2) {
    let block_id: usize = i / 2;
    let block_size = input.get(i).expect("file size at id").clone();
    for _ in 0..block_size {
      blocks.push(Block {
        id: block_id,
        is_empty: false,
      })
    }
    let free_space = match input.get(i + 1) {
      Some(free_space) => free_space.clone(),
      None => 0,
    };
    for _ in 0..free_space {
      blocks.push(Block {
        id: block_id,
        is_empty: true,
      })
    }
    block_sizes.insert(block_id, block_size);
  }

  let mut blocks_clone = blocks.to_vec();
  println!("part 1 = {}", p1(&mut blocks_clone));
  println!("part 2 = {}", p2(&mut blocks, &block_sizes));
}

#[derive(Clone)]
struct Block {
  id: usize,
  is_empty: bool,
}

fn p1(blocks: &mut Vec<Block>) -> usize {
  move_blocks(blocks);
  // println!("{}", format_blocks(&blocks));
  return compute_checksum(&blocks);
}

fn p2(blocks: &mut Vec<Block>, block_sizes: &HashMap<usize, u32>) -> usize {
  let mut attempts: HashSet<usize> = HashSet::new();
  move_files(blocks, block_sizes, &mut attempts);

  return compute_checksum(&blocks);
}

fn move_files(
  blocks: &mut Vec<Block>,
  block_sizes: &HashMap<usize, u32>,
  swapped: &mut HashSet<usize>,
) {
  let mut j: i32 = blocks.len() as i32 - 1;

  while j > 0 {
    let end_block = blocks.get(j as usize).expect("non-empty block at j");
    let end_block_id = end_block.id;
    let end_block_size = block_sizes
      .get(&end_block_id)
      .expect("block sizes at block id")
      .clone();

    if swapped.contains(&end_block_id) {
      j -= end_block_size as i32;
      continue;
    }

    if end_block.is_empty {
      j -= 1;
      continue;
    }

    let mut i: usize = 0;
    while i < j as usize {
      let curr = blocks.get(i).expect("current block at i");
      if !curr.is_empty {
        i += 1;
        continue;
      }

      let mut free_space: u32 = 0;
      for k in i..blocks.len() {
        if blocks.get(k).expect("block at k").is_empty {
          free_space += 1;
        } else {
          break;
        }
      }

      if end_block_size <= free_space {
        swapped.insert(end_block_id);
        for k in 0..end_block_size {
          blocks.swap(i + k as usize, j as usize - k as usize);
        }
        break;
      }

      i += 1;
    }

    j -= end_block_size as i32;
  }
}

fn move_blocks(blocks: &mut Vec<Block>) {
  let mut j = blocks.len() - 1;
  let mut i = 0;
  while j > i {
    let curr = blocks.get(i).expect("current block at i");
    let block = blocks.get(j).expect("non-empty block at j");

    if !curr.is_empty {
      i += 1;
      continue;
    }

    // get non-empty block from end of blocks
    if !block.is_empty {
      blocks.swap(i, j);
      i += 1;
    }

    j -= 1;
  }
}

fn compute_checksum(blocks: &Vec<Block>) -> usize {
  let mut checksum: usize = 0;
  for pos in 0..blocks.len() {
    let block = blocks.get(pos).expect("block at pos");
    if !block.is_empty {
      checksum += pos * block.id;
    }
  }

  return checksum;
}

#[allow(dead_code)]
fn format_blocks(blocks: &Vec<Block>) -> String {
  let mut formatted: Vec<String> = Vec::new();
  for block in blocks {
    formatted.push(match block.is_empty {
      true => ".".to_string(),
      false => block.id.to_string(),
    })
  }
  return formatted.join("");
}
