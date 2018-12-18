use regex::Regex;
use std::collections::HashMap;
use std::collections::HashSet;

use crate::util;

pub fn solve() {
    let input_file = "input-day-6.txt";
    println!("Day 6 answers");
    print!(" first puzzle: ");
    let answer = solve_first_file(input_file);
    println!("{}", answer);
    print!(" second puzzle: ");
    let answer = solve_second_file(input_file, 10000);
    println!("{}", answer);
}

fn solve_first_file(input: &str) -> usize {
    let v = util::aoc2018_row_file_to_vector(input);
    solve_first(&v)
}

fn solve_second_file(input: &str, limit_dist: usize) -> usize {
    let v = util::aoc2018_row_file_to_vector(input);
    solve_second(&v, limit_dist)
}

fn solve_second(str_vector: &[String], limit_dist: usize) -> usize {
    let (world, points) = shared_puzzle_start(str_vector);

    let mut area_count = 0;
    for x in world.x..=world.x2 {
        for y in world.y..=world.y2 {
            let mut sum_of_mh_dist = 0;
            for (_point_idx, point) in points.iter().enumerate() {
                sum_of_mh_dist += manhattan_distance(point, x, y);
            }
            if sum_of_mh_dist < limit_dist {
                area_count += 1;
            }
            //           println!("XXX at: {},{} sum of dist: {}", x,y, sum_of_mh_dist);
        }
    }

    area_count
}

fn solve_first(str_vector: &[String]) -> usize {
    let (mut world, points) = shared_puzzle_start(str_vector);

    // Expand your mind - expand the world (frame outer points by one)
    world.x -= 1;
    world.x2 += 1;
    world.y -= 1;
    world.y2 += 1;

    let (grid_width, grid_height) = world_width_and_height(&world);
    let grid_size = grid_width * grid_height;

    let mut res_vec: Vec<Option<usize>> = Vec::with_capacity(grid_size);
    for x in world.x..=world.x2 {
        for y in world.y..=world.y2 {
            // let mut point_dists : Vec<Option<usize>> = Vec::with_capacity(points.len());
            let mut min: usize = <usize>::max_value();
            let mut multiple = false;
            let mut min_point_idx = <usize>::max_value();
            for (point_idx, point) in points.iter().enumerate() {
                let m_dist = manhattan_distance(point, x, y);
                if m_dist == min {
                    multiple = true;
                } else if m_dist < min {
                    min = m_dist;
                    multiple = false;
                    min_point_idx = point_idx;
                }
                //                println!("XXX x,y   point.x,point.y -> dist for  point_idx: {},{}   {},{} -> {} for {}",  x, y, point.x, point.y, m_dist,point_idx);
            }
            if !points.is_empty() {
                if multiple {
                    res_vec.push(None);
                } else {
                    res_vec.push(Some(min_point_idx));
                }
            }
        }
    }

    let mut infinite_points: HashSet<usize> = HashSet::new();

    let mut local_x;
    let mut local_y;

    for (pos, grid_res) in res_vec.iter().enumerate() {
        local_y = pos % (grid_height);
        if let Some(x) = pos.checked_div(grid_height) {
            local_x = x;
        } else {
            local_x = 0;
        }
        //       println!("XXX local_x, local_y  grid_res: -> {},{} {:?}", local_x, local_y, grid_res);
        if let Some(point_idx) = grid_res {
            if local_x == 0
                || local_x == grid_width - 1
                || local_y == 0
                || local_y == grid_height - 1
            {
                //                println!("XXX Infinite as local x,y  skip point idx: {},{} -> {}", local_x, local_y, point_idx);
                infinite_points.insert(point_idx.to_owned());
            }
        }
    }

    // Let's summarize
    let mut point_areas: HashMap<usize, usize> = HashMap::new();
    for grid_res in res_vec {
        if let Some(winning_idx) = grid_res {
            if !infinite_points.contains(&winning_idx) {
                let mut area_to_set = 1;
                let curr_area = point_areas.get(&winning_idx);
                if let Some(curr_area) = curr_area {
                    area_to_set += curr_area;
                }
                point_areas.insert(winning_idx, area_to_set);
            }
        }
    }
    // And the winner is...
    let mut max_area = 0;
    for (_idx, area) in point_areas {
        if area > max_area {
            max_area = area;
        }
    }

    max_area
}

fn shared_puzzle_start(str_vector: &[String]) -> (World, Vec<Point>) {
    let re = Regex::new(r"^(\d+),\s*(\d+).*$").unwrap();
    let mut world: World = Default::default();
    let mut points: Vec<Point> = vec![];
    for (index, line) in str_vector.iter().enumerate() {
        let caps = re.captures(&line).unwrap();
        let x: usize = caps[1].parse().expect("Not an integer");
        let y: usize = caps[2].parse().expect("Not an integer");
        let point = Point { x, y };
        points.push(point);
        if index == 0 {
            world.x = x;
            world.x2 = x;
            world.y = y;
            world.y2 = y;
        } else {
            if x < world.x {
                world.x = x;
            } else if x > world.x2 {
                world.x2 = x;
            }
            if y < world.y {
                world.y = y;
            } else if y > world.y2 {
                world.y2 = y;
            }
        }
    }
    (world, points)
}

fn manhattan_distance(point: &Point, x: usize, y: usize) -> usize {
    point.x.max(x) - point.x.min(x) + point.y.max(y) - point.y.min(y)
}

fn world_width_and_height(world: &World) -> (usize, usize) {
    let w = world.x2 - world.x + 1;
    let h = world.y2 - world.y + 1;
    (w, h)
}

#[derive(Default)]
struct World {
    x: usize,
    y: usize,
    x2: usize,
    y2: usize,
}

#[derive(Debug)]
struct Point {
    x: usize,
    y: usize,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_day_6_first() {
        // provided examples
        let input = &[
            String::from("1, 1"),
            String::from("1, 6"),
            String::from("8, 3"),
            String::from("3, 4"),
            String::from("5, 5"),
            String::from("8, 9"),
        ];

        let answer = solve_first(input);
        assert_eq!(answer, 17);
    }

    #[test]
    fn test_day_6_second() {
        // provided examples
        let input = &[
            String::from("1, 1"),
            String::from("1, 6"),
            String::from("8, 3"),
            String::from("3, 4"),
            String::from("5, 5"),
            String::from("8, 9"),
        ];

        let answer = solve_second(input, 32);
        assert_eq!(answer, 16);
    }

}
