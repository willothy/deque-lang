use std::{
    collections::{HashMap, VecDeque},
    error::Error,
};

struct VM {
    ip: usize,
    program: Vec<Instruction>,
    labels: HashMap<String, usize>,
    data: VecDeque<i64>,
}

impl VM {
    pub fn new() -> Self {
        Self {
            ip: 0,
            program: Vec::new(),
            labels: HashMap::new(),
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
            .enumerate()
            .map(|(addr, inst)| {
                // load label addresses
                if inst.ends_with(":") {
                    let label = &inst[..inst.len() - 1];
                    self.labels.insert(label.to_ascii_lowercase(), addr);
                }
                inst
            })
            .filter_map(|inst| {
                // convert text to instructions
                if inst.starts_with("!") {
                    Some(Instruction {
                        direction: Direction::Right,
                        op: (&inst[1..]).to_string(),
                    })
                } else if inst.ends_with("!") {
                    Some(Instruction {
                        direction: Direction::Right,
                        op: (&inst[..inst.len() - 1]).to_string(),
                    })
                } else if inst.ends_with(":") {
                    Some(Instruction {
                        op: "label".to_owned(),
                        direction: Direction::Left,
                    })
                } else {
                    None
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
                "jmp" => {
                    self.ip = self.pop(&dir) as usize;
                    continue;
                }
                "jmpif" => {
                    let addr = self.pop(&dir);
                    let cond = self.pop(&dir);
                    if cond != 0 {
                        self.ip = addr as usize;
                        continue;
                    }
                }
                ">" => {
                    let a = self.pop(&dir);
                    let b = self.pop(&dir);
                    self.push(&dir, (a > b) as i64);
                }
                "<" => {
                    let a = self.pop(&dir);
                    let b = self.pop(&dir);
                    self.push(&dir, (a < b) as i64);
                }
                ">=" => {
                    let a = self.pop(&dir);
                    let b = self.pop(&dir);
                    self.push(&dir, (a >= b) as i64);
                }
                "<=" => {
                    let a = self.pop(&dir);
                    let b = self.pop(&dir);
                    self.push(&dir, (a <= b) as i64);
                }
                "dup" => {
                    let temp = self.pop(&dir);
                    self.push(&dir, temp);
                    self.push(&dir, temp);
                }
                "print" => {
                    let val = self.pop(&dir);
                    println!("{}", val);
                }
                "label" => {}
                val => {
                    let val = if let Ok(val) = val.parse::<i64>() {
                        // it's a value
                        val
                    } else {
                        // it's a label reference
                        (*self.labels.get(val).unwrap()) as i64
                    };
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
    let test_program = "
!10
loop: !dup !0 !>= !end !jmpif
    !dup !print
    !1 !sub
!loop !jmp
end:
    ";
    let mut vm = VM::new();
    vm.load_program(test_program.to_owned());
    vm.execute()?;
    Ok(())
}
