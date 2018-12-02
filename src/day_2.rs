use std::collections::HashMap;
use util;

// Ok, pretty hackish and direct. Horrible, but its late...
// Lots of known limitation, i.e. only works for ascii (one char / byte) input

pub fn solve() {
    let input_file = "input-day-2.txt";

    println!("Day 2 answers");
    print!(" first puzzle: ");
    let (answer, _) = solve_first_file(input_file);
    println!("{}", answer);

    print!(" second puzzle: ");
    let answer = solve_second_file(input_file);
    println!("{}", answer);
}

fn solve_first_file(input: &str) -> (i32, Vec<String>) {
    let v = util::aoc2018_row_file_to_vector(input);
    solve_first(v)
}

fn solve_first(str_vector: Vec<String>) -> (i32, Vec<String>) {
    let mut doubles = 0;
    let mut triplets = 0;
    let mut filtered_box_ids = Vec::<String>::new();

    for box_id in str_vector {
        let mut double_hit: bool = false;
        let mut triple_hit: bool = false;

        // println!("XXX Checking box id: {}", box_id);

        let box_id_char_vec: Vec<char> = box_id.chars().collect();
        let mut char_hits = HashMap::new();

        for c in box_id_char_vec {
            let new_no_of_c;
            {
                let current_no_of_char = char_hits.get(&c);
                if let Some(no_of_c) = current_no_of_char {
                    // println!("XXX matched {:?} {:?} time", c, no_of_c);
                    new_no_of_c = no_of_c + 1;
                } else {
                    new_no_of_c = 1;
                }
            }
            char_hits.insert(c, new_no_of_c);
        }

        for value in char_hits.values() {
            if !double_hit && *value == 2 {
                double_hit = true;
            } else if !triple_hit && *value == 3 {
                triple_hit = true;
            }
            if double_hit && triple_hit {
                break;
            }
            // println!("XXX key, val: {}: {}", key, value);
        }

        if double_hit || triple_hit {
            filtered_box_ids.push(box_id.to_string());

            if double_hit {
                doubles += 1;
            }

            if triple_hit {
                triplets += 1;
            }
        }

        //        println!(
        //            "XXXXXX boxid, doubles, triplets {} {} {}",
        //            box_id, doubles, triplets
        //        );
    }
    (doubles * triplets, filtered_box_ids)
}

fn solve_second_file(input: &str) -> String {
    let (_, v) = solve_first_file(input);
    solve_second(&v)
}

// Some shortcuts as we know we're searcing two specific top matching boxes
// XXXXfn solve_second(str_vector: Vec<String>) -> String {
    fn solve_second(str_vector: &[String]) -> String {
    let mut best_match = 0;
    let box_ids = str_vector.len();
    let box_id_last_idx = box_ids - 1;
    let mut box_id_1: Option<String> = None;
    let mut box_id_2: Option<String> = None;

    for (current_pos, current_box_id) in str_vector.iter().enumerate() {
        // println!("XXX solve_second input: {} {} ", current_pos, current_box_id);
        // Skip last as we only compare to those after.
        if current_pos == box_id_last_idx {
            break;
        }
        // with subsequent box ids
        let sub_str_vector = &str_vector[(current_pos + 1)..];
        for (_cmp_pos, cmp_box_id) in sub_str_vector.iter().enumerate() {
            let shared_in_position = compare_box_ids(current_box_id, cmp_box_id);
            //println!("XXX    shared_in_position: {} ", shared_in_position);
            if shared_in_position > best_match {
                best_match = shared_in_position;
                box_id_1 = Some(current_box_id.to_string());
                box_id_2 = Some(cmp_box_id.to_string());
            }
        }
    }

    match box_id_1 {
        None => {
            "Fail".to_string()
        }
        Some(b1) => {
            let b2 = box_id_2.unwrap();
            chars_at_correct_place(&b1, &b2)
        }
    }
}

fn chars_at_correct_place(s: &str, s2: &str) -> String {
    let len_to_cmp = (s.len().min(s2.len())) as i32;
    let mut string = String::new();
    for i in 00..len_to_cmp {
        let c1 = s.as_bytes()[i as usize];
        if c1 == s2.as_bytes()[i as usize] {
            string.push(c1 as char);
        }
    }
    string
}

fn compare_box_ids(s: &str, s2: &str) -> i32 {
    let len_to_cmp = (s.len().min(s2.len())) as i32;
    let mut no_of_matches = 0;
    for i in 00..len_to_cmp {
        if s.as_bytes()[i as usize] == s2.as_bytes()[i as usize] {
            no_of_matches += 1;
        }
    }
    // println!("XXX  last_idx {} s: {} <> {}  matches: {}", last_idx, s, s2, no_of_matches);
    no_of_matches
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_cases_first() {
        // provided examples

        let (answer, _) = solve_first(vec![
            "abcdef".to_string(),
            "bababc".to_string(),
            "abbcde".to_string(),
            "abcccd".to_string(),
            "aabcdd".to_string(),
            "abcdee".to_string(),
            "ababab".to_string(),
        ]);

        assert_eq!(answer, 12);
        let (answer, _) = solve_first_file("test-day-2.txt");
        assert_eq!(answer, 12);
    }

    #[test]
    fn test_cases_second() {
        // provided examples

        let answer = solve_second(&vec![
            "abcde".to_string(),
            "fghij".to_string(),
            "klmno".to_string(),
            "pqrst".to_string(),
            "fguij".to_string(),
            "axcye".to_string(),
            "wvxyz".to_string(),
        ]);

        assert_eq!(answer, "fgij".to_string());
    }

}
