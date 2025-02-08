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
    prompt: bool,
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let mut state: RedState = red_init_state(args);

    red_main_loop(&mut state);
    red_print_goodbye();
}

fn red_main_loop(state: &mut RedState) {
    let mut input: String = String::new();

    loop {
        input.clear();
        if state.prompt {
            print!("{}", "*".to_string().bold().blue());
            std::io::stdout().flush().unwrap();
        }
        if let Ok(_) = std::io::stdin().read_line(&mut input){
            if state.mode == MODES::COMMAND {
                input = input.trim_end().to_string();
            }
        }
        else {
            red_print_error();
            continue ;
        }
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
                    input = input.trim_end().to_string();
                    state.content.insert(state.line, input.clone());
                    state.line += 1;
                    state.modified = true;
                }
            }
        }
    }
}

fn red_handle_multi_command(state: &mut RedState, input: &mut String) -> bool {
    let mut start: usize = 0;
    let mut end: usize = 0;
    let mut command: char = '?';
    let mut comma_split: Vec<&str> = input.split(",").collect();
    let space_split: Vec<&str> = input.split(" ").collect();

    if state.content.len() == 0 {
        red_print_error();
        return false;
    }

    // %?
    if input.chars().nth(0).unwrap() == '%' {
        if input.len() == 2 {
            start = 0;
            end = state.content.len() - 1;
            command = input.pop().unwrap();
        }
        else {
            red_print_error()
        }
    }
    // ,?
    else if input.chars().nth(0).unwrap() == ',' {
        if input.len() == 2 {
            start = 0;
            end = state.content.len() - 1;
            command = input.pop().unwrap();
        }
        else {
            command = input.pop().unwrap();
            let split: String = input.split(",").nth(1).unwrap().to_string();
            if let Ok(x) = split.parse::<usize>() {
                if x != 0 && x <= state.content.len() {
                    start = state.line;
                    end = x - 1;
                }
                else {
                    red_print_error();
                    return false;
                }
            }
            else {
                red_print_error();
                return false;
            }
        }
    }
    // $?
    else if input.chars().nth(0).unwrap().is_numeric() && comma_split.len() != 2 {
        command = input.pop().unwrap();
        if let Ok(x) = input.parse::<usize>() {
            if x != 0 && x <= state.content.len() {
                start = x - 1;
                end = x - 1;
            }
            else {
                red_print_error();
                return false;
            }
        }
        else {
            red_print_error();
            return false;
        }
    }
    // $,$
    else if comma_split.len() == 2 {
        let mut parse_command: String = comma_split.pop().unwrap().to_string();
        command = parse_command.pop().unwrap();
        comma_split.push(&parse_command);

        if let Ok(x) = comma_split.first().unwrap().parse::<usize>() {
            if x != 0 && x <= state.content.len() {
                start = x - 1;
            }
            else {
                red_print_error();
                return false;
            }
        }
        if let Ok(x) = comma_split.last().unwrap().parse::<usize>() {
            if x != 0 && x <= state.content.len() {
                end = x - 1;
            }
            else {
                red_print_error();
                return false;
            }
        }
        if start > end {
            red_print_error();
            return false;
        }
    }
    else if space_split.len() == 2 {
        if let Some(x) = space_split.get(0) {
            if x.to_string() == "w" {
                red_open_file(state, space_split.get(1).unwrap().to_string(), false, true);
                red_save_file(state);
            }
            else {
                red_print_error();
            }
            return false;
        }
    }
    // Special cases
    else if input == "wq" {
        red_save_file(state);
        return true;
    }
    else {
        red_print_error();
        return false;
    }

    match command {
        'p' => { red_print_lines(state, start, end, false); }
        'n' => { red_print_lines(state, start, end, true); }

        'd' => {
            state.content.drain(start..=end);
            state.modified = true;
            state.line = 0;
        }
        'c' => {
            state.content.drain(start..=end);
            state.mode = MODES::INSERT;
            state.line = start;
            state.modified = true;
        }

        'a' if start == end => { state.line = start + 1; state.mode = MODES::INSERT; }
        'i' if start == end => { state.line = start; state.mode = MODES::INSERT; }

        _ => { red_print_error() }
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
        'P' => { state.prompt = !state.prompt; }

        ',' if state.content.len() != 0 => { state.line = state.content.len() - 1; println!("{}", state.content.get(state.line).unwrap()); }
        '%' if state.content.len() != 0 => { state.line = state.content.len() - 1; println!("{}", state.content.get(state.line).unwrap()); }
        'p' if state.content.len() != 0 => { println!("{}", state.content.get(state.line).unwrap()); }
        'n' if state.content.len() != 0 => { println!("{}    {}", (state.line + 1).to_string().bold().green(), state.content.get(state.line).unwrap()); }

        'q' if !state.modified => { return true; }
        'Q' => { return true; }
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
        prompt: false,
    };

    if args.len() > 1 {
        red_open_file(&mut state, args.get(1).unwrap().to_string(), true, false);
    }
    return state;
}

fn red_open_file(state: &mut RedState, filename: String, verbose: bool, safeguard: bool) {
    if Path::new(&filename).is_dir() {
        println!("{}{}", filename.bold().red(), ": is a folder".to_string().bold().red());
    }
    else if Path::new(&filename).exists() {
        state.filename = filename;
        if !safeguard {
            if let Ok(file) = File::open(&state.filename) {
                state.content = BufReader::new(file)
                    .lines()
                    .map(|line| line.expect("Error while reading line"))
                    .collect();
            }
            red_save_file(state);
        }
        if state.content.len() != 0 {
            state.line = state.content.len() - 1;
        }
    }
    else {
        state.filename = filename;
        println!("{}", "Creating a new file".to_string().bold().yellow());
        if verbose {
            println!("{}", 0.to_string().bold().yellow());
        }
    }
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
