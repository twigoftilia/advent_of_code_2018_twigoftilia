use std::collections::HashMap;
use std::collections::HashSet;

use crate::util;

pub fn solve() {
    let input_file = "input-day-3.txt";

    println!("Day 3 answers");
    print!(" first puzzle: ");
    let answer = solve_first_file(input_file);
    println!("{}", answer);

    print!(" second puzzle: ");
    let answer = solve_second_file(input_file);
    println!("{}", answer);
}

fn solve_first_file(input: &str) -> i32 {
    let v = util::aoc2018_row_file_to_vector(input);
    solve_first(v)
}

fn solve_first(str_vector: Vec<String>) -> i32 {
    let claims: Vec<Claim> = get_claims(str_vector);

    let mut overclaimed_rects: Vec<Rect> = Vec::new();

    let no_of_claims = claims.len();
    for (current_pos, claim) in claims.iter().enumerate() {
        // Skip last as we only compare to those after.
        if current_pos == no_of_claims - 1 {
            break;
        }

        let sub_vector = &claims[(current_pos + 1)..];

        //        println!("XXX: sub_vector_size: {} ", sub_vector.len());

        let claim_rec = claim.get_rect();
        // println!(
        //     "XXXXXXXXXX: claim_rec rect: {},{} {},{}",
        //     claim_rec.x, claim_rec.y, claim_rec.x2, claim_rec.y2
        // );

        for (_cmp_pos, cmp_claim) in sub_vector.iter().enumerate() {
            //             println!("XXX: cmp pos_ : {} ", _cmp_pos);
            let cmp_claim_rec = cmp_claim.get_rect();
            // println!(
            //     "XXXXXXXXXXX: cmp_claim_rec: {},{} {},{}",
            //     cmp_claim_rec.x, cmp_claim_rec.y, cmp_claim_rec.x2, cmp_claim_rec.y2
            // );

            if let Some(rect) = claim_rec.get_intersect_rect(&cmp_claim_rec) {
                // println!(
                //     "XXXXXXXXXXXX: cmp_claim_rec: {},{} {},{}",
                //     rect.x, rect.y, rect.x2, rect.y2
                // );

                overclaimed_rects.push(rect);
            }
        }
    }

    type YSet = HashSet<i32>;
    // Ok, don't care for a full sweep implementation, see just add to a matrix and count...
    // Might be slooow for larger inputs. Might give time for reflection and contemplation
    let mut overclaimed_square_inches: HashMap<i32, YSet> = HashMap::new();

    let mut new_insert: i32 = 0;

    for rect in overclaimed_rects {
        // println!(
        //     "XXXXXXXXXX: oc rect: {},{} {},{}",
        //     rect.x, rect.y, rect.x2, rect.y2
        // );

        for x in rect.x..=rect.x2 {
            overclaimed_square_inches.entry(x).or_insert_with(YSet::new);
            let y_map_res: Option<&mut YSet> = overclaimed_square_inches.get_mut(&x);
            let y_map = y_map_res.unwrap();

            //           println!("XXXXXXXXXX: check y    {} rect.y..rect.y2 {}", rect.y, rect.y2 );
            for y in rect.y..=rect.y2 {
                //                println!("XXXXXXXXXX: fo loop y    {} ", y );
                if y_map.insert(y) {
                    new_insert += 1;
                }
            }
        }
    }
    new_insert
}

fn solve_second_file(input: &str) -> String {
    let v = util::aoc2018_row_file_to_vector(input);
    solve_second(v)
}

// Ok, only one was expected, coded to find first. Brute fore and use the cpu. Love the heat.
fn solve_second(str_vector: Vec<String>) -> String {
    let claims: Vec<Claim> = get_claims(str_vector);

    for (current_pos, claim) in claims.iter().enumerate() {
        // Skip last as we only compare to those after.
        let claim_rec = claim.get_rect();

        let mut found_overlap: bool = false;
        for (_cmp_pos, cmp_claim) in claims.iter().enumerate() {
            let cmp_claim_rec = cmp_claim.get_rect();
            if current_pos != _cmp_pos && claim_rec.get_intersect_rect(&cmp_claim_rec).is_some() {
                found_overlap = true;
                break;
            }
        }

        if !found_overlap {
            return claim.id.to_string();
        }
    }

    "Fail".to_string()
}

fn get_claims(str_vector: Vec<String>) -> Vec<Claim> {
    let mut claims: Vec<Claim> = Vec::new();
    for line in str_vector {
        let v: Vec<&str> = line
            .split(|c| c == '#' || c == '@' || c == ',' || c == ':' || c == 'x')
            .filter(|k| !k.is_empty())
            .collect();

        // &['#', '@', ',', ':', 'x']  let tokens: Vec<_> = line.split(&['#', '@', ',', ':', 'x']).).collect();
        let claim = Claim {
            id: String::from(v[0].trim()),
            offset_x: v[1].trim().parse().unwrap(),
            offset_y: v[2].trim().parse().unwrap(),
            width: v[3].trim().parse().unwrap(),
            height: v[4].trim().parse().unwrap(),
        };
        claims.push(claim);
    }
    claims
}

struct Claim {
    #[allow(dead_code)]
    id: String,
    offset_x: i32,
    offset_y: i32,
    width: i32,
    height: i32,
}

struct Rect {
    x: i32,
    y: i32,
    x2: i32,
    y2: i32,
}

impl Claim {
    pub fn get_rect(self: &Self) -> Rect {
        Rect {
            x: self.offset_x,
            y: self.offset_y,
            x2: self.offset_x + self.width - 1,
            y2: self.offset_y + self.height - 1,
        }
    }
}

impl Rect {
    // note, requires normalized rects
    pub fn get_intersect_rect(self: &Self, r2: &Rect) -> Option<Rect> {
        let irect = Rect {
            x: self.x.max(r2.x),
            y: self.y.max(r2.y),
            x2: self.x2.min(r2.x2),
            y2: self.y2.min(r2.y2),
        };

        // println!(
        //     "XXXXXXXXXXX: test int: {},{} {},{}",
        //     irect.x, irect.y, irect.x2, irect.y2
        // );

        if irect.x > irect.x2 || irect.y > irect.y2 {
            // println!(
            //     "XXXXXXXXXXX: NONE test  irect.x > irect.x2 || irect.y > irect.y2 {} {}",
            //     irect.x > irect.x2,
            //     irect.y > irect.y2
            // );
            None
        } else {
            Some(irect)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cases_first() {
        // provided examples
        let answer = solve_first_file("test-day-3.txt");
        assert_eq!(answer, 4);

        // my test
        let answer = solve_first_file("test-day-3-2.txt");
        assert_eq!(answer, 1);
    }

    #[test]
    fn test_day_3_second() {
        // provided examples
        let answer = solve_second_file("test-day-3.txt");
        assert_eq!(answer, "3");

        // my test
        let answer = solve_second_file("test-day-3-2.txt");
        assert_eq!(answer, "Fail");
    }
}
