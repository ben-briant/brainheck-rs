use std::collections::HashMap;
use std::fs::read_to_string;
use std::io::Read;
use token::Token;

fn main() {
    let lines = read_to_string("./examples/fib")
        .unwrap_or_else(|_| panic!("Unable to read file"))
        .chars()
        .filter(|c| !c.is_ascii_whitespace())
        .collect::<String>();
    let (tokens, loop_map) = generate_tokens(&lines);
    run_commands(tokens, loop_map);
}

fn generate_tokens(chars: &str) -> (Vec<Token>, HashMap<usize, usize>) {
    let mut stack: stack::Stack<usize> = stack::Stack::new();
    let mut map: HashMap<usize, usize> = HashMap::new();

    // let tokens: Vec<Token> = Vec::new();
    let tokens = chars
        .chars()
        .enumerate()
        .map(|(i, c)| {
            let tok = token::char_to_token(c);
            match tok {
                Token::LoopStart => {
                    stack.push(i);
                }
                Token::LoopEnd => {
                    let start = stack
                        .pop()
                        .unwrap_or_else(|| panic!("Unable to match end of loop"));
                    let end = i;
                    map.insert(start, end);
                    map.insert(end, start);
                }
                _ => (),
            }
            tok
        })
        .collect::<Vec<Token>>();
    (tokens, map)
}

fn run_commands(tokens: Vec<Token>, loop_map: HashMap<usize, usize>) {
    let mut memory: [u8; 20000] = [0; 20000];
    let mut instruction = 0;
    let mut mem_pointer = 0;
    while instruction < tokens.len() {
        match tokens[instruction] {
            Token::MoveRight => {
                mem_pointer += 1;
            }
            Token::MoveLeft => {
                mem_pointer -= 1;
            }
            Token::Inc => {
                memory[mem_pointer] += 1;
            }
            Token::Dec => {
                memory[mem_pointer] -= 1;
            }
            Token::Output => {
                print!(
                    "{}",
                    // char::from_u8(memory[instruction])
                    //     .unwrap_or_else(|| panic!("Unable to convert memory value to char!"))
                    memory[mem_pointer] as char
                )
            }
            Token::Input => {
                std::io::stdin()
                    .read(&mut memory[instruction..instruction + 1])
                    .unwrap();
            }
            Token::LoopStart => {
                if memory[mem_pointer] == 0 {
                    instruction = *loop_map.get(&instruction).unwrap();
                }
            }
            Token::LoopEnd => {
                if memory[mem_pointer] != 0 {
                    instruction = *loop_map.get(&instruction).unwrap();
                }
            }
        }
        instruction += 1;
    }
}

mod stack {
    pub struct Stack<T> {
        stack: Vec<T>,
    }

    impl<T> Stack<T> {
        pub fn new() -> Self {
            Stack { stack: Vec::new() }
        }

        pub fn pop(&mut self) -> Option<T> {
            self.stack.pop()
        }

        pub fn push(&mut self, item: T) {
            self.stack.push(item)
        }

        pub fn size(&self) -> usize {
            self.stack.len()
        }
    }
}

mod token {
    #[derive(Debug)]
    pub enum Token {
        MoveRight,
        MoveLeft,
        Inc,
        Dec,
        Output,
        Input,
        LoopStart,
        LoopEnd,
    }

    pub fn char_to_token(c: char) -> Token {
        match c {
            '>' => Token::MoveRight,
            '<' => Token::MoveLeft,
            '+' => Token::Inc,
            '-' => Token::Dec,
            '.' => Token::Output,
            ',' => Token::Input,
            '[' => Token::LoopStart,
            ']' => Token::LoopEnd,
            _ => panic!("yeet"),
        }
    }
}
