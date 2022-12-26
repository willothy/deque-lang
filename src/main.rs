use std::collections::{HashMap, VecDeque};

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

    fn pop(&mut self, dir: &Direction) -> Result<i64, String> {
        match dir {
            Direction::Left => self
                .data
                .pop_front()
                .ok_or("Could not pop from front of deque.".into()),
            Direction::Right => self
                .data
                .pop_back()
                .ok_or("Could not pop from back of deque.".into()),
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
                        direction: Direction::Left,
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
                    panic!()
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
                    let a = self.pop(&dir)?;
                    let b = self.pop(&dir)?;
                    self.push(&dir, a + b);
                }
                "sub" => {
                    let a = self.pop(&dir)?;
                    let b = self.pop(&dir)?;
                    self.push(&dir, b - a);
                }
                "jmp" => {
                    self.ip = self.pop(&dir)? as usize;
                    continue;
                }
                "jmpif" => {
                    let addr = self.pop(&dir)?;
                    let cond = self.pop(&dir)?;
                    if cond == 1 {
                        self.ip = addr as usize;
                        continue;
                    }
                }
                "swap" => {
                    let a = self.pop(&dir)?;
                    let b = self.pop(&dir)?;
                    self.push(&dir, a);
                    self.push(&dir, b);
                }
                "move" => {
                    let a = self.pop(&dir)?;
                    self.push(&!dir, a);
                }
                "over" => {
                    let a = self.pop(&dir)?;
                    let b = self.pop(&dir)?;
                    self.push(&dir, b);
                    self.push(&dir, a);
                    self.push(&dir, b);
                }
                "drop" => {
                    self.pop(&dir)?;
                }
                "shr" => {
                    let a = self.pop(&dir)?;
                    let b = self.pop(&dir)?;
                    self.push(&dir, b >> a);
                }
                "shl" => {
                    let a = self.pop(&dir)?;
                    let b = self.pop(&dir)?;
                    self.push(&dir, b << a);
                }
                "eq" => {
                    let a = self.pop(&dir)?;
                    let b = self.pop(&dir)?;
                    self.push(&dir, (a == b) as i64);
                }
                "or" => {
                    let a = self.pop(&dir)?;
                    let b = self.pop(&dir)?;
                    self.push(&dir, a | b);
                }
                "and" => {
                    let a = self.pop(&dir)?;
                    let b = self.pop(&dir)?;
                    self.push(&dir, a & b);
                }
                ">" => {
                    let a = self.pop(&dir)?;
                    let b = self.pop(&dir)?;
                    self.push(&dir, (a > b) as i64);
                }
                "<" => {
                    let a = self.pop(&dir)?;
                    let b = self.pop(&dir)?;
                    self.push(&dir, (a < b) as i64);
                }
                ">=" => {
                    let a = self.pop(&dir)?;
                    let b = self.pop(&dir)?;
                    self.push(&dir, (a >= b) as i64);
                }
                "<=" => {
                    let a = self.pop(&dir)?;
                    let b = self.pop(&dir)?;
                    self.push(&dir, (a <= b) as i64);
                }
                "dup" => {
                    let temp = self.pop(&dir)?;
                    self.push(&dir, temp);
                    self.push(&dir, temp);
                }
                "print" => {
                    let val = self.pop(&dir)?;
                    println!("{}", val);
                }
                "exit" => {
                    let code = self.pop(&dir)?;
                    if code != 0 {
                        return Err(format!("Exit code {}", code));
                    }
                    return Ok(());
                }
                "trace" => {
                    let dots = self
                        .data
                        .iter()
                        .map(|x| if *x == 1 { '*' } else { ' ' })
                        .collect::<String>();
                    println!("{}", dots);
                }
                "label" => {}
                val => {
                    let val = if let Ok(val) = val.parse::<i64>() {
                        // it's a value
                        val
                    } else {
                        // it's a label reference
                        (*self
                            .labels
                            .get(val)
                            .ok_or(format!("Label {} does not exist.", val))?)
                            as i64
                    };
                    self.push(&dir, val);
                }
            }
            if DEBUG {
                println!("data {:?}", self.data);
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

impl std::ops::Not for Direction {
    type Output = Self;

    fn not(self) -> Self::Output {
        match self {
            Direction::Left => Direction::Right,
            Direction::Right => Direction::Left,
        }
    }
}

const DEBUG: bool = false;

fn main() -> Result<(), String> {
    let program = std::fs::read_to_string(
        std::env::args()
            .nth(1)
            .ok_or("File name is required.".to_owned())?,
    )
    .map_err(|_| "Could not read file.".to_owned())?;
    let mut vm = VM::new();
    vm.load_program(program);
    vm.execute()?;
    Ok(())
}
