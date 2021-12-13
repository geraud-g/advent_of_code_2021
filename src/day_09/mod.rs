use itertools::{iproduct, Itertools};

use advent_of_code_2021::utils::inputs::get_file;
use advent_of_code_2021::utils::point::Point;

pub fn day_09() {
    let lava_tubes = get_input();
    let lowest_points = get_low_points(&lava_tubes);

    let solution_a = sum_risk_level(&lava_tubes, &lowest_points);
    println!("Solution for Day 9, part A is: {}", solution_a);

    let solution_b = get_basins_score(&lava_tubes, &lowest_points);
    println!("Solution for Day 9, part B is: {}", solution_b);
}


fn get_input() -> Vec<Vec<u32>> {
    get_file("./src/day_09/input.txt").lines()
        .map(|line| line.chars().map(|c| c.to_digit(10).unwrap()).collect())
        .collect()
}


fn get_low_points(tubes: &[Vec<u32>]) -> Vec<Point> {
    let height = tubes.len();
    let width = tubes[0].len();

    iproduct!(0..height, 0..width)
        .map(|(y, x)| Point::from_usize(y, x))
        .filter(|point| is_lowest_point(tubes, height, width, point))
        .collect()
}


fn is_lowest_point(tubes: &[Vec<u32>], height: usize, width: usize, point: &Point) -> bool {
    point.get_neighbours(height, width).iter()
        .all(|neighbour| tubes[neighbour.y][neighbour.x] > tubes[point.y][point.x])
}


fn sum_risk_level(tubes: &[Vec<u32>], lowest_points: &[Point]) -> u32 {
    lowest_points.iter()
        .map(|lowest_point| tubes[lowest_point.y][lowest_point.x] + 1)
        .sum()
}


fn get_basins_score(tubes: &[Vec<u32>], lowest_points: &[Point]) -> usize {
    let mut basins = vec![];
    let height = tubes.len();
    let width = tubes[0].len();

    for point in lowest_points {
        let mut visited = vec![];
        get_basin(tubes, height, width, &mut visited, point);
        basins.push(visited)
    }

    let basins_sizes: Vec<_> = basins.iter().map(|b| b.len()).sorted().collect();
    basins_sizes[basins_sizes.len() - 3..].iter()
        .product()
}


fn get_basin(tubes: &[Vec<u32>], height: usize, width: usize, visited: &mut Vec<Point>, point: &Point) {
    let val = tubes[point.y][point.x];
    visited.push(*point);

    for neighbour in &point.get_neighbours(height, width) {
        let neighbour_val = tubes[neighbour.y][neighbour.x];
        if neighbour_val > val && neighbour_val < 9 && !visited.contains(neighbour) {
            get_basin(tubes, height, width, visited, neighbour)
        }
    }
}

