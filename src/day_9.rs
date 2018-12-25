use crate::util;
use regex::Regex;
use std::collections::HashMap;
use std::collections::VecDeque; 

pub fn solve() {
    let input_file = "input-day-9.txt";
    println!("Day 9 answers");
    print!(" first puzzle: ");
    let answer1 = solve_file(input_file, 1);
    println!("{}", answer1);
    print!(" second puzzle: ");
    let answer2 = solve_file(input_file, 100);
    println!("{}", answer2);
}

fn solve_file(input: &str, marbles_factor: usize) -> usize {
    let v = util::aoc2018_row_file_to_vector(input);
    solve_puzzle(&v, marbles_factor)
}

fn solve_puzzle(str_vector: &[String], marble_factor: usize) -> usize {
    let (players, mut marbles) = shared_puzzle_start(str_vector);

    marbles *= marble_factor;

    let mut player_score: HashMap<usize, usize> = HashMap::new();
    let mut circle: VecDeque<usize> = VecDeque::with_capacity(marbles);
    let mut marble_val = 0;

    circle.push_front(marble_val);
    marble_val += 1;

    'outer: loop {
        for p in 0..players {
            if marble_val % 23 != 0 {
                for _rot_step in 0..2 {
                    let rot_marble = circle.pop_front().expect("Ain't popping");
                    circle.push_back(rot_marble);
                }
                circle.push_front(marble_val);
            } else {
                for _rot_step in 0..6 {
                    // Other way now
                    let rot_marble = circle.pop_back().expect("Ain't popping right...");
                    circle.push_front(rot_marble);
                }
                let removed_marble_val = circle.pop_back().unwrap();

                let round_points = removed_marble_val + marble_val;
                player_score
                    .entry(p)
                    .and_modify(|e| *e += round_points)
                    .or_insert(round_points);
            }
            marble_val += 1;
            if marble_val > marbles {
                break 'outer;
            }
        }
    }

    let mut max_score = 0;

    for (_k, v) in player_score {
        if v > max_score {
            max_score = v;
        };
    }
    max_score
}

fn shared_puzzle_start(str_vector: &[String]) -> (usize, usize) {
    if str_vector.len() != 1 {
        panic!("One-liners, if you please");
    }
    let line = &str_vector[0];

    let re = Regex::new(r"^(\d+) players; last marble is worth (\d+) points$").unwrap();

    let caps = re.captures(&line).unwrap();
    let a: usize = caps[1].parse().expect("Not an integer");
    let b: usize = caps[2].parse().expect("Not an integer");
    (a, b)
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_day_9_first() {
        let deps = vec![String::from("9 players; last marble is worth 25 points")];

        let a1 = solve_puzzle(&deps, 1);

        assert_eq!(a1, 32);

        let deps = vec![String::from("10 players; last marble is worth 1618 points")];
        let a1 = solve_puzzle(&deps, 1);
        assert_eq!(a1, 8317);

        let deps = vec![String::from("13 players; last marble is worth 7999 points")];
        let a1 = solve_puzzle(&deps, 1);
        assert_eq!(a1, 146373);

        let deps = vec![String::from("17 players; last marble is worth 1104 points")];
        let a1 = solve_puzzle(&deps, 1);
        assert_eq!(a1, 2764);

        let deps = vec![String::from("21 players; last marble is worth 6111 points")];
        let a1 = solve_puzzle(&deps, 1);
        assert_eq!(a1, 54718);

        let deps = vec![String::from("30 players; last marble is worth 5807 points")];
        let a1 = solve_puzzle(&deps, 1);
        assert_eq!(a1, 37305);
    }
}
