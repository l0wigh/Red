use std::env;
use std::fs::File;
use std::io::BufReader;
use std::io::prelude::*;
use std::path::Path;

use colored::Colorize;

#[derive(PartialEq, Eq)]
enum MODES {
    COMMAND,
    INSERT,
}

struct RedState {
    mode: MODES,
    line: usize,
    content: Vec<String>,
    filename: String,
    filesize: u64,
    modified: bool,
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let mut state: RedState = red_init_state(args);

    red_main_loop(&mut state);
}

fn red_main_loop(state: &mut RedState) {
    let mut input: String = String::new();

    loop {
        input.clear();
        std::io::stdin().read_line(&mut input).unwrap();
        input = input.trim_end().to_string();
        if state.mode == MODES::COMMAND {
            if red_is_numbers(state, &mut input) { continue ; }
            if input.len() == 0 {
                red_print_error();
                continue ;
            }
            if input.len() == 1 {
                if red_handle_single_command(state, &mut input) {
                    return ;
                }
            }
            else {
                if red_handle_multi_command(state, &mut input) {
                    return ;
                }
            }
        }
        else if state.mode == MODES::INSERT {
            match input.chars().nth(0).unwrap() {
                '.' => { state.mode = MODES::COMMAND; state.line -= 1; }
                _ => {
                    state.content.insert(state.line, input.clone());
                    state.line += 1;
                    state.modified = true;
                }
            }
        }
    }
}

fn red_handle_multi_command(state: &mut RedState, input: &mut String) -> bool {
    let space_split: Vec<&str> = input.split(' ').collect();
    // let comma_split: Vec<&str> = input.split(',').collect();
    let mut chars_split: Vec<&str> = input.split("").collect();
    let full_file_chars = ["%", ","];
    chars_split.remove(0);
    chars_split.pop();

    if input.len() == 2 {
        if full_file_chars.contains(chars_split.first().unwrap()) && state.content.len() != 0 {
            match chars_split.pop().unwrap() {
                "n" => { red_print_lines(state, 0, state.content.len() - 1, true); }
                "p" => { red_print_lines(state, 0, state.content.len() - 1, false); }

                "d" => {
                    state.content.drain(0..=state.content.len() - 1);
                    state.modified = true;
                }
                "c" => {
                    state.content.drain(0..=state.content.len() - 1);
                    state.mode = MODES::INSERT;
                    state.line = 0;
                    state.modified = true;
                }

                _ => { red_print_error(); }
            };
        }
        else if input == "wq" {
            if state.filename.len() != 0 {
                red_save_file(state);
                return true;
            }
            red_print_error();
        }
        else {
            red_print_error();
        }
    }
    else if chars_split.first().unwrap().to_string() == "w" {
        state.filename = space_split.get(1).unwrap().to_string();
        red_save_file(state);
    }
    else {
        red_print_error();
    }
    return false;
}

fn red_print_lines(state: &mut RedState, start: usize, end: usize, numbers: bool) {
    let mut l_start: usize = start;

    while l_start <= end {
        if numbers {
            let padding = end.to_string().len() - (l_start + 1).to_string().len();
            println!("{}{}   {}",
                " ".repeat(padding),
                (l_start + 1).to_string().bold().green(),
                state.content.get(l_start as usize).unwrap()
            );
        }
        else {
            println!("{}", state.content.get(l_start as usize).unwrap());
        }
        l_start += 1;
    }
}

fn red_handle_single_command(state: &mut RedState, input: &mut String) -> bool {
    let command: char = input.chars().nth(0).unwrap();
    match command {
        ',' if state.content.len() != 0 => { state.line = state.content.len() - 1; println!("{}", state.content.get(state.line).unwrap()); }
        '%' if state.content.len() != 0 => { state.line = state.content.len() - 1; println!("{}", state.content.get(state.line).unwrap()); }
        'p' if state.content.len() != 0 => { println!("{}", state.content.get(state.line).unwrap()); }
        'n' if state.content.len() != 0 => { println!("{}    {}", (state.line + 1).to_string().bold().green(), state.content.get(state.line).unwrap()); }

        'q' if !state.modified => { red_print_goodbye(); return true; }
        'Q' => { red_print_goodbye(); return true; }
        'w' if state.filename.len() != 0 => { red_save_file(state) }

        'a' if state.content.len() != 0 => { state.line += 1; state.mode = MODES::INSERT; }
        'a' | 'i' => { state.mode = MODES::INSERT; }
        'c' => { state.content.remove(state.line); state.mode = MODES::INSERT; }

        'd' => { state.content.remove(state.line); }

        _ => { red_print_error() }
    };
    return false;
}

fn red_is_numbers(state: &mut RedState, input: &mut String) -> bool {
    if let Ok(x) = input.parse::<usize>() {
        if x <= state.content.len() && x != 0 {
            state.line = x - 1;
            println!("{}", state.content.get(state.line).unwrap());
        }
        else {
            red_print_error();
        }
        return true;
    }
    return false;
}

fn red_print_error() {
    println!("{}", "?".to_string().bold().red());
}

fn red_print_goodbye() {
    println!("{}", "Have a nice day !".bold().green());
}

fn red_init_state(args: Vec<String>) -> RedState {
    let mut state: RedState = RedState {
        mode: MODES::COMMAND,
        line: 0,
        content: Vec::new(),
        filename: String::new(),
        filesize: 0,
        modified: false,
    };

    if args.len() > 1 {
        if Path::new(args.get(1).unwrap()).is_dir() {
            let error_filename: String = args.get(1).unwrap().to_string();
            println!("{}{}", error_filename.bold().red(), ": is a folder".to_string().bold().red());
        }
        else if Path::new(args.get(1).unwrap()).exists() {
            state.filename = args.get(1).unwrap().to_string();
            if let Ok(file) = File::open(&state.filename) {
                state.content = BufReader::new(file)
                    .lines()
                    .map(|line| line.expect("Error while reading line"))
                    .collect();
            }
            red_save_file(&mut state);
            if state.content.len() != 0 {
                state.line = state.content.len() - 1;
            }
        }
        else {
            state.filename = args.get(1).unwrap().to_string();
            println!("{}", "Creating a new file".to_string().bold().yellow());
            println!("{}", 0.to_string().bold().yellow());
        }
    }
    return state;
}

fn red_save_file(state: &mut RedState) {
    if let Ok(mut x) = File::create(&state.filename) {
        _ = x.write_all(state.content.join("\n").as_bytes());
    }
    else {
        println!("{}", "File is protected".to_string().bold().red());
        state.filename = "".to_string();
        return ;
    }
    state.modified = false;
    state.filesize = std::fs::metadata(&state.filename).unwrap().len();
    println!("{}", state.filesize.to_string().bold().yellow());
}
