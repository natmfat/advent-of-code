use std::collections::HashMap;

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

  // println!("part 2 = {}", p2(&mut blocks, &block_sizes));
  println!("part 1 = {}", p1(&mut blocks));
}

struct Block {
  id: usize,
  is_empty: bool,
}

fn p1(blocks: &mut Vec<Block>) -> usize {
  move_blocks(blocks);
  println!("{}", format_blocks(&blocks));
  return compute_checksum(&blocks);
}

fn p2(blocks: &mut Vec<Block>, block_sizes: &HashMap<usize, u32>) -> usize {
  println!("{}", format_blocks(&blocks));
  move_files(blocks, block_sizes);
  println!("{}", format_blocks(&blocks));
  // move_blocks(blocks);
  return compute_checksum(&blocks);
}

fn move_files(blocks: &mut Vec<Block>, block_sizes: &HashMap<usize, u32>) {
  let mut attempted: HashMap<usize, bool> = HashMap::new();
  for i in 0..blocks.len() {
    let curr = blocks.get(i).expect("current block at i");
    if curr.is_empty {
      // get available free space
      let mut free_space: u32 = 0;
      for j in i..blocks.len() {
        let block = blocks.get(j).expect("block at j");
        if block.is_empty {
          free_space += 1;
        } else {
          break;
        }
      }

      // get non-empty block from end of blocks
      for j in ((i + 1)..blocks.len()).rev() {
        let block = blocks.get(j).expect("non-empty block at j");

        // can we swap entire block?
        if !block.is_empty && attempted.get(&block.id).is_none() {
          attempted.insert(block.id, true);
          let block_size = block_sizes
            .get(&block.id)
            .expect("block sizes at block id")
            .clone();
          if block_size <= free_space {
            for k in 0..block_size {
              blocks.swap(i + k as usize, j - k as usize);
            }
            break;
          }
        }
      }
    }
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
    if curr.is_empty && !block.is_empty {
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
