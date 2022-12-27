use std::{
    collections::{HashMap, VecDeque},
    io::stdin,
};

struct VM {
    ip: i64,
    program: Vec<Instruction>,
    labels: HashMap<String, i64>,
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
                    self.labels.insert(label.to_ascii_lowercase(), addr as i64);
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

    fn add(&mut self, dir: &Direction) -> Result<(), String> {
        let a = self.pop(dir)?;
        let b = self.pop(dir)?;
        self.push(dir, a + b);
        Ok(())
    }

    fn sub(&mut self, dir: &Direction) -> Result<(), String> {
        let a = self.pop(dir)?;
        let b = self.pop(dir)?;
        self.push(dir, b - a);
        Ok(())
    }

    fn jmp(&mut self, dir: &Direction) -> Result<(), String> {
        self.ip = self.pop(dir)?;
        Ok(())
    }

    fn jmpif(&mut self, dir: &Direction) -> Result<bool, String> {
        let addr = self.pop(dir)?;
        let cond = self.pop(dir)?;
        if cond == 1 {
            self.ip = addr;
            return Ok(true);
        }
        Ok(false)
    }

    fn swap(&mut self, dir: &Direction) -> Result<(), String> {
        let a = self.pop(dir)?;
        let b = self.pop(dir)?;
        self.push(dir, a);
        self.push(dir, b);
        Ok(())
    }

    fn move_(&mut self, dir: &Direction) -> Result<(), String> {
        let a = self.pop(dir)?;
        self.push(&dir.invert(), a);
        Ok(())
    }

    fn over(&mut self, dir: &Direction) -> Result<(), String> {
        let a = self.pop(dir)?;
        let b = self.pop(dir)?;
        self.push(dir, b);
        self.push(dir, a);
        self.push(dir, b);
        Ok(())
    }

    fn drop(&mut self, dir: &Direction) -> Result<(), String> {
        self.pop(dir)?;
        Ok(())
    }

    fn shr(&mut self, dir: &Direction) -> Result<(), String> {
        let a = self.pop(&dir)?;
        let b = self.pop(&dir)?;
        self.push(&dir, b >> a);
        Ok(())
    }

    fn shl(&mut self, dir: &Direction) -> Result<(), String> {
        let a = self.pop(&dir)?;
        let b = self.pop(&dir)?;
        self.push(&dir, b << a);
        Ok(())
    }

    fn eq(&mut self, dir: &Direction) -> Result<(), String> {
        let a = self.pop(&dir)?;
        let b = self.pop(&dir)?;
        self.push(&dir, (a == b) as i64);
        Ok(())
    }

    fn or(&mut self, dir: &Direction) -> Result<(), String> {
        let a = self.pop(&dir)?;
        let b = self.pop(&dir)?;
        self.push(&dir, a | b);
        Ok(())
    }

    fn and(&mut self, dir: &Direction) -> Result<(), String> {
        let a = self.pop(&dir)?;
        let b = self.pop(&dir)?;
        self.push(&dir, a & b);
        Ok(())
    }

    fn xor(&mut self, dir: &Direction) -> Result<(), String> {
        let a = self.pop(&dir)?;
        let b = self.pop(&dir)?;
        self.push(&dir, a ^ b);
        Ok(())
    }

    fn not(&mut self, dir: &Direction) -> Result<(), String> {
        let a = self.pop(&dir)?;
        self.push(&dir, !a);
        Ok(())
    }

    fn greater(&mut self, dir: &Direction) -> Result<(), String> {
        let a = self.pop(&dir)?;
        let b = self.pop(&dir)?;
        self.push(&dir, (a > b) as i64);
        Ok(())
    }

    fn less(&mut self, dir: &Direction) -> Result<(), String> {
        let a = self.pop(&dir)?;
        let b = self.pop(&dir)?;
        self.push(&dir, (a < b) as i64);
        Ok(())
    }

    fn greater_eq(&mut self, dir: &Direction) -> Result<(), String> {
        let a = self.pop(&dir)?;
        let b = self.pop(&dir)?;
        self.push(&dir, (a >= b) as i64);
        Ok(())
    }

    fn less_eq(&mut self, dir: &Direction) -> Result<(), String> {
        let a = self.pop(&dir)?;
        let b = self.pop(&dir)?;
        self.push(&dir, (a <= b) as i64);
        Ok(())
    }

    fn dup(&mut self, dir: &Direction) -> Result<(), String> {
        let a = self.pop(&dir)?;
        self.push(&dir, a);
        self.push(&dir, a);
        Ok(())
    }

    fn print(&mut self, dir: &Direction) -> Result<(), String> {
        let a = self.pop(&dir)?;
        println!("{}", a);
        Ok(())
    }

    fn printc(&mut self, dir: &Direction) -> Result<(), String> {
        let a = self.pop(&dir)?;
        println!("{}", a as u8 as char);
        Ok(())
    }

    fn read(&mut self, dir: &Direction) -> Result<(), String> {
        let mut input = String::new();
        stdin().read_line(&mut input).map_err(|e| e.to_string())?;
        let a = input.trim().parse::<i64>().map_err(|e| e.to_string())?;
        self.push(&dir, a);
        Ok(())
    }

    fn readc(&mut self, dir: &Direction) -> Result<(), String> {
        let mut input = String::new();
        stdin().read_line(&mut input).map_err(|e| e.to_string())?;
        let a = input.trim().chars().next().unwrap_or(' ') as i64;
        self.push(&dir, a);
        Ok(())
    }

    fn trace(&mut self) {
        let dots = self
            .data
            .iter()
            .map(|x| if *x == 1 { '*' } else { ' ' })
            .collect::<String>();
        println!("{}", dots);
    }

    pub fn execute(&mut self) -> Result<(), String> {
        let program_len = self.program.len() as i64;
        while self.ip < program_len {
            let (op, dir) = {
                let instruction = &self.program[self.ip as usize];
                let op = instruction.op.as_str();
                let dir = instruction.direction.clone();
                (op, dir)
            };
            match op {
                "add" => self.add(&dir)?,
                "sub" => self.sub(&dir)?,
                "swap" => self.swap(&dir)?,
                "move" => self.move_(&dir)?,
                "over" => self.over(&dir)?,
                "drop" => self.drop(&dir)?,
                "shr" => self.shr(&dir)?,
                "shl" => self.shl(&dir)?,
                "eq" => self.eq(&dir)?,
                "or" => self.or(&dir)?,
                "and" => self.and(&dir)?,
                "xor" => self.xor(&dir)?,
                "not" => self.not(&dir)?,
                ">" => self.greater(&dir)?,
                "<" => self.less(&dir)?,
                ">=" => self.greater_eq(&dir)?,
                "<=" => self.less_eq(&dir)?,
                "dup" => self.dup(&dir)?,
                "print" => self.print(&dir)?,
                "printc" => self.printc(&dir)?,
                "read" => self.read(&dir)?,
                "readc" => self.readc(&dir)?,
                "trace" => self.trace(),
                "jmp" => {
                    self.jmp(&dir)?;
                    continue;
                }
                "jmpif" => {
                    if self.jmpif(&dir)? {
                        continue;
                    }
                }
                "exit" => {
                    let code = self.pop(&dir)?;
                    if code != 0 {
                        return Err(format!("Exit code {}", code));
                    }
                    return Ok(());
                }
                "label" => {}
                val => {
                    let val = if let Ok(val) = val.parse::<i64>() {
                        // it's a value
                        val
                    } else {
                        // it's a label reference
                        *self
                            .labels
                            .get(val)
                            .ok_or(format!("Label {} does not exist.", val))?
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

impl Direction {
    fn invert(&self) -> Direction {
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
