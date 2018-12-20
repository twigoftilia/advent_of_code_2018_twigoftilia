use regex::Regex;
use std::str::FromStr as StrFromStr;
use strum_macros::EnumString;
use crate::util;

pub fn solve() {
    let input_file = "input-day-19.txt";
    println!("Day 19 answers");
    print!(" first puzzle: ");
    let (answer_p1, _answer_p2) = solve_both_file(input_file);
    println!("{}", answer_p1);
    //    print!(" second puzzle: ");
    //    println!("{}", answer_p2);
}

fn solve_both_file(input: &str) -> (usize, usize) {
    let v = util::aoc2018_row_file_to_vector(input);
    solve_both(&v)
}

fn solve_both(str_vector: &[String]) -> (usize, usize) {
    let (ip_register, program) = shared_puzzle_start(&str_vector);

    let mut registers: Registers = vec![0; 6];
    run_program(&program, ip_register, &mut registers);

    let mut registers2: Registers = vec![0; 6];
    registers2[0] = 1;
    run_program(&program, ip_register, &mut registers2);
    (registers[0], registers2[0])
}

fn run_program(program: &[Instruction], ip_register: usize, registers: &mut Vec<usize>) {
    let mut ip: usize = 0;
    //   println!("XXX: IP: {:?} IP reg: {}", ip, ip_register);

    loop {
        if ip >= program.len() {
            println!("XXX: ip: {} -> registers: {:?}", ip, registers);
            break;
        }

        let ins = &program[ip];
        let op_code = OpCode::from_str(&ins.opcode).expect("Not a valid OpCode");

        // print!("XXX: PRE  OP: ip: {} registers  instr ip refgisters {:?}     {:?}   ", ip, registers, ins);
        ip = op_code.execute(
            ins.input_a,
            ins.input_b,
            ins.output,
            registers,
            ip,
            ip_register,
        );
        // println!("XXX: ip: {} -> registers: {:?}", ip, registers);
    }
}

fn shared_puzzle_start(str_vector: &[String]) -> (usize, Vec<Instruction>) {
    let re = Regex::new(r"^(\w{4})\s*(\d+)\s*(\d+)\s*(\d+)\s*$").unwrap();
    let re_ip_register = Regex::new(r"^#ip\s*(\d)\s*$").unwrap();

    let mut program: Vec<Instruction> = vec![];
    let mut ip_reg: Option<usize> = None;

    for (_index, line) in str_vector.iter().enumerate() {
        //        println!("XXX: PArsing line: {:?}", line);
        if !line.trim().is_empty() {
            let caps = re.captures(&line);
            if let Some(caps) = caps {
                let ins_str = caps[1].to_owned();
                //  let o = OpCode::from_str(&ins_str).expect("Not a valid OpCode");
                let i = Instruction {
                    opcode: ins_str,
                    input_a: caps[2].parse().expect("Not an usize"),
                    input_b: caps[3].parse().expect("Not an usize"),
                    output: caps[4].parse().expect("Not an usize"),
                };
                program.push(i);
            } else {
                //              println!("XXX: PArsing line: {:?}", line);
                let caps = re_ip_register.captures(&line);
                if let Some(caps) = caps {
                    ip_reg = Some(caps[1].parse().expect("Not an usize"));
                }
            }
        }
    }
    (ip_reg.unwrap(), program)
}

#[derive(Debug, EnumString)]
#[strum(serialize_all = "snake_case")]
enum OpCode {
    Addr = 6,
    Addi = 2,

    Mulr = 12,
    Muli = 13,

    Banr = 15,
    Bani = 3,

    Borr = 8,
    Bori = 1,

    Setr = 10,
    Seti = 4,

    Gtir = 9,
    Gtri = 7,
    Gtrr = 14,

    Eqir = 11,
    Eqri = 0,
    Eqrr = 5,
}

// struct Registers (Vec<usize>);
type Registers = Vec<usize>;

impl OpCode {
    fn execute(
        self,
        a: usize,
        b: usize,
        o: usize,
        register: &mut Registers,
        ip: usize,
        ip_register: usize,
    ) -> usize {
        register[ip_register] = ip;
        match self {
            OpCode::Addr => {
                register[o] = register[a] + register[b];
            }
            OpCode::Addi => {
                register[o] = register[a] + b;
            }
            OpCode::Mulr => {
                register[o] = register[a] * register[b];
            }
            OpCode::Muli => {
                register[o] = register[a] * b;
            }
            OpCode::Banr => {
                register[o] = register[a] & register[b];
            }
            OpCode::Bani => {
                register[o] = register[a] & b;
            }
            OpCode::Borr => {
                register[o] = register[a] | register[b];
            }
            OpCode::Bori => {
                register[o] = register[a] | b;
            }
            OpCode::Setr => {
                register[o] = register[a];
            }
            OpCode::Seti => {
                register[o] = a;
            }
            OpCode::Gtir => {
                if a > register[b] {
                    register[o] = 1;
                } else {
                    register[o] = 0;
                }
            }
            OpCode::Gtri => {
                if register[a] > b {
                    register[o] = 1;
                } else {
                    register[o] = 0;
                }
            }
            OpCode::Gtrr => {
                if register[a] > register[b] {
                    register[o] = 1;
                } else {
                    register[o] = 0;
                }
            }
            OpCode::Eqir => {
                if a == register[b] {
                    register[o] = 1;
                } else {
                    register[o] = 0;
                }
            }
            OpCode::Eqri => {
                let cmp_res = register[a] == b;
                if cmp_res {
                    register[o] = 1;
                } else {
                    register[o] = 0;
                }
            }
            OpCode::Eqrr => {
                let cmp_res = register[a] == register[b];
                if cmp_res {
                    register[o] = 1;
                } else {
                    register[o] = 0;
                }
            }
        }

        register[ip_register] += 1;
        register[ip_register]
    }
}

#[derive(Debug)]
struct Instruction {
    opcode: String,
    input_a: usize,
    input_b: usize,
    output: usize,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_day_19() {
        // provided examples
        let input = &[
            String::from("#ip 0"),
            String::from("seti 5 0 1"),
            String::from("seti 6 0 2"),
            String::from("addi 0 1 0"),
            String::from("addr 1 2 3"),
            String::from("setr 1 0 0"),
            String::from("seti 8 0 4"),
            String::from("seti 9 0 5"),
        ];

        let (a1, _a2) = solve_both(input);
        assert_eq!(a1, 7);
        //        assert_eq!(a2, 7);
    }
}
