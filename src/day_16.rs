use regex::Regex;
use std::collections::HashMap;
use std::collections::HashSet;

use crate::util;

pub fn solve() {
    let input_file = "input-day-16.txt";
    println!("Day 16 answers");
    print!(" first puzzle: ");
    let answer = solve_first_file(input_file);
    println!("{}", answer);
    print!(" second puzzle: ");
    let answer = solve_second_file(input_file);
    println!("{}", answer);
}

fn solve_first_file(input: &str) -> usize {
    let v = util::aoc2018_row_file_to_vector(input);
    solve_first(&v)
}

fn solve_second_file(input: &str) -> u32 {
    let v = util::aoc2018_row_file_to_vector(input);
    solve_second(&v)
}

fn solve_first(str_vector: &[String]) -> usize {
    let (samples, _program) = shared_puzzle_start(str_vector);

    let check_level = 3;

    let mut no_of_matches = 0;
    for sample in &samples {
        let mut no_of_possible_opcodes_for_sample = 0;
        for op_code in OpCode::op_codes_as_vector() {
            //            println!("XXX : Testing opcode {:?} ", op_code);
            let res_register = op_code.execute(
                sample.instruction.input_a,
                sample.instruction.input_b,
                sample.instruction.output,
                &sample.pre_state,
            );
            if res_register == sample.post_state {
                no_of_possible_opcodes_for_sample += 1;
                //                println!("XXX : Match++ ");
                if no_of_possible_opcodes_for_sample == check_level {
                    break; // test no more for this sample
                }
            }
        }
        if no_of_possible_opcodes_for_sample >= check_level {
            no_of_matches += 1;
        }
    }

    no_of_matches
}

fn solve_second(str_vector: &[String]) -> u32 {
    let (samples, program) = shared_puzzle_start(str_vector);
    //    println!("XXX: samples: {:?}", samples);
    //    println!("XXX: program: {:?}", program);

    let op_codes = OpCode::op_codes_as_vector();

    let mut op_to_num: HashMap<&OpCode, u32> = HashMap::new();
    let mut num_to_op: HashMap<u32, OpCode> = HashMap::new();
    let mut found_op_code_numbers: HashSet<u32> = HashSet::new();

    loop {
        let mut found_any_opcode_this_loop = false;

        for op_code in &op_codes {
            if !op_to_num.contains_key(&op_code) {
                let mut possible_opcode_number: Option<u32> = None;
                for sample in &samples {
                    if !found_op_code_numbers.contains(&sample.instruction.opcode) {
                        let res_register = op_code.execute(
                            sample.instruction.input_a,
                            sample.instruction.input_b,
                            sample.instruction.output,
                            &sample.pre_state,
                        );
                        if res_register == sample.post_state {
                            match possible_opcode_number {
                                Some(possible_found_earlier) => {
                                    if possible_found_earlier != sample.instruction.opcode {
                                        // println!(
                                        //     "XXX Multiple possible no for opCode {:?}",
                                        //     op_code
                                        // );
                                        possible_opcode_number = None;
                                        break;
                                    }
                                }
                                None => {
                                    // println!(
                                    //     "XXX Possible no for opCode: {:?} {}",
                                    //     op_code, sample.instruction.opcode
                                    // );
                                    possible_opcode_number = Some(sample.instruction.opcode);
                                }
                            }
                        }
                    } else {
                        // println!(
                        //     "Skipping sample for already found opcode no {} ",
                        //     sample.instruction.opcode
                        // );
                    }
                }

                if let Some(match_opcode_number) = possible_opcode_number {
                    found_any_opcode_this_loop = true;
                    op_to_num.insert(&op_code, match_opcode_number);
                    num_to_op.insert(match_opcode_number, op_code.clone());
                    found_op_code_numbers.insert(match_opcode_number);
                } else {
                    // println!(
                    //     "XXX Insufficient data - no match at all for op_code {:?}",
                    //     op_code
                    // );
                }
            } else {
                // println!("Already found op_code: {:?} ", op_code);
            }
        }

        if found_op_code_numbers.len() == op_codes.len() {
            // break if all fine
            // println!(
            //     "XXX Founds alla op_code_ numbers: {}",
            //     found_op_code_numbers.len()
            // );
            break;
        }

        if !found_any_opcode_this_loop {
            // panic if not
            panic!("Can't fully resolve puzzle, no more opcodes matched");
        }
    }

    // for (op, no) in op_to_num {
    //     println!("XXX OpCode value: {:?}={}", op, no);
    // }

    let mut regs = Registers {
        r_0: 0,
        r_1: 0,
        r_2: 0,
        r_3: 0,
    };

    for (_i, instruction) in program.iter().enumerate() {
        let op = &num_to_op[&instruction.opcode];

        // println!(
        //     "Row: {} Op: {:?}  Ins: {:?}  Reg: {:?}",
        //     _i, op, instruction, regs
        // );

        regs = op.execute(
            instruction.input_a,
            instruction.input_b,
            instruction.output,
            &regs,
        );
    }

    regs.r_0
}

enum ParseState {
    OpenForRequests,
    ExpectOpCode,
    ExpectResult,
}

fn shared_puzzle_start(str_vector: &[String]) -> (Vec<Sample>, Vec<Instruction>) {
    let re_before =
        Regex::new(r"^Before:\s*.[\[](\d+),\s*(\d+),\s*(\d+),\s*(\d+)\s*[\]]\s*$").unwrap();
    let re_after =
        Regex::new(r"^After:\s*.[\[](\d+),\s*(\d+),\s*(\d+),\s*(\d+)\s*[\]]\s*$").unwrap();

    let mut samples: Vec<Sample> = vec![];
    let mut program: Vec<Instruction> = vec![];

    let mut parse_state = ParseState::OpenForRequests;

    let mut tmp_pre_state: Option<Registers> = None;
    let mut tmp_instruction: Option<Instruction> = None;

    for (_index, line) in str_vector.iter().enumerate() {
        //        println!("XXX: PArsing line: {:?}", line);
        if !line.trim().is_empty() {
            match parse_state {
                ParseState::OpenForRequests => {
                    let caps = re_before.captures(&line);
                    if let Some(caps) = caps {
                        tmp_pre_state = Some(Registers {
                            r_0: caps[1].parse().expect("Not an u32"),
                            r_1: caps[2].parse().expect("Not an u32"),
                            r_2: caps[3].parse().expect("Not an u32"),
                            r_3: caps[4].parse().expect("Not an u32"),
                        });
                        //                      println!("XXX: tmp_pre_state: {:?}", tmp_pre_state);
                        parse_state = ParseState::ExpectOpCode;
                    } else {
                        let instruction = parse_instruction(line);
                        //                    println!("XXX: add instruction: {:?}", instruction);
                        program.push(instruction);
                    }
                }
                ParseState::ExpectOpCode => {
                    tmp_instruction = Some(parse_instruction(line));
                    //              println!("XXX: add instruction: {:?}", tmp_instruction);
                    parse_state = ParseState::ExpectResult;
                }
                ParseState::ExpectResult => {
                    //            println!("XXX: Parsing result line: {:?}", line);
                    let caps = re_after.captures(&line);
                    if let Some(caps) = caps {
                        let post_state = Registers {
                            r_0: caps[1].parse().expect("Not an u32"),
                            r_1: caps[2].parse().expect("Not an u32"),
                            r_2: caps[3].parse().expect("Not an u32"),
                            r_3: caps[4].parse().expect("Not an u32"),
                        };

                        //                println!("XXX: post_state: {:?}", post_state);
                        let sample = Sample {
                            pre_state: tmp_pre_state.unwrap(),
                            instruction: tmp_instruction.unwrap(),
                            post_state,
                        };
                        tmp_pre_state = None;
                        tmp_instruction = None;
                        samples.push(sample);
                    } else {
                        panic!(
                            "Error parsing input, expected post state (\"After:\") at line {}",
                            line
                        );
                    }
                    parse_state = ParseState::OpenForRequests;
                }
            }
        } else {
            //         println!("XXX: Skip empty line");
        }
    }

    if let ParseState::OpenForRequests = parse_state {
    } else {
        panic!("Error parsing input");
    }

    (samples, program)
}

fn parse_instruction(line: &str) -> Instruction {
    let re_instruction = Regex::new(r"^(\d+)\s*(\d+)\s*(\d+)\s*(\d+)\s*$").unwrap();
    let caps = re_instruction.captures(line);
    if let Some(caps) = caps {
        Instruction {
            opcode: caps[1].parse().expect("Not an u32"),
            input_a: caps[2].parse().expect("Not an u32"),
            input_b: caps[3].parse().expect("Not an u32"),
            output: caps[4].parse().expect("Not an u32"),
        }
    } else {
        panic!("Error parsing input, expected instruction at line {}", line);
    }
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
enum OpCode {
    Addr,
    Addi,

    Mulr,
    Muli,

    Banr,
    Bani,

    Borr,
    Bori,

    Setr,
    Seti,

    Gtir,
    Gtri,
    Gtrr,

    Eqir,
    Eqri,
    Eqrr,
}

impl OpCode {
    fn op_codes_as_vector() -> Vec<OpCode> {
        vec![
            OpCode::Addr,
            OpCode::Addi,
            OpCode::Mulr,
            OpCode::Muli,
            OpCode::Banr,
            OpCode::Bani,
            OpCode::Borr,
            OpCode::Bori,
            OpCode::Setr,
            OpCode::Seti,
            OpCode::Gtir,
            OpCode::Gtri,
            OpCode::Gtrr,
            OpCode::Eqir,
            OpCode::Eqri,
            OpCode::Eqrr,
        ]
    }

    fn execute(
        self,
        input_a: u32,
        input_b: u32,
        output: u32,
        pre_register: &Registers,
    ) -> Registers {
        let mut res_register = Registers { ..*pre_register };

        match self {
            OpCode::Addr => {
                let val = pre_register.val_at(input_a) + pre_register.val_at(input_b);
                res_register.set_val_to(val, output);
            }

            OpCode::Addi => {
                let val = pre_register.val_at(input_a) + input_b;
                res_register.set_val_to(val, output);
            }

            OpCode::Mulr => {
                let val = pre_register.val_at(input_a) * pre_register.val_at(input_b);
                res_register.set_val_to(val, output);
            }
            OpCode::Muli => {
                let val = pre_register.val_at(input_a) * input_b;
                res_register.set_val_to(val, output);
            }

            OpCode::Banr => {
                let val = pre_register.val_at(input_a) & pre_register.val_at(input_b);
                res_register.set_val_to(val, output);
            }
            OpCode::Bani => {
                let val = pre_register.val_at(input_a) & input_b;
                res_register.set_val_to(val, output);
            }

            OpCode::Borr => {
                let val = pre_register.val_at(input_a) | pre_register.val_at(input_b);
                res_register.set_val_to(val, output);
            }
            OpCode::Bori => {
                let val = pre_register.val_at(input_a) | input_b;
                res_register.set_val_to(val, output);
            }

            OpCode::Setr => {
                let val = pre_register.val_at(input_a);
                res_register.set_val_to(val, output);
            }
            OpCode::Seti => {
                let val = input_a;
                res_register.set_val_to(val, output);
            }

            OpCode::Gtir => {
                let cmp_res = input_a > pre_register.val_at(input_b);
                if cmp_res {
                    res_register.set_val_to(1, output);
                } else {
                    res_register.set_val_to(0, output);
                }
            }
            OpCode::Gtri => {
                let cmp_res = pre_register.val_at(input_a) > input_b;
                if cmp_res {
                    res_register.set_val_to(1, output);
                } else {
                    res_register.set_val_to(0, output);
                }
            }
            OpCode::Gtrr => {
                let cmp_res = pre_register.val_at(input_a) > pre_register.val_at(input_b);
                if cmp_res {
                    res_register.set_val_to(1, output);
                } else {
                    res_register.set_val_to(0, output);
                }
            }

            OpCode::Eqir => {
                let cmp_res = input_a == pre_register.val_at(input_b);
                if cmp_res {
                    res_register.set_val_to(1, output);
                } else {
                    res_register.set_val_to(0, output);
                }
            }
            OpCode::Eqri => {
                let cmp_res = pre_register.val_at(input_a) == input_b;
                if cmp_res {
                    res_register.set_val_to(1, output);
                } else {
                    res_register.set_val_to(0, output);
                }
            }
            OpCode::Eqrr => {
                let cmp_res = pre_register.val_at(input_a) == pre_register.val_at(input_b);
                if cmp_res {
                    res_register.set_val_to(1, output);
                } else {
                    res_register.set_val_to(0, output);
                }
            }
        }

        res_register
    }
}

#[derive(Debug, PartialEq)]
struct Registers {
    r_0: u32,
    r_1: u32,
    r_2: u32,
    r_3: u32,
}

impl Registers {
    fn set_val_to(&mut self, value: u32, position: u32) {
        match position {
            0 => {
                self.r_0 = value;
            }
            1 => {
                self.r_1 = value;
            }
            2 => {
                self.r_2 = value;
            }
            3 => {
                self.r_3 = value;
            }
            _ => {
                panic!(
                    "Guru meditation, can't insert {} at (r_){}",
                    value, position
                );
            }
        }
    }

    fn val_at(&self, position: u32) -> u32 {
        match position {
            0 => self.r_0,
            1 => self.r_1,
            2 => self.r_2,
            3 => self.r_3,
            _ => {
                panic!("Guru meditation, can't read from (r_){}. Duh.", position);
            }
        }
    }
}

#[derive(Debug)]
struct Instruction {
    opcode: u32,
    input_a: u32,
    input_b: u32,
    output: u32,
}

#[derive(Debug)]
struct Sample {
    pre_state: Registers,
    instruction: Instruction,
    post_state: Registers,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_day_16_first() {
        // provided examples
        let input = &[
            String::from("Before: [3, 2, 1, 1]"),
            String::from("9 2 1 2"),
            String::from("After:  [3, 2, 2, 1]"),
            String::from("0 0 0 0"),
            String::from("0 0 0 1"),
            String::from("0 0 0 2"),
        ];

        let answer = solve_first(input);
        assert_eq!(answer, 1);
    }
}
