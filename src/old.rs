// R(ust)ed(itor)
use std::env;
use std::fs::File;
use std::io::BufReader;
use std::io::prelude::*;

use colored::Colorize;

#[derive(PartialEq, Eq)]
enum MODES {
    COMMAND,
    INSERT,
}

struct RedState {
    mode: MODES,
    line: usize,
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        println!("Usage : {} <filename>", args.first().unwrap());
        return ;
    }

    let path = args.get(1).unwrap();
    let mut content: Vec<String>;

    if let Ok(file) = File::open(path) {
        content = BufReader::new(file)
            .lines()
            .map(|line| line.expect("Error while reading line"))
            .collect();
        println!("{}", std::fs::metadata(path).unwrap().len().to_string().yellow());
    }
    else if let Ok(_) = File::create(path) {
        content = Vec::new();
        println!("{}", "0".yellow());
    }
    else {
        return ;
    }

    red_core(&mut content, path);
    return ;
}

fn red_core(content: &mut Vec<String>, path: &str) {
    let mut state: RedState = RedState {
        mode: MODES::COMMAND,
        line: content.len(),
    };
    let mut input: String = String::new();
    let mut parsed: (usize, usize, String);

    if state.line != 0 {
        state.line -= 1;
    }
    loop {
        input.clear();
        std::io::stdin().read_line(&mut input).unwrap();
        if state.mode == MODES::COMMAND {
            input = input.trim_end().to_string();
            if input.len() == 0 {
                println!("{}", "?".bold().red());
                continue ;
            }
            if let Ok(x) = input.parse::<usize>() {
                if x <= content.len() {
                    if x == 0 {
                        state.line = 0;
                    }
                    state.line = x - 1;
                }
                else {
                    println!("{}", "?".bold().red());
                }
                continue ;
            }
            if input.chars().nth(0).unwrap() == '%' {
                if content.len() > 0 {
                    parsed = (1, content.len(), String::from(input.pop().unwrap()));
                }
                else {
                    continue ;
                }
            }
            else {
                parsed = match red_parsed_command(input.clone(), state.line, content) {
                    Ok(x) => x,
                    Err(_) => (0, 0, "?".to_string())
                };
            }
            if parsed.2 == "a" {
                state.mode = MODES::INSERT;
                state.line = parsed.0;
            }
            else if parsed.2 == "d" { red_delete_command(parsed, content); }
            else if parsed.2 == "p" { red_print_line(parsed, content, false); }
            else if parsed.2 == "n" { red_print_line(parsed, content, true); }
            else if parsed.2 == "c" {
                if parsed.0 == 0 {
                    println!("{}", "?".bold().red());
                    continue ;
                }
                state.line = parsed.0 - 1;
                red_delete_command(parsed, content);
                state.mode = MODES::INSERT;
            }
            else if parsed.2 == "w" { red_save_file(content, path); }
            else if parsed.2 == "q" { return; }
            else { println!("{}", "?".bold().red()); }
        }
        else if state.mode == MODES::INSERT {
            match &input.chars().nth(0).unwrap() {
                '.' => {
                    state.mode = MODES::COMMAND;
                    state.line = content.len();
                },
                _ => {
                    input.pop();
                    if state.line > content.len() {
                        state.line = content.len();
                    }
                    content.insert(state.line, input.clone());
                    state.line += 1;
                },
            };
        }
    }
}

fn red_parsed_command(mut input: String, line: usize, content: &mut Vec<String>) -> Result<(usize, usize, String), &str> {
    if input.len() == 1 {
        return Ok((line + 1, line + 1, input));
    }
    else if input.len() == 2 || input.find(',') == None {
        let command: String = String::from(input.pop().unwrap());
        if input.chars().any(|c| matches!(c, 'a'..='z')){
            return Err("?");
        }
        let parsed_line: usize = input.parse::<usize>().unwrap();
        return Ok((parsed_line, parsed_line, command))
    }
    else {
        let command: String = String::from(input.pop().unwrap());
        let split: Vec<&str> = input.split(',').collect();
        let mut first: usize = line;
        let mut last: usize = content.len() - 1;
        if let Some(f) = split.get(0) {
           first = f.parse::<usize>().unwrap();
        }
        if let Some(f) = split.get(1) {
           last = f.parse::<usize>().unwrap();
        }
        return Ok((first, last, command));
    }
}

fn red_delete_command(command: (usize, usize, String), content: &mut Vec<String>) {
    let start: usize;
    let end: usize;
    if command.0 == 0 || command.1 == 0 {
        println!("{}", "?".bold().red());
        return ;
    }
    if command.0 > command.1 {
        start = command.1 - 1;
        end = command.0 - 1;
    }
    else {
        start = command.0 - 1;
        end = command.1 - 1;
    }
    if end >= content.len() || start >= content.len() {
        println!("{}", "?".bold().red());
        return ;
    }
    content.drain(start..=end);
}

fn red_print_line(command: (usize, usize, String), content: &mut Vec<String>, numbers: bool) {
    let mut start: usize;
    let end: usize;
    if command.0 == 0 || command.1 == 0 {
        println!("{}", "?".bold().red());
        return ;
    }
    if command.0 > command.1 {
        start = command.1 - 1;
        end = command.0 - 1;
    }
    else {
        start = command.0 - 1;
        end = command.1 - 1;
    }
    if end > content.len() || start > content.len() {
        println!("{}", "?".bold().red());
        return ;
    }
    while start <= end {
        if let Some(x) = content.get(start) {
            let padding = end.to_string().len() - (start + 1).to_string().len();
            if numbers {
                println!("{}{}    {}", " ".repeat(padding), (start + 1).to_string().bold().green(), x);
            }
            else {
                println!("{}", x);
            }
            start += 1;
        }
        else {
            break ;
        }
    }
}

fn red_save_file(content: &mut Vec<String>, path: &str) {
    _ = File::create(path).unwrap().write_all(
        content.join("\n")
        .as_bytes()
    );
    println!("{}", std::fs::metadata(path).unwrap().len().to_string().yellow());
}

