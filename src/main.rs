use std::{collections::VecDeque, error::Error};

struct VM {
    ip: usize,
    program: Vec<Instruction>,
    data: VecDeque<i64>,
}

impl VM {
    pub fn new() -> Self {
        Self {
            ip: 0,
            program: Vec::new(),
            data: VecDeque::new(),
        }
    }

    fn pop(&mut self, dir: &Direction) -> i64 {
        match dir {
            Direction::Left => self.data.pop_front().unwrap(),
            Direction::Right => self.data.pop_back().unwrap(),
        }
    }

    fn push(&mut self, dir: &Direction, val: i64) {
        match dir {
            Direction::Left => self.data.push_front(val),
            Direction::Right => self.data.push_back(val),
        }
    }

    pub fn load_program(&mut self, program: String) {
        let instructions = program.split_whitespace();
        let instructions: Vec<Instruction> = instructions
            .map(|inst| {
                let (direction, op) = if inst.starts_with("!") {
                    (Direction::Left, &inst[1..])
                } else {
                    (Direction::Right, &inst[..inst.len() - 1])
                };
                Instruction {
                    direction,
                    op: op.to_string(),
                }
            })
            .collect();
        self.program = instructions;
    }

    pub fn execute(&mut self) -> Result<(), String> {
        let program_len = self.program.len();
        while self.ip < program_len {
            let (op, dir) = {
                let instruction = &self.program[self.ip];
                let op = instruction.op.as_str();
                let dir = instruction.direction.clone();
                (op, dir)
            };
            match op {
                "add" => {
                    let a = self.pop(&dir);
                    let b = self.pop(&dir);
                    self.push(&dir, b + a);
                }
                "sub" => {
                    let a = self.pop(&dir);
                    let b = self.pop(&dir);
                    self.push(&dir, b - a);
                }
                "print" => {
                    let val = self.pop(&dir);
                    println!("{}", val);
                    self.push(&dir, val);
                }
                inst => {
                    let val = inst
                        .parse::<i64>()
                        .map_err(|_| "Could not parse integer".to_owned())?;
                    self.push(&dir, val);
                }
            }
            self.ip += 1;
        }
        Ok(())
    }
}

#[derive(Debug, Clone)]
struct Instruction {
    op: String,
    direction: Direction,
}

#[derive(Debug, Clone)]
enum Direction {
    Left,
    Right,
}

fn main() -> Result<(), Box<dyn Error>> {
    let test_program = "3! !5 !2 sub! !add !print";
    let mut vm = VM::new();
    vm.load_program(test_program.to_owned());
    vm.execute()?;
    Ok(())
}
