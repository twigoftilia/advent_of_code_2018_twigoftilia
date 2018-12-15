use crate::util;
use regex::Regex;
use std::collections::HashMap;

pub fn solve() {
    let input_file = "input-day-4.txt";

    println!("Day 4 answers");
    print!(" first puzzle: ");
    let (answer_first, answer_first2, answer_first3, answer_second, answer_second2, answer_second3) =
        solve_file(input_file);
    println!("{}*{}={}", answer_first, answer_first2, answer_first3);
    print!(" second puzzle: ");
    println!("{}*{}={}", answer_second, answer_second2, answer_second3);

    // print!(" second puzzle: ");
    // let answer = solve_second_file(input_file);
    // println!("{}", answer);
}

fn solve_file(input: &str) -> (String, u32, u32, String, u32, u32) {
    let v = util::aoc2018_row_file_to_vector(input);
    solve_first(v)
}

fn solve_first(mut str_vector: Vec<String>) -> (String, u32, u32, String, u32, u32) {
    str_vector.sort();
    let re = Regex::new(
        r"^[\[](.{10})\s+.{3}(.{2})[\]]\s+(\w+)\s+#?(\S+)(.*)", // #\s+(\w)\s+(\d+)\s+
    )
    .unwrap();

    let mut guard_surveillance: HashMap<String, GuardsNightNaps> = HashMap::new();
    let mut guard_state = GuardState::None;

    let mut current_guard: String = "None".to_string();
    let mut current_sleep_minute: Option<u32> = None;

    for line in str_vector {
        //        println!("XXX line: {}", line);
        let caps = re.captures(&line).unwrap();

        let date_str = &caps[1];
        let min_str = &caps[2];
        let keyword_str = &caps[3];
        let opt_gnome_id_str = &caps[4];
        // println!(
        //     "XXXXXXXXXX dates_Str {}  {}  {}  {}",
        //     date_str, min_str, keyword_str, opt_gnome_id_str
        // );

        match keyword_str {
            "Guard" => {
                match guard_state {
                    GuardState::None | GuardState::Awake => {
                        guard_state = GuardState::Awake;
                        // note, defer setting date as guard may arrive before noon
                        current_guard = opt_gnome_id_str.to_string();
                    }
                    GuardState::Napping => {
                        panic!("State error, previous Guard must wake before new arrives");
                    }
                }
            }
            "falls" => match guard_state {
                GuardState::Awake => {
                    guard_state = GuardState::Napping;
                    current_sleep_minute = Some(min_str.parse().unwrap());
                }
                GuardState::None | GuardState::Napping => {
                    panic!("State error, a current guard must be awake");
                }
            },
            "wakes" => match guard_state {
                GuardState::Napping => {
                    guard_state = GuardState::Awake;
                    let wake_minute: u32 = min_str.parse().unwrap();
                    let nap: Nap = Nap {
                        sleep_minute: current_sleep_minute.unwrap(),
                        wake_minute,
                    };

                    //let mut current_guard_watch_vec: &Vec<GuardWatch>;
                    let b = &current_guard;
                    let guard_nites = guard_surveillance
                        .entry(b.to_string())
                        .or_insert_with(HashMap::new);
                    let guard_nite = guard_nites
                        .entry(date_str.to_string())
                        .or_insert_with(Vec::new);

                    guard_nite.push(nap);
                }
                GuardState::None | GuardState::Awake => {
                    panic!("State error, a current guard must be napping");
                }
            },
            _ => {
                panic!("Error parsing \"{}\"", line);
            }
        }
    }

    // first part solution
    //
    let mut mr_sleepy = "None".to_string();
    let mut mr_sleepys_sleept_time: u32 = 0;
    for (guard, nights) in &guard_surveillance {
        //println!("XXXX key {}", guard);
        let mut sleep_time = 0;
        for naps in nights.values() {
            //println!("XXXX    key {}", date);
            for nap in naps {
                sleep_time += nap.wake_minute - nap.sleep_minute;
            }
        }
        //println!("XXXX {} slept for {} minutes", guard, sleep_time);
        if sleep_time > mr_sleepys_sleept_time {
            mr_sleepys_sleept_time = sleep_time;
            mr_sleepy = guard.to_string();
        }
    }

    let mr_sleepys_nights = &guard_surveillance[&mr_sleepy];
    let (_top_value, mr_sleepys_top_minute) = max_minute_for_nights(mr_sleepys_nights);
    if mr_sleepys_top_minute == 0 {
        panic!("Can't handle non sleeping guards.");
    }
    let mr_sleepy_as_u32 = mr_sleepy
        .to_string()
        .parse::<u32>()
        .expect("Not an integer");

    // second part solution
    //
    let mut mr_predictable = "None".to_string();
    let mut mr_predictables_top_minute: u32 = 0;
    let mut mr_predictables_top_value: u32 = 0;
    for (guard, nights) in &guard_surveillance {
        let guard_nights = nights;
        let (top_value, top_minute) = max_minute_for_nights(guard_nights);
        //       println!("XXX Guard : {} -> top_value: {} : top_minute  {} ", guard, _top_value, top_minute);
        if top_value > mr_predictables_top_value {
            mr_predictables_top_value = top_value;
            mr_predictables_top_minute = top_minute;
            mr_predictable = guard.to_string();
        }
    }

    let mr_predictable_as_u32 = mr_predictable
        .to_string()
        .parse::<u32>()
        .expect("Not an integer");

    (
        mr_sleepy.to_string(),
        mr_sleepys_top_minute,
        mr_sleepy_as_u32 * mr_sleepys_top_minute,
        mr_predictable.to_string(),
        mr_predictables_top_minute,
        mr_predictable_as_u32 * mr_predictables_top_minute,
    )
}

// returns (top_value, top_minute)
// top_minute only valid if top_value > 0

fn max_minute_for_nights(nights: &GuardsNightNaps) -> (u32, u32) {
    let mut minutes_slept_at_minute: HashMap<u32, u32> = HashMap::new();
    let mut top_minute: u32 = 0;
    let mut top_value: u32 = 0;
    for naps in nights.values() {
        // println!("XXXX    key {}", date);
        for nap in naps {
            // println!("XXXX    nap {} {}", nap.sleep_minute, nap.sleep_minute );
            for minute in nap.sleep_minute..nap.wake_minute {
                let mut slept_inclusive: u32;
                {
                    let slept_before = minutes_slept_at_minute.get_mut(&minute);
                    slept_inclusive = 0;
                    if let Some(n) = slept_before {
                        slept_inclusive = 1 + *n;
                    } else {
                        slept_inclusive += 1;
                    }
                }
                if slept_inclusive > top_value {
                    top_value = slept_inclusive;
                    top_minute = minute;
                }
                // println!("XXXXXXXX inserting minutes {} at  key minute {} ", slept_inclusive, minute);
                minutes_slept_at_minute.insert(minute, slept_inclusive);
            }
        }
    }
    (top_value, top_minute)
}

type GuardsNightNaps = HashMap<String, Vec<Nap>>;

// Nap limited to nap that starts and ends same hour
pub struct Nap {
    sleep_minute: u32,
    wake_minute: u32,
}

enum GuardState {
    None,
    Awake,
    Napping,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_day_4_test_first() {
        // provided examples
        let (answer, answer2, answer3, answer_second, answer_second2, answer_second3) =
            solve_file("test-day-4.txt");
        assert_eq!(answer, "10");
        assert_eq!(answer2, 24);
        assert_eq!(answer3, 240);
        assert_eq!(answer_second, "99");
        assert_eq!(answer_second2, 45);
        assert_eq!(answer_second3, 4455);
    }

}
