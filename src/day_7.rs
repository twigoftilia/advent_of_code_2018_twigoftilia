use crate::util;
use regex::Regex;
use std::collections::BTreeMap;
use std::collections::BTreeSet;

pub fn solve() {
    let input_file = "input-day-7.txt";
    println!("Day 7 answers");
    print!(" first puzzle: ");
    let answer = solve_first_file(input_file);
    println!("{}", answer);
    print!(" second puzzle: ");
    let build_time = solve_second_file(input_file, 5, 60);
    println!("{}", build_time);
}

fn solve_first_file(input: &str) -> String {
    let v = util::aoc2018_row_file_to_vector(input);
    solve_first(&v)
}

fn solve_second_file(input: &str, workers: usize, base_delay: usize) -> usize {
    let v = util::aoc2018_row_file_to_vector(input);
    solve_second(&v, workers, base_delay)
}

fn solve_first(str_vector: &[String]) -> String {
    let (step_dep_map, dep_on_me_map) = shared_puzzle_start(str_vector);

    let mut start_steps = BTreeSet::new();
    for (key, val) in &step_dep_map {
        if val.is_empty() {
            start_steps.insert(*key);
        }
    }

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

fn solve_second(str_vector: &[String], no_of_workers: usize, base_delay: usize) -> usize {
    let (step_dep_map, dep_on_me_map) = shared_puzzle_start(str_vector);

    let mut start_steps = BTreeSet::new();
    for (key, val) in &step_dep_map {
        if val.is_empty() {
            start_steps.insert(*key);
        }
    }

    let mut workers: Vec<(usize, char)> = Vec::with_capacity(no_of_workers);
    workers.resize(no_of_workers, (0, '-'));

    // Resolve Returns possible steps to resolve after thid
    fn recursive_func(
        chars: BTreeSet<char>,
        step_dep_map: &StepIsDepOnMap,
        dep_on_me_map: &DepOnMeMap,
        cur_time: usize,
        base_delay: usize,
        workers: &mut Vec<(usize, char)>,
        done_steps: &mut Vec<char>,
    ) -> usize {
        let mut next_possbile_steps: BTreeSet<char> = BTreeSet::new();
        let mut next_possbile_steps2: BTreeSet<char> = BTreeSet::new();
        for c in chars {
            next_possbile_steps.insert(c);
            next_possbile_steps2.insert(c);
        }

        let mut max_time = 0;

        let mut next_free_worker_time: Option<usize> = None;

        for (_i, worker) in workers.into_iter().enumerate() {
            let (w_time, w_char) = worker;
            if *w_time == cur_time && *w_char >= 'A' && *w_char <= 'Z' {
                done_steps.push(*w_char);

                max_time = cur_time;

                next_possbile_steps.remove(w_char);
                next_possbile_steps2.remove(w_char);
                //                   println!("XXX DONE step {} :  possible_steps: {:?}", *w_char, next_possbile_steps);

                // ok, made any steps available?
                let dep_on_me_set_option = dep_on_me_map.get(w_char);
                if let Some(dep_on_me_set) = dep_on_me_set_option {
                    for dep_on_me_char in dep_on_me_set {
                        // all those that directly depends on the char
                        let mut pass = true;
                        let step_deps = step_dep_map.get(dep_on_me_char); // get all chars that it depends on (one should be the one we just set done)

                        if let Some(step_deps) = step_deps {
                            for x in step_deps {
                                if !done_steps.contains(x) {
                                    //                                       println!("XXX step {} blocks {} for bereing added", x, dep_on_me_char);
                                    pass = false;
                                    break;
                                }
                            }
                            if pass {
                                // println!("XXX - new step available: {} ", dep_on_me_char);
                                next_possbile_steps.insert(*dep_on_me_char);
                                next_possbile_steps2.insert(*dep_on_me_char);
                            }
                        }
                    }
                }
            }
        }

        for (_i, ch) in next_possbile_steps.into_iter().enumerate() {
            //          println!("XXXX i, ch: {}, {:?}  ", i, ch);

            let mut next_free_gnome: Option<usize> = None;
            for (i, worker) in workers.into_iter().enumerate() {
                let (w_time, _w_char) = worker;

                if *w_time <= cur_time {
                    next_free_gnome = Some(i);
                    //                    println!("XXXX Worker for {} found,  {}", ch, i);
                    break;
                }
            }
            if let Some(worker_no) = next_free_gnome {
                let step_duration = ch as usize - 'A' as usize + 1 + base_delay;

                let fin_time = step_duration + cur_time;
                workers[worker_no] = (fin_time, ch);
                next_possbile_steps2.remove(&ch);
            }
        }

        // Set next time to when next active worker is done
        for (_i, worker) in workers.into_iter().enumerate() {
            let (w_time, _w_char) = worker;
            if *w_time > cur_time {
                if let Some(time) = next_free_worker_time {
                    if *w_time < time {
                        next_free_worker_time = Some(*w_time);
                    }
                } else {
                    next_free_worker_time = Some(*w_time);
                }
            }
        }

        if let Some(time) = next_free_worker_time {
            let fin_time = recursive_func(
                next_possbile_steps2,
                step_dep_map,
                dep_on_me_map,
                time,
                base_delay,
                workers,
                done_steps,
            );
            if fin_time > max_time {
                max_time = fin_time;
            }
        }
        max_time
    }

    let mut done_steps: Vec<char> = vec![];

    let build_time = recursive_func(
        start_steps,
        &step_dep_map,
        &dep_on_me_map,
        0,
        base_delay,
        &mut workers,
        &mut done_steps,
    );

    let mut res = String::new();
    for ch in done_steps {
        res.push(ch);
    }

    build_time
}

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
    #[test]
    fn test_day_7_second() {
        let deps = vec![
            String::from("Step C must be finished before step A can begin."),
            String::from("Step C must be finished before step F can begin."),
            String::from("Step A must be finished before step B can begin."),
            String::from("Step A must be finished before step D can begin."),
            String::from("Step B must be finished before step E can begin."),
            String::from("Step D must be finished before step E can begin."),
            String::from("Step F must be finished before step E can begin."),
        ];
        let build_time = solve_second(&deps, 2, 0);

        assert_eq!(build_time, 15);
    }

}
