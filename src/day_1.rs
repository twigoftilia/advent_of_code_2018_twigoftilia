use crate::util;
use std::collections::BTreeSet;

pub fn solve() {
    let day1_input_file = "input-day-1.txt";

    println!("Day 1 answers");
    print!(" first puzzle: ");
    let answer = solve_first_file(day1_input_file);
    println!("{}", answer);

    print!(" second puzzle: ");
    let answer = solve_second_file(day1_input_file);
    println!("{}", answer);
}

fn solve_first_file(input: &str) -> i32 {
    let v = util::aoc2018_integer_file_to_vector(input);
    solve_first(&v)
}

fn solve_second_file(input: &str) -> i32 {
    let v = util::aoc2018_integer_file_to_vector(input);
    solve_second(&v)
}

fn solve_first(int_vector: &[i32]) -> i32 {
    let mut frequecy_diff = 0;
    for value in int_vector {
        frequecy_diff += value;
        //        println!("XXX {} -> frequecy_diff {}", value, frequecy_diff);
    }

    frequecy_diff
}

fn solve_second(int_vector: &[i32]) -> i32 {
    let mut frequecy_diff = 0;
    let mut used_freqs = BTreeSet::new();
    used_freqs.insert(frequecy_diff);

    loop {
        for value in int_vector {
            //            println!(
            //              "XXX Current {}, change {}, resulting {}",
            //            frequecy_diff,
            //          value,
            //      frequecy_diff + value
            //    );
            frequecy_diff += value;

            if used_freqs.contains(&frequecy_diff) {
                return frequecy_diff;
            }
            used_freqs.insert(frequecy_diff);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn solve_first_string(input: &str) -> i32 {
        let v = util::parse_string_of_insts_to_vec(input);
        solve_first(&v)
    }

    fn solve_second_string(input: &str) -> i32 {
        let v = util::parse_string_of_insts_to_vec(input);
        solve_second(&v)
    }

    #[test]
    fn test_cases_second() {
        // provided examples
        assert_eq!(solve_second_string("+1, -2, +3, +1"), 2);
        assert_eq!(solve_second_string("+1, -1"), 0);
        assert_eq!(solve_second_string("+3, +3, +4, -2, -4"), 10);
        assert_eq!(solve_second_string("-6, +3, +8, +5, -6"), 5);
        assert_eq!(solve_second_string("+7, +7, -2, -7, -4"), 14);
    }

    #[test]
    fn test_cases_first() {
        // my tests
        assert_eq!(solve_first_string(""), 0);
        assert_eq!(solve_first_string("0, +0, -0"), 0);

        assert_eq!(solve_first_file("test-day-1.txt"), 3);

        // provided examples
        assert_eq!(solve_first_string("+1, -2, +3, +1"), 3);
        assert_eq!(solve_first_string("+1, +1, +1"), 3);
        assert_eq!(solve_first_string("+1, +1, -2"), 0);
        assert_eq!(solve_first_string("-1, -2, -3"), -6);
    }

}
