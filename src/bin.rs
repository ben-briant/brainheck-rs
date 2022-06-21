use std::collections::HashMap;
use std::fs::read_to_string;
use std::io::Read;

fn main() {
    let lines = read_to_string("./examples/head")
        .unwrap_or_else(|_| panic!("Unable to read file"))
        .chars()
        .filter(|c| !c.is_ascii_whitespace())
        .collect::<String>();
    let (tokens, loop_map) = generate_tokens(&lines);
    run_commands(tokens, loop_map);
}

fn generate_tokens(chars: &str) -> (Vec<char>, HashMap<usize, usize>) {
    let mut stack: Vec<usize> = Vec::new();
    let mut map: HashMap<usize, usize> = HashMap::new();
    let tokens = chars
        .chars()
        .enumerate()
        .map(|(i, c)| {
            match c {
                '[' => {
                    stack.push(i);
                }
                ']' => {
                    let start = stack
                        .pop()
                        .unwrap_or_else(|| panic!("Unable to match end of loop"));
                    let end = i;
                    map.insert(start, end);
                    map.insert(end, start);
                }
                _ => (),
            }
            c
        })
        .collect::<Vec<char>>();
    (tokens, map)
}

fn run_commands(tokens: Vec<char>, loop_map: HashMap<usize, usize>) {
    let mut memory: [u8; 20000] = [0; 20000];
    let mut instruction = 0;
    let mut mem_pointer = 0;
    while instruction < tokens.len() {
        match tokens[instruction] {
            '>' => {
                mem_pointer += 1;
            }
            '<' => {
                mem_pointer -= 1;
            }
            '+' => {
                memory[mem_pointer] = u8::wrapping_add(memory[mem_pointer], 1);
            }
            '-' => {
                memory[mem_pointer] = u8::wrapping_sub(memory[mem_pointer], 1);
            }
            '.' => {
                print!("{}", memory[mem_pointer] as char)
            }
            ',' => {
                std::io::stdin()
                    .read(&mut memory[mem_pointer..mem_pointer + 1])
                    .unwrap();
            }
            '[' => {
                if memory[mem_pointer] == 0 {
                    instruction = *loop_map.get(&instruction).unwrap();
                }
            }
            ']' => {
                if memory[mem_pointer] != 0 {
                    instruction = *loop_map.get(&instruction).unwrap();
                }
            }
            _ => (),
        }
        instruction += 1;
    }
}
