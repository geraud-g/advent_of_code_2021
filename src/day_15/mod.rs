use std::cmp::Ordering;
use std::collections::BinaryHeap;
use std::collections::HashMap;

use itertools::iproduct;

use advent_of_code_2021::utils::inputs::get_file;
use advent_of_code_2021::utils::point::Point;

pub fn day_15() {
    let cave = get_input();

    let solution_a = dijkstra(&cave);
    println!("Solution for Day 15, part A is: {}", solution_a.unwrap());

    let full_cave = get_full_cave(&cave);
    let solution_b = dijkstra(&full_cave);
    println!("Solution for Day 15, part B is: {}", solution_b.unwrap());
}



fn get_input() -> Vec<Vec<usize>> {
    get_file("./src/day_15/input.txt").lines()
        .map(|line| line.chars().map(|c| c.to_digit(10).unwrap() as usize).collect())
        .collect()
}


fn get_full_cave(cave: &[Vec<usize>]) -> Vec<Vec<usize>> {
    let mut new_cave = vec![];
    for y in 0..5 {
        for line in cave {
            let mut new_line = vec![];
            for x in 0..5 {
                let line: Vec<_> = line.iter()
                    .map(|el|el + x + y)
                    .map(|el| if el > 9 {el - 9} else {el})
                    .collect();
                new_line.extend(line)
            }
            new_cave.push(new_line)
        }
    }
    new_cave
}


#[derive(Copy, Clone, Eq, PartialEq)]
struct State {
    cost: usize,
    position: Point,
}

// The priority queue depends on `Ord`.
// Explicitly implement the trait so the queue becomes a min-heap
// instead of a max-heap.
impl Ord for State {
    fn cmp(&self, other: &Self) -> Ordering {
        // Notice that the we flip the ordering on costs.
        // In case of a tie we compare positions - this step is necessary
        // to make implementations of `PartialEq` and `Ord` consistent.
        other.cost.cmp(&self.cost)
            .then_with(|| self.position.cmp(&other.position))
    }
}

// `PartialOrd` needs to be implemented as well.
impl PartialOrd for State {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}



fn dijkstra(cave: &[Vec<usize>]) -> Result<usize, &'static str> {
    let mut dist = HashMap::new();
    let height = cave.len();
    let width = cave[0].len();
    let mut heap = BinaryHeap::new();

    for point in iproduct!(0..height, 0..width).map(|(y, x)| Point::from_usize(y, x)) {
        dist.insert(point, usize::MAX);
    }
    let source = Point::from_usize(0, 0);
    let dest = Point::from_usize(height - 1, width - 1);
    heap.push(State { cost: 0, position: source });

    while let Some(State { cost, position }) = heap.pop() {
        // Alternatively we could have continued to find all shortest paths
        if position.eq(&dest) {
            return Ok(cost);
        }

        // Important as we may have already found a better way
        if cost > dist[&position] { continue; }

        // For each node we can reach, see if we can find a way with
        // a lower cost going through this node
        for neighbour in &position.get_neighbours(height, width) {
            let neighbour_cost = cave[neighbour.y][neighbour.x];
            let next = State { cost: cost + neighbour_cost, position: *neighbour };

            // If so, add it to the frontier and continue
            if next.cost < dist[&next.position] {
                heap.push(next);
                // Relaxation, we have now found a better way
                *dist.get_mut(&next.position).unwrap() = next.cost;
            }
        }
    }
    Err("Could not find any solution")
}
