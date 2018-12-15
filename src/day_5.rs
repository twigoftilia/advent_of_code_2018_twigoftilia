use std::cmp;
use std::collections::BTreeMap;
use std::collections::BTreeSet;

use crate::util;

// Not to happy with this one, tried a couple times, still slow (~30 sec).
// Plenty to optimaze, but lets go on. 
// Also now rust 2018 edition.
//
pub fn solve() {
    let input_file = "input-day-5.txt";
    println!("Day 5 answers");
    print!(" first puzzle: ");
    let answer = solve_first_file(input_file);
    println!("{}", answer);
    print!(" second puzzle: ");
    let answer = solve_second_file(input_file);
    println!("{}", answer);

    // print!(" second puzzle: ");
    // let answer = solve_second_file(input_file);
    // println!("{}", answer);
}

fn solve_first_file(input: &str) -> usize {
    let v = util::aoc2018_row_file_to_vector(input);
    solve_first(&v)
}

fn solve_second_file(input: &str) -> usize {
    let v = util::aoc2018_row_file_to_vector(input);
    solve_second(&v)
}

fn solve_first(str_vector: &[String]) -> usize {
    let input;
    if str_vector.len() == 1 {
        input = &str_vector[0];
    } else {
        panic!("One row content expected");
    }

    let mut reduced_indexes: ReducedIndices = Default::default();
    reduced_indexes.limit = input.len() as u32;
    reduce_polymer(input, &mut reduced_indexes);
    let reduced_polymer = reduced_indexes.get_reduced_string(input);
    //  println!("XXXX reduced_polymer:_ {} ", reduced_polymer);
    
    reduced_polymer.len()
}

fn solve_second(str_vector: &[String]) -> usize {
    let input;
    if str_vector.len() == 1 {
        input = &str_vector[0];
    } else {
        panic!("One row content expected");
    }
    let input_len = input.len();
    let mut chars: BTreeSet<char> = BTreeSet::new();
    let mut char_reduced_size: BTreeMap<char, usize> = BTreeMap::new();
    for i in 0..input_len {
        let c = input.as_bytes()[i] as char;
        chars.insert(c.to_ascii_uppercase());
    }

    for c in chars {
        let mut reduced_indexes: ReducedIndices = Default::default();
        reduced_indexes.limit = input_len as u32;
        reduce_char(c, input, &mut reduced_indexes);
        reduce_polymer(input, &mut reduced_indexes);
        let reduced_polymer = reduced_indexes.get_reduced_string(input);
        let reduced_polymer_len = reduced_polymer.len();
        char_reduced_size.insert(c.to_ascii_uppercase(), reduced_polymer_len);
        //let m_c = char_reduced_size.get_mut(&c.to_ascii_uppercase());
        //m_c.unwrap() = char_pol_len;

 //       println!("XXX Char -> reduction {}->{}", c, reduced_polymer_len);
    }
    let min_opt = char_reduced_size.values().min();
    let min = min_opt.unwrap();
    
    min.to_owned()
}

fn reduce_char(ch: char, input: &str, reduced_indexes: &mut ReducedIndices) {
    for i in 0..input.len() {
        let c = input.as_bytes()[i as usize] as char;
        if c.to_ascii_uppercase() == ch {
            reduced_indexes.add_reduced(i as u32, i as u32);
        }
    }
}

fn reduce_polymer(in_polymer: &str, reduced_indexes: &mut ReducedIndices) {
    loop {
        //        println!("XXXX iter start:  iter no, current no of reductions ranges: {},{}", reduce_iter, reduced_indexes.ranges.len());
        let mut skip_back_check = true;
        let mut last_char = '.';
        let mut last_char_pos = 0;
        let mut loop_reductions: Vec<(u32, u32)> = Vec::new();

        for i in reduced_indexes.into_iter() {
            let c = in_polymer.as_bytes()[i as usize] as char;
            let c_uc = c.to_ascii_uppercase();
            if !skip_back_check && c != last_char && c_uc == last_char.to_ascii_uppercase() {
                // found a reduction
                loop_reductions.push((last_char_pos, i));
                skip_back_check = true;
            } else {
                skip_back_check = false;
            }
            last_char = c;
            last_char_pos = i;
        }

        for (start, end) in &loop_reductions {
            reduced_indexes.add_reduced(start.to_owned(), end.to_owned());
        }

        if loop_reductions.is_empty() {
            break;
        }
    }
}

#[derive(Default)]
struct ReducedIndices {
    ranges: BTreeMap<u32, u32>,
    limit: u32,
}

impl<'a> IntoIterator for &'a ReducedIndices {
    type Item = u32;
    type IntoIter = ReducedIndicesIntoIterator<'a>;

    fn into_iter(self) -> Self::IntoIter {
        ReducedIndicesIntoIterator {
            reduced_indices: self,
            initialised: false,
            index_to_check_from: 0,
            keys_left: vec![],
            next_stop_position: self.limit,
            next_reduced_range_stop: None
           // my_memory: Default::default(),
        }
    }
}

struct ReducedIndicesIntoIterator<'a> {
    reduced_indices: &'a ReducedIndices,
    initialised: bool,
    index_to_check_from: u32,
    keys_left: Vec<u32>,
    next_stop_position: u32,
    next_reduced_range_stop: Option<u32>,
}

impl<'a> Iterator for ReducedIndicesIntoIterator<'a> {
    type Item = u32;

    fn next(&mut self) -> Option<u32> {
        if !self.initialised {
            let keys = self.reduced_indices.ranges.keys();
            //keys
            // println!("XXXX next() init Adding keys: {:?} ", keys);

            for k in keys {
                self.keys_left.insert(0, k.to_owned());
            }

            // Add first range if any
            if let Some(s) = self.keys_left.pop() {
                self.next_stop_position = cmp::min(s, self.reduced_indices.limit);
                // let e = self.reduced_indices.ranges.get(&s).unwrap();
                let e = &self.reduced_indices.ranges[&s];
                self.next_reduced_range_stop = Some(e.to_owned());
                //            println!("XXXX next() init: next_stop_position, next_reduced_range_stop : {},{:?}", self.next_stop_position, self.next_reduced_range_stop);
            }
            self.initialised = true;
        }

        loop {
            if self.index_to_check_from < self.next_stop_position {
                let cur_pos = self.index_to_check_from;
                self.index_to_check_from += 1;
                return Some(cur_pos);
            }

            //                println!("XXX index_to_check_from: {}", self.index_to_check_from);
            if self.index_to_check_from >= self.reduced_indices.limit {
                return None;
            }

            if let Some(range_end) = self.next_reduced_range_stop {
                self.index_to_check_from = range_end + 1;

                if let Some(s) = self.keys_left.pop() {
                    // Any more reducing ranges?
                    self.next_stop_position = cmp::min(s, self.reduced_indices.limit);
                    let e = &self.reduced_indices.ranges[&s];
                    self.next_reduced_range_stop = Some(e.to_owned());
                } else {
                    self.next_stop_position = self.reduced_indices.limit;
                }
            }
        }
    }
}

fn overlap_or_intersect(a1: u32, a2: u32, b1: u32, b2: u32) -> bool {
    if a1 <= b2 && b1 <= a2
        || (a1 as i32 - b1 as i32).abs() == 1
        || (a1 as i32 - b2 as i32).abs() == 1
        || (a2 as i32 - b1 as i32).abs() == 1
        || (a2 as i32 - b2 as i32).abs() == 1
    {
        return true;
    }
    false
}

impl ReducedIndices {
    fn add_reduced(&mut self, start: u32, end: u32) {
        let mut overlapping: Vec<u32> = vec![];
        let mut lowest_start: u32 = start;
        let mut max_end: u32 = end;
        for (cmp_start, cmp_end) in &self.ranges {
            if overlap_or_intersect(start, end, *cmp_start, *cmp_end) {
                lowest_start = cmp::min(lowest_start, *cmp_start);
                max_end = cmp::max(max_end, *cmp_end);
                overlapping.push(*cmp_start);
            }
        }
        for idx in overlapping {
            self.ranges.remove(&idx);
        }
        self.ranges.insert(lowest_start, max_end);
    }

    fn get_reduced_string(&self, input: &str) -> String {
        let mut reduced_string = "".to_string();

        for i in self {
            let slice = &input[i as usize..=i as usize]; // its just AoC dammit
                                                         //            println!("XXXXXXXXXXXXXXXXXXXXXXXX: slice from i={} : >{}<", i, slice);
            reduced_string.push_str(slice)
        }
        reduced_string
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_day_5_first() {
        // provided examples
        let answer = solve_first(&[String::from("dabAcCaCBAcCcaDA")]);
        assert_eq!(answer, 10);
    }

    #[test]
    fn test_day_5_lab() {
        let input = "dabAcCaCBAcCcaDA";
        let mut reduced_indexes: ReducedIndices = Default::default();
        reduced_indexes.limit = input.len() as u32;

        reduced_indexes.add_reduced(4, 5);
        reduced_indexes.add_reduced(10, 11);

        let mut count = 0;
        for i in reduced_indexes.into_iter() {
            count += 1;
            println!("Test : count, i: {}>{}", count, i);
        }
    }

    #[test]
    fn test_day_5_internals_1() {
        // provided examples
        let input = "0123456";

        let mut reduced_indexes: ReducedIndices = Default::default();
        reduced_indexes.limit = input.len() as u32;

        assert_eq!(reduced_indexes.into_iter().count(), input.len());

        reduced_indexes.add_reduced(1, 2);
        reduced_indexes.add_reduced(3, 4);

        let mut count = 0;
        for i in reduced_indexes.into_iter() {
            count += 1;
            println!("Test : count, i: {}>{}", count, i);
        }
        assert_eq!(count, input.len() - 4);
        println!(
            "Test : string -> {} ",
            reduced_indexes.get_reduced_string(input)
        );

        reduced_indexes.add_reduced(6, 6);
        assert_eq!(reduced_indexes.into_iter().count(), input.len() - 5);
    }

    #[test]
    fn test_day_5_internals_2() {
        // provided examples
        let input = "0123456";

        let mut reduced_indexes: ReducedIndices = Default::default();
        reduced_indexes.limit = input.len() as u32;
        // assert_eq!(reduced_indexes.into_iter().count(), input.len());
        //  reduced_indexes.add_reduced(0, 0);
        // reduced_indexes.add_reduced(1, 1);

        let mut count = 0;
        for i in reduced_indexes.into_iter() {
            count += 1;
            println!("Test : count, i: {}>{}", count, i);
        }
        assert_eq!(count, 7);
    }

    #[test]
    fn test_day_5_internals_overlap() {
        // provided examples
        let input = "0123456";

        let mut reduced_indexes: ReducedIndices = Default::default();
        reduced_indexes.limit = input.len() as u32;

        print_ranges(&reduced_indexes);
        reduced_indexes.add_reduced(1, 2);
        print_ranges(&reduced_indexes);
        reduced_indexes.add_reduced(2, 4);
        print_ranges(&reduced_indexes);

        reduced_indexes.add_reduced(2, 3);

        print_ranges(&reduced_indexes);

        //        assert_eq!(count, 7);
    }

    #[test]
    fn test_day_5_second() {
        // provided examples
        let answer = solve_second(&[String::from("dabAcCaCBAcCcaDA")]);
        assert_eq!(answer, 4);
    }

    fn print_ranges(reduced_indexes: &ReducedIndices) {
        let mut i = 0;
        for (start, end) in &reduced_indexes.ranges {
            println!("Range {} : {}-{}", i, start, end);
            i += 1;
            if i > 10 {
                return;
            }
        }
    }

}
