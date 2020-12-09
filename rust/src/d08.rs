use std::collections::HashSet;
use std::fmt::{Display, Formatter, Result};

pub(crate) struct Solver;

#[derive(Debug)]
struct Instruction {
    opp: String,
    param: i32
}

#[derive(Debug)]
struct Runtime {
    position: usize,
    accumulator: i32,
    run_instructions: HashSet<usize>,
    instructions: Vec<Instruction>
}

fn exec(rt: &mut Runtime) -> &Runtime {
    let instruction = rt.instructions.get(rt.position).unwrap();
    rt.run_instructions.insert(rt.position);
    match instruction.opp.as_str() {
       "nop" => {
           rt.position += 1
       },
       "acc" => {
           rt.accumulator += instruction.param as i32;
           rt.position += 1
       },
       "jmp" => rt.position = (rt.position as i32 + instruction.param) as usize,

        _ => {panic!("Unknown Opp {:?}", instruction)}
    }
    return rt;
}

impl Display for Runtime {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(f, "{}, {}",
               self.position,
               self.accumulator)
    }
}

fn line_to_instruction(line: &str) -> Instruction {
    Instruction {
        opp: line[0..3].to_string(),
        param: line[4..].parse().unwrap()
    }
}

fn init_runtime(input: &str) -> Runtime {
    Runtime {
        position: 0,
        accumulator: 0,
        run_instructions: HashSet::new(),
        instructions: input.lines().map(line_to_instruction).collect()
    }
}

fn in_stop_state(rt: &Runtime) -> bool {
    rt.run_instructions.contains(&rt.position)
        || rt.position >= rt.instructions.len()
}

fn run_until_stop(rt: &mut Runtime) -> &Runtime {
    while !in_stop_state(rt) {
        exec(rt);
    }
    return rt
}

fn swap_instruction(mut rt: Runtime, index: usize) -> Runtime {
    let  mut instruction = rt.instructions.get_mut(index).unwrap();
    if instruction.opp == "nop".to_string() {
        instruction.opp = "jmp".to_string();
    } else if instruction.opp == "jmp".to_string() {
        instruction.opp = "nop".to_string();
    }
    return rt;
}

impl crate::utils::Solver for Solver {
    fn part1(&self, input: &str) -> String {
        let mut rt = init_runtime(input);
        while !rt.run_instructions.contains(&rt.position) {
            exec(&mut rt);
        }
        return rt.accumulator.to_string();
    }
    fn part2(&self, input: &str) -> String {
        let instruction_count = input.lines().count();
        for i in 0..instruction_count {
            let mut rt = swap_instruction(init_runtime(input), i);
            run_until_stop(&mut rt);
            if rt.position == instruction_count {
                return rt.accumulator.to_string()
            }
        }
        return "Not found".to_string()
    }
}