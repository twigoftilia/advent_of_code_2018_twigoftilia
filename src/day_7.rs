use regex::Regex;
use std::collections::BTreeMap;
use std::collections::BTreeSet;
use crate::util;

pub fn solve() {
    let input_file = "input-day-7.txt";
    println!("Day 7 answers");
    print!(" first puzzle: ");
    let answer = solve_first_file(input_file);
    println!("{}", answer);
    print!(" second puzzle: ");
    // let answer = solve_second_file(input_file, 10000);
    // println!("{}", answer);
}

fn solve_first_file(input: &str) -> String {
    let v = util::aoc2018_row_file_to_vector(input);
    solve_first(&v)
}

fn solve_first(str_vector: &[String]) -> String {
    let (step_dep_map, dep_on_me_map) = shared_puzzle_start(str_vector);

    let mut start_steps = BTreeSet::new();
    for (key, val) in &step_dep_map {
        if val.is_empty() {
            start_steps.insert(*key);
        }
    }

    // if start_steps.len() != 1 {
    //     panic!(
    //         "Expected one and only one start step, found {}",
    //         start_steps.len()
    //     );
    // }

    // Resolve Returns possible steps to resolve after thid
    fn recursive_func(
        chars: BTreeSet<char>,
        step_dep_map: &StepIsDepOnMap,
        dep_on_me_map: &DepOnMeMap,
        done_steps: &mut Vec<char>,
    ) {
//        println!("Resolve possible steps {:?} ...", chars);

        let mut next_possbile_steps: BTreeSet<char> = BTreeSet::new();
        for (i, ch) in chars.into_iter().enumerate() {
            // loop over all possible steps. They are already sorted so first is to be markde as done

            if i == 0 {
                done_steps.push(ch);

                // ok, made any steps available?
                let dep_on_me_set_option = dep_on_me_map.get(&ch);
                if let Some(dep_on_me_set) = dep_on_me_set_option {
                    for dep_on_me_char in dep_on_me_set {
                        // all those that directly depends on the char
                        let mut pass = true;
                        let step_deps = step_dep_map.get(dep_on_me_char); // get all chars that it depends on (one should be the one we just set done)

                        if let Some(step_deps) = step_deps {
                            for x in step_deps {
                                if !done_steps.contains(x) {
                                    pass = false;
                                    break;
                                }
                            }
                            if pass {
                                next_possbile_steps.insert(*dep_on_me_char);
                            }
                        }
                    }
                }
            } else {
                next_possbile_steps.insert(ch);
            }
        }

        if !next_possbile_steps.is_empty() {
            recursive_func(next_possbile_steps, step_dep_map, dep_on_me_map, done_steps);
        }
    }

    let mut done_steps: Vec<char> = vec![];
    recursive_func(start_steps, &step_dep_map, &dep_on_me_map, &mut done_steps);

    let mut res = String::new();
    for ch in done_steps {
        res.push(ch);
    }

    res
}
// XXX Line: Step C must be finished before step A can begin.
// XXX Line: Step C must be finished before step F can begin.
// XXX Line: Step A must be finished before step B can begin.
// XXX Line: Step A must be finished before step D can begin.
// XXX Line: Step B must be finished before step E can begin.
// XXX Line: Step D must be finished before step E can begin.
// XXX Line: Step F must be finished before step E can begin.
// Step A depends on {'C'}
// Step B depends on {'A'}
// Step C depends on {}
// Step D depends on {'A'}
// Step E depends on {'B', 'D', 'F'}
// Step F depends on {'C'}
// Step A must be done before: {'B', 'D'}
// Step B must be done before: {'E'}
// Step C must be done before: {'A', 'F'}
// Step D must be done before: {'E'}
// Step E must be done before: {}
// Step F must be done before: {'E'}
//   -->A--->B--
//  /    \      \
// C      -->D----->E
//  \           /
//   ---->F-----

fn shared_puzzle_start(str_vector: &[String]) -> (StepIsDepOnMap, DepOnMeMap) {
    let re =
        Regex::new(r"^Step (\w{1}) must be finished before step (\w{1}) can begin\.$").unwrap();

    let mut step_dep_map: StepIsDepOnMap = StepIsDepOnMap::new(); // key is dependent on
    let mut dep_on_me_map: DepOnMeMap = DepOnMeMap::new(); // are dependent of key

    for (_index, line) in str_vector.iter().enumerate() {
  //      println!("XXX Line: {} ", line);
        let caps = re.captures(&line);

        if let Some(caps) = caps {
            let dep_step = caps[1].chars().next().unwrap();
            let step = caps[2].chars().next().unwrap();

            let dep_set = step_dep_map.entry(step).or_insert_with(BTreeSet::new);
            dep_set.insert(dep_step);
            step_dep_map.entry(dep_step).or_insert_with(BTreeSet::new);

            let dep_on_me_set = dep_on_me_map.entry(dep_step).or_insert_with(BTreeSet::new);
            dep_on_me_set.insert(step);
            dep_on_me_map.entry(step).or_insert_with(BTreeSet::new);
        } else {
            panic!("Can't parse line: {}", line);
        }
    }

    // for (step, dep_set) in &step_dep_map {
    //     println!("Step {} depends on {:?}", step, dep_set);
    // }
    // for (step, dep_on_me_set) in &dep_on_me_map {
    //     println!("Step {} must be done before: {:?}", step, dep_on_me_set);
    // }

    (step_dep_map, dep_on_me_map)
}

type StepIsDepOnMap = BTreeMap<char, BTreeSet<char>>;
type DepOnMeMap = BTreeMap<char, BTreeSet<char>>;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_day_7_first() {
        let deps = vec![
            String::from("Step C must be finished before step A can begin."),
            String::from("Step C must be finished before step F can begin."),
            String::from("Step A must be finished before step B can begin."),
            String::from("Step A must be finished before step D can begin."),
            String::from("Step B must be finished before step E can begin."),
            String::from("Step D must be finished before step E can begin."),
            String::from("Step F must be finished before step E can begin."),
        ];
        let answer = solve_first(&deps);

        assert_eq!(answer, "CABDFE");
    }

}
