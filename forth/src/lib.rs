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
    Custom(Vec<u8>),
    RefCmd(usize)
}

// My answer to the `alloc-attack` test - two HashMaps
// The `commands` vec holds all commands created and `latest` vec holds the latest version of each
//
// This solves two issues -
// 1. If a command has embedded commands (ie `: bar foo ;`), a call to `bar` will always reference `foo` 
// at the time of `bar`'s creation.
// 2. The embedded command doesn't have to be expanded within (ie all of `foo`'s commands don't have to expand into `bar`).
// This lowers the overall memory usage because in `bar` there's only a key (`Commands::RefCmd(usize)`) to the value in 
// the `commands` HashMap.
pub struct Forth {
    stack: Vec<Value>,
    commands: HashMap<usize, Vec<Commands>>,
    latest: HashMap<Commands, usize>
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
            commands: HashMap::new(),
            latest: HashMap::new()
        }
    }

    pub fn stack(&self) -> &[Value] {
        &self.stack
    }

    pub fn eval(&mut self, input: &'static str) -> Result {
        let mut input = self.get_commands(input)?;

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
                        self.stack.push(*num)
                    } else {
                        return Err(Error::StackUnderflow)
                    }
                },
                Commands::Drop => {
                    if self.stack.pop().is_none() {
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

                    let target = self.stack[n - 2];

                    self.stack.push(target)
                },
                c @ Commands::Custom(_) => {
                    let key = self.latest.get(&c).unwrap();
                    let mut cmd = self.commands.get(key).unwrap().clone();

                    while let Some(c) = cmd.pop() {
                        input.push_front(c)
                    }
                }
                Commands::RefCmd(key) => {
                    let mut cmd = self.commands.get(&key).unwrap().clone();

                    while let Some(c) = cmd.pop() {
                        input.push_front(c)
                    }
                }
            };
        };

        Ok(())
    }

    fn add_new_cmd(&mut self, input: &mut std::str::Split<'static, char>) -> Result {
        let cmd_name = if let Some(n) = input.next() {
            let name = n.to_lowercase().as_bytes().to_vec();

            if name[0].is_ascii_digit() {
                return Err(Error::InvalidWord)
            }

            Commands::Custom(name)
        } else {
            return Err(Error::InvalidWord)
        };

        let mut cmds: Vec<Commands> = vec![];

        for n in input.by_ref() {
            if n == ";" {
                // Easiest way to create unique id - just increment by 1
                let new_id = self.commands.len() + 1;
                self.commands.insert(new_id, cmds);
                self.latest.insert(cmd_name, new_id);

                return Ok(())
            }

            match n.try_into() {
                Ok(cmd) => cmds.push(cmd),
                Err(e) => {
                    if let Some((_, key)) = self.contains_cmd(n) {
                        // This way the embedded command will point to the correct invocation without
                        // having to expand and increase memory allocation
                        cmds.push(Commands::RefCmd(key))
                    } else {
                        return Err(e)
                    }
                }
            }
        }

        Err(Error::InvalidWord)
    }

    fn get_commands(&mut self, input: &'static str) -> std::result::Result<VecDeque<Commands>, Error> {
        let mut input = input.split(' ');
        let mut cmds: VecDeque<Commands> = VecDeque::new();

        while let Some(i) = input.next() {
            if i == ":" {
                self.add_new_cmd(&mut input)?;
                continue;
            }

            if let Some((c, _)) = self.contains_cmd(i) {
                cmds.push_back(c);
                continue
            }
            
            let c: Commands = i.try_into()?;
            cmds.push_back(c)
        };

        Ok(cmds)
    }

    fn contains_cmd(&self, cmd: &str) -> Option<(Commands, usize)> {
        let key = Commands::Custom(cmd.to_lowercase().as_bytes().to_vec());

        self.latest.get(&key).map(|k| (key, *k))
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