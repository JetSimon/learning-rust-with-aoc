use std::{collections::HashSet, fs, vec};

#[derive(Debug, Clone, Copy)]
struct Block {
    start: usize,
    end: usize,
    id: i64,
    size: usize,
}

impl std::hash::Hash for Block {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.id.hash(state);
        self.size.hash(state);
    }
}

fn print_memory(memory: &Vec<i64>) {
    for block in memory {
        print!(
            "{}",
            if *block == -1 {
                ".".to_string()
            } else {
                block.to_string()
            }
        )
    }

    print!("\n");
}

fn get_used_blocks_from_right(memory: &Vec<i64>) -> Vec<Block> {
    let mut blocks = vec![];
    let mut right: i64 = (memory.len() - 1) as i64;
    let mut left: i64 = right - 1;

    while left > 0 && right > 0 {
        while memory[right as usize] == -1 && right > 0 {
            right -= 1;
        }

        if left > right {
            if right > 0 {
                left = right - 1;
            } else {
                left = 0;
            }
        }

        while left >= 0 && memory[left as usize] == memory[right as usize] {
            left -= 1;
        }

        blocks.push(Block {
            start: (left + 1) as usize,
            end: (right as usize),
            id: memory[right as usize],
            size: (right - left) as usize,
        });
        right = left;
    }

    return blocks;
}

fn get_free_blocks_from_left(memory: &Vec<i64>, max_index: usize) -> Vec<Block> {
    let mut blocks = vec![];
    let mut left: usize = 0;
    let mut right: usize = left;

    while left < max_index + 1 && right < max_index + 1 {
        while left < max_index + 1 && memory[left] != -1 {
            left += 1;
        }

        right = left;

        while right < max_index + 1 && memory[right] == -1 {
            right += 1;
        }

        blocks.push(Block {
            start: left,
            end: right - 1,
            id: -1,
            size: right - left,
        });

        left = right;
    }

    return blocks;
}

fn compact_naive(memory: &Vec<i64>) -> Vec<i64> {
    let mut compacted_memory = memory.clone();

    let mut left: usize = 0;
    let mut right = compacted_memory.len() - 1;

    while left < right {
        while compacted_memory[left] != -1 {
            left += 1;
        }
        while compacted_memory[right] == -1 {
            right -= 1;
        }

        if left > right {
            break;
        }

        compacted_memory[left] = compacted_memory[right];
        compacted_memory[right] = -1;
    }

    return compacted_memory;
}

fn swap_blocks(full_block: &Block, free_block: &Block, memory: &mut Vec<i64>) {
    for i in free_block.start..(free_block.start + full_block.size) {
        memory[i] = full_block.id;
    }

    for i in full_block.start..(full_block.start + full_block.size) {
        memory[i] = free_block.id;
    }
}

fn get_unmoved_used_blocks(memory: &Vec<i64>, moved_blocks: &HashSet<i64>) -> Vec<Block> {
    let mut blocks = vec![];

    for block in get_used_blocks_from_right(&memory) {
        if !moved_blocks.contains(&block.id) {
            blocks.push(block);
        }
    }

    return blocks;
}

fn compact_defrag(memory: &Vec<i64>) -> Vec<i64> {
    let mut compacted_memory = memory.clone();

    let mut moved_blocks = HashSet::new();

    let blocks = get_unmoved_used_blocks(&compacted_memory, &moved_blocks);

    for block in blocks {
        let free = get_free_blocks_from_left(&compacted_memory, block.end);
        for free_block in free {
            if free_block.size >= block.size {
                swap_blocks(&block, &free_block, &mut compacted_memory);
                moved_blocks.insert(block.id);
                break;
            }
        }
    }

    return compacted_memory;
}

fn checksum(memory: &Vec<i64>) -> i64 {
    let mut total = 0;

    for i in 0..memory.len() {
        let id = memory[i];
        if id != -1 {
            total += id * i as i64;
        }
    }

    return total;
}

pub fn run(path: String) {
    // --snip--
    println!("In file {path}");

    let contents = fs::read_to_string(path);

    let res = if let Ok(res) = contents {
        res
    } else {
        let error = contents.err();
        panic!("Problem opening the file: {error:?}");
    };

    let mut memory = vec![];
    let mut is_file = true;
    let mut id = 0;

    for n in res
        .split("")
        .filter(|c| *c != "")
        .map(|c| c.parse::<usize>().unwrap())
    {
        for _ in 0..n {
            memory.push(if is_file { id } else { -1 });
        }

        if is_file {
            id += 1;
        }
        is_file = !is_file;
    }

    let compacted_part1 = compact_naive(&memory);
    println!("Day 9 Part 1: {}", checksum(&compacted_part1));

    let compacted_part2 = compact_defrag(&memory);
    println!("Day 9 Part 2: {}", checksum(&compacted_part2));
}
