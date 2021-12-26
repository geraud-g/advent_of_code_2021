use itertools::Itertools;

use advent_of_code_2021::utils::inputs::get_file;

pub fn day_18() {
    let mut arena = ArenaTree::default();
    let numbers = get_input(&mut arena);


    let magnitude = get_magnitude_sum(&arena, &numbers);
    println!("Solution for Day 18, part A is: {}", magnitude);

    let mut arena = ArenaTree::default();
    let numbers = get_input(&mut arena);
    let max_comb_val = get_max_combination_value(&mut arena, &numbers);
    println!("Solution for Day 18, part B is: {}", max_comb_val);
}


// #################################################################################################
// # Structs
// #################################################################################################
#[derive(Debug, Default, Clone)]
pub struct ArenaTree {
    pub arena: Vec<Node>,
}

impl ArenaTree {
    fn add_new_node(&mut self, value: Option<u8>, parent: Option<usize>) -> usize {
        let new_idx = self.arena.len();
        let new_node = Node::new_node(new_idx, value, parent);
        self.arena.push(new_node);
        new_idx
    }

    fn merge_nodes(&mut self, left_node: usize, right_node: usize) -> usize {
        let new_node_idx = self.add_new_node(None, None);
        self.arena[new_node_idx].left = Some(left_node);
        self.arena[new_node_idx].right = Some(right_node);

        self.arena[left_node].parent = Some(new_node_idx);
        self.arena[right_node].parent = Some(new_node_idx);
        new_node_idx
    }

    #[allow(dead_code)]
    fn to_str(&self, root: usize) -> String {
        if let Some(value) = self.arena[root].val {
            value.to_string()
        } else {
            format!("[{},{}]",
                    self.to_str(self.arena[root].left.unwrap()),
                    self.to_str(self.arena[root].right.unwrap())
            )
        }
    }
}


#[derive(Debug, PartialEq, Clone)]
pub struct Node {
    pub idx: usize,
    pub val: Option<u8>,
    pub parent: Option<usize>,
    pub left: Option<usize>,
    pub right: Option<usize>,
}


impl Node {
    fn new_node(idx: usize, value: Option<u8>, parent: Option<usize>) -> Self {
        Self {
            idx,
            val: value,
            parent,
            left: None,
            right: None,
        }
    }
}


// #################################################################################################
// # Parsing
// #################################################################################################
fn get_input(arena: &mut ArenaTree) -> Vec<usize> {
    let mut numbers = vec![];
    for line in get_file("./src/day_18/input.txt").lines() {
        numbers.push(parse_line(arena, &line[1..line.len() - 1]))
    }
    numbers
}


fn parse_line(arena: &mut ArenaTree, line: &str) -> usize {
    let mut separator_idx = None;
    let mut depth = 0;
    for (idx, val) in line.chars().enumerate() {
        depth += match val {
            '[' => 1,
            ']' => -1,
            _ => 0
        };
        if depth == 0 && val == ',' {
            separator_idx = Some(idx);
            break;
        }
    }
    let current_node_idx = arena.add_new_node(None, None);

    if let Some(idx) = separator_idx {
        let new_left_node = match &line[..idx].parse::<u8>() {
            Ok(ok) => arena.add_new_node(Some(*ok), None),
            Err(_) => parse_line(arena, &line[1..idx - 1])
        };
        arena.arena[current_node_idx].left = Some(new_left_node);
        arena.arena[new_left_node].parent = Some(current_node_idx);

        let new_right_node = match &line[(idx + 1)..].parse::<u8>() {
            Ok(ok) => arena.add_new_node(Some(*ok), None),
            Err(_) => parse_line(arena, &line[(idx + 2)..line.len() - 1])
        };
        arena.arena[current_node_idx].right = Some(new_right_node);
        arena.arena[new_right_node].parent = Some(current_node_idx);
        current_node_idx
    } else {
        panic!("Separator not found for {}", line)
    }
}


// #################################################################################################
// # Puzzle
// #################################################################################################
fn get_magnitude_sum(arena: &ArenaTree, numbers: &[usize]) -> usize {
    let mut new_arena = arena.clone();
    let root_idx = sum(&mut new_arena, numbers);
    get_magnitude(&mut new_arena, root_idx)
}


fn get_magnitude(arena: &mut ArenaTree, idx: usize) -> usize {
    if let Some(value) = arena.arena[idx].val {
        value as usize
    } else {
        get_magnitude(arena, arena.arena[idx].left.unwrap()) * 3
            + get_magnitude(arena, arena.arena[idx].right.unwrap()) * 2
    }
}

fn sum(arena: &mut ArenaTree, numbers: &[usize]) -> usize {
    if numbers.len() == 1 {
        return numbers[0];
    }
    let new_root_idx = arena.merge_nodes(numbers[0], numbers[1]);
    process(arena, new_root_idx);
    sum(arena, &[&[new_root_idx], &numbers[2..]].concat())
}

fn process(arena: &mut ArenaTree, idx: usize) {
    if explode_if_required(arena, idx, 0) {
        process(arena, idx)
    }
    if split_if_required(arena, idx) {
        process(arena, idx)
    }
}


fn split_if_required(arena: &mut ArenaTree, current_node: usize) -> bool {
    if let Some(value) = arena.arena[current_node].val {
        if value >= 10 {
            let left_val = value / 2;
            let right_val = left_val + value % 2;
            let left_idx = arena.add_new_node(Some(left_val), Some(current_node));
            let right_idx = arena.add_new_node(Some(right_val), Some(current_node));
            arena.arena[current_node].left = Some(left_idx);
            arena.arena[current_node].right = Some(right_idx);
            arena.arena[current_node].val = None;
            return true;
        }
        return false;
    }
    if let Some(left_idx) = arena.arena[current_node].left {
        if split_if_required(arena, left_idx) {
            return true;
        }
    }
    if let Some(right_idx) = arena.arena[current_node].right {
        if split_if_required(arena, right_idx) {
            return true;
        }
    }
    false
}


fn explode_if_required(arena: &mut ArenaTree, current_node: usize, depth: usize) -> bool {
    if depth == 4 && arena.arena[current_node].val.is_none() {
        let left_idx = arena.arena[current_node].left.unwrap();
        let right_idx = arena.arena[current_node].right.unwrap();
        let left_val = arena.arena[left_idx].val.unwrap();
        let right_val = arena.arena[right_idx].val.unwrap();

        add_left_value(arena, current_node, left_val);
        add_right_value(arena, current_node, right_val);
        arena.arena[current_node].left = None;
        arena.arena[current_node].right = None;
        arena.arena[current_node].val = Some(0);
        return true;
    }
    if let Some(left_idx) = arena.arena[current_node].left {
        if explode_if_required(arena, left_idx, depth + 1) {
            return true;
        }
    }
    if let Some(right_idx) = arena.arena[current_node].right {
        if explode_if_required(arena, right_idx, depth + 1) {
            return true;
        }
    }
    false
}


fn add_left_value(arena: &mut ArenaTree, start_idx: usize, val_to_add: u8) {
    let mut last_idx = start_idx;
    let parent_idx = arena.arena[start_idx].parent.unwrap();
    let mut current_idx = parent_idx;

    while let Some(left_idx) = arena.arena[current_idx].left {
        if left_idx != last_idx {
            break;
        }
        if let Some(parent_idx) = arena.arena[current_idx].parent {
            last_idx = current_idx;
            current_idx = parent_idx;
        } else {
            return;
        }
    }

    current_idx = arena.arena[current_idx].left.unwrap();

    while arena.arena[current_idx].val.is_none() {
        current_idx = arena.arena[current_idx].right.unwrap();
    }
    let new_val = arena.arena[current_idx].val.unwrap() + val_to_add;
    arena.arena[current_idx].val = Some(new_val)
}


fn add_right_value(arena: &mut ArenaTree, start_idx: usize, val_to_add: u8) {
    let mut last_idx = start_idx;
    let parent_idx = arena.arena[start_idx].parent.unwrap();
    let mut current_idx = parent_idx;

    while let Some(right_idx) = arena.arena[current_idx].right {
        if right_idx != last_idx {
            break;
        }
        if let Some(parent_idx) = arena.arena[current_idx].parent {
            last_idx = current_idx;
            current_idx = parent_idx;
        } else {
            return;
        }
    }
    current_idx = arena.arena[current_idx].right.unwrap();

    while arena.arena[current_idx].val.is_none() {
        current_idx = arena.arena[current_idx].left.unwrap();
    }
    let new_val = arena.arena[current_idx].val.unwrap() + val_to_add;
    arena.arena[current_idx].val = Some(new_val)
}


fn get_max_combination_value(arena: &mut ArenaTree, numbers: &[usize]) -> usize {
    numbers.iter()
        .permutations(2)
        .map(|p| vec![*p[0], *p[1]])
        .map(|n| get_magnitude_sum(arena, &n))
        .max().unwrap()
}
