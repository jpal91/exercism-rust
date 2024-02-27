use std::collections::{HashMap, VecDeque};

pub type Value = i32;
pub type Result = std::result::Result<(), Error>;

#[derive(Clone, PartialEq, Eq, Hash, Debug)]
enum Commands {
    Add,
    Sub,
    Mult,
    Div,
    Dup,
    Drop,
    Swap,
    Over,
    Num(Value),
    Custom(Vec<u8>)
}

pub struct Forth {
    stack: Vec<Value>,
    commands: HashMap<Commands, Vec<Commands>>
}

#[derive(Debug, PartialEq, Eq)]
pub enum Error {
    DivisionByZero,
    StackUnderflow,
    UnknownWord,
    InvalidWord,
}

impl Forth {
    pub fn new() -> Forth {
        Self {
            stack: Vec::new(),
            commands: HashMap::new()
        }
    }

    pub fn stack(&self) -> &[Value] {
        &self.stack
    }

    pub fn eval(&mut self, input: &'static str) -> Result {
        let mut input = self.to_commands(input)?;


        while let Some(cmd) = input.pop_front() {

            match cmd {
                Commands::Num(n) => {
                    self.stack.push(n);
                }
                t @ (Commands::Add | Commands::Sub | Commands::Mult | Commands::Div) => {
                    
                    if self.stack.len() < 2 {
                        return Err(Error::StackUnderflow)
                    }

                    let second = self.stack.pop().unwrap();
                    let first = self.stack.pop().unwrap();

                    let val = match t {
                        Commands::Add => first + second,
                        Commands::Sub => first - second,
                        Commands::Mult => first * second,
                        Commands::Div if second != 0 => first / second,
                        Commands::Div => return Err(Error::DivisionByZero),
                        _ => unreachable!()
                    };

                    self.stack.push(val)
                },
                Commands::Dup => {
                    if let Some(num) = self.stack.last() {
                        self.stack.push(num.clone())
                    } else {
                        return Err(Error::StackUnderflow)
                    }
                },
                Commands::Drop => {
                    if let None = self.stack.pop() {
                        return Err(Error::StackUnderflow)
                    }
                },
                Commands::Swap => {
                    let first = if let Some(num) = self.stack.pop() {
                        num
                    } else {
                        return Err(Error::StackUnderflow)
                    };

                    let last = if let Some(num) = self.stack.pop() {
                        num
                    } else {
                        return Err(Error::StackUnderflow)
                    };

                    self.stack.extend([first, last])
                },
                Commands::Over => {
                    let n = self.stack.len();
                    
                    if n < 2 {
                        return Err(Error::StackUnderflow)
                    }

                    let target = self.stack[n - 2].clone();

                    self.stack.push(target)
                },
                Commands::Custom(c) => {
                    let mut cmd = self.commands.get(&Commands::Custom(c)).unwrap().clone();

                    while let Some(c) = cmd.pop() {
                        input.push_front(c)
                    }
                }
            };
        };

        Ok(())
    }

    fn add_new_cmd(&mut self, input: &mut std::str::Split<'static, &str>) -> Result {
        let cmd_name = if let Some(n) = input.next() {
            n.to_lowercase().as_bytes().to_vec()
        } else {
            return Err(Error::InvalidWord)
        };

        if cmd_name[0].is_ascii_digit() {
            return Err(Error::InvalidWord)
        }

        let mut cmds: Vec<Commands> = vec![];

        while let Some(n) = input.next() {
            if n == ";" {
                self.commands.insert(Commands::Custom(cmd_name), cmds);
                return Ok(())
            }
            
            match n.try_into() {
                Ok(cmd) => cmds.push(cmd),
                Err(e) => {
                    if let Some(cmd) = self.contains_cmd(n) {
                        let extras = self.commands.get(&cmd).unwrap();
                        cmds.extend(extras.clone());
                    } else {
                        return Err(e)
                    }
                }
            }
        }

        Err(Error::InvalidWord)
    }

    fn to_commands(&mut self, input: &'static str) -> std::result::Result<VecDeque<Commands>, Error> {
        let mut input = input.split(" ");
        let mut cmds: VecDeque<Commands> = VecDeque::new();

        while let Some(i) = input.next() {
            if i == ":" {
                self.add_new_cmd(&mut input)?;
                continue;
            }

            if let Some(c) = self.contains_cmd(i) {
                cmds.push_back(c);
                continue
            }
            
            let c: Commands = i.try_into()?;
            cmds.push_back(c)
        };

        Ok(cmds)
    }

    fn contains_cmd(&self, cmd: &str) -> Option<Commands> {
        let key = Commands::Custom(cmd.to_lowercase().as_bytes().to_vec());

        if self.commands.contains_key(&key) {
            Some(key)
        } else {
            None
        }
    }
}

impl TryFrom<&'static str> for Commands {
    type Error = Error;

    fn try_from(value: &'static str) -> std::prelude::v1::Result<Self, Self::Error> {
        match value.to_lowercase().as_bytes() {
            b"+" => Ok(Commands::Add),
            b"-" => Ok(Commands::Sub),
            b"*" => Ok(Commands::Mult),
            b"/" => Ok(Commands::Div),
            b"drop" => Ok(Commands::Drop),
            b"over" => Ok(Commands::Over),
            b"swap" => Ok(Commands::Swap),
            b"dup" => Ok(Commands::Dup),
            v if v[0].is_ascii_digit() => {
                let digit: i32 = value.parse().unwrap();
                Ok(Commands::Num(digit))
            }
            _ => Err(Error::UnknownWord)
        }
    }
}