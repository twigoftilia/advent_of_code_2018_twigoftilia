use crate::util;

pub fn solve() {
    let input_file = "input-day-8.txt";
    println!("Day 8 answers");
    print!(" first puzzle: ");
    let (answer1, answer2) = solve_both_file(input_file);
    println!("{}", answer1);
    print!(" second puzzle: ");
    println!("{}", answer2);
}

fn solve_both_file(input: &str) -> (usize, usize) {
    let v = util::aoc2018_row_file_to_vector(input);
    solve_both(&v)
}

fn solve_both(str_vector: &[String]) -> (usize, usize) {
    let numbers = shared_puzzle_start(str_vector);

    #[derive(Default)]
    struct RecursiveResultFirst {
        metadata_sum: usize,
        length: usize,
    }
    // Resolve Returns possible steps to resolve after thid
    fn recursive_func_first(numbers: &[usize], node_start: usize) -> RecursiveResultFirst {
        let mut res: RecursiveResultFirst = Default::default();
        let no_childs = numbers[node_start];
        let no_meta = numbers[node_start + 1];
        let mut idx = node_start + 2;
        for _i in 0..no_childs {
            let child_res = recursive_func_first(numbers, idx);
            res.metadata_sum += child_res.metadata_sum;
            idx += child_res.length;
        }
        for _i in 0..no_meta {
            res.metadata_sum += numbers[idx];
            idx += 1;
        }
        res.length = idx - node_start;
        res
    }

    #[derive(Default)]
    struct RecursiveResultSecond {
        node_value: usize,
        length: usize,
    }

    // Resolve Returns possible steps to resolve after thid
    fn recursive_func_second(numbers: &[usize], node_start: usize) -> RecursiveResultSecond {
        let mut res: RecursiveResultSecond = Default::default();
        let no_childs = numbers[node_start];
        let no_meta = numbers[node_start + 1];
        let mut idx = node_start + 2;

        let mut child_node_vals: Vec<usize> = vec![];
        for _i in 0..no_childs {
            let child_res = recursive_func_second(numbers, idx);
            child_node_vals.push(child_res.node_value);
            idx += child_res.length;
        }

        for _i in 0..no_meta {
            let meta_val = numbers[idx];
            if no_childs == 0 {
                res.node_value += meta_val;
            } else if meta_val <= no_childs {
                res.node_value += child_node_vals[meta_val - 1];
            }

            idx += 1;
        }
        res.length = idx - node_start;
        res
    }

    let res_first = recursive_func_first(&numbers, 0);
    let res_second = recursive_func_second(&numbers, 0);

    (res_first.metadata_sum, res_second.node_value)
}

fn shared_puzzle_start(str_vector: &[String]) -> Vec<usize> {
    let mut res_vec = Vec::new();
    for (_index, line) in str_vector.iter().enumerate() {
        for s in line.split_whitespace() {
            let i = s.parse().expect("Error parsing to integer");
            res_vec.push(i);
        }
    }
    res_vec
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_day_7_first() {
        let deps = vec![String::from("2 3 0 3 10 11 12 1 1 0 1 99 2 1 1 2")];
        let (a1, a2) = solve_both(&deps);
        assert_eq!(a1, 138);
        assert_eq!(a2, 66);
    }
}
