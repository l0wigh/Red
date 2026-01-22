use std::env;
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;
use std::path::Path;

use colored::Colorize;
use regex::Regex;
use rustyline::history::History;
use sedregex::ReplaceCommand;
use rustyline::error::ReadlineError;
use rustyline::history::SearchDirection;
use rustyline::DefaultEditor;

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

    if let Some(x) = args.get(1) {
        if x == "--version" || x == "-v" {
            println!("{}", "0.4.0".bold().green());
            return;
        } else if x == "--help" || x == "-h" {
            println!(
                "{}",
                "Red Manual: https://github.com/l0wigh/Red/blob/master/doc/MANUAL.md"
                    .bold()
                    .green()
            );
            println!(
                "{}",
                "Red Commands: https://github.com/l0wigh/Red/blob/master/doc/COMMANDS.md"
                    .bold()
                    .green()
            );
            println!(
                "{}",
                "Ed Manual: https://www.gnu.org/software/ed/manual/ed_manual.html"
                    .bold()
                    .green()
            );
            println!("{}", "\nEd Manual will contain stuff not implemented in Red.\nJust try it and see if it works.".bold().green());
            return;
        } else if x == "--zen" || x == "-z" {
            println!("{}", "Without Ed, no Ex. Without Ex, no Vi. Without Vi, no Vim. Without Vim, no Good Programmers.".bold().green());
            return;
        }
    }

    let mut state: RedState = red_init_state(args);
    let mut rl = DefaultEditor::new().expect("Failed to init Red");

    red_main_loop(&mut state, &mut rl);
    red_print_goodbye();
}

fn red_main_loop(state: &mut RedState, mut rl: &mut DefaultEditor) {
    loop {
        let mut prompt = String::new();
        if state.prompt && state.mode != MODES::INSERT {
            let star = if state.modified {
                "*".bold().yellow()
            } else {
                "*".bold().blue()
            };
            prompt = format!(
                "\n[{}] {}",
                (state.line + 1).to_string().bold().green(),
                star
            );
        }

        let readline = rl.readline(&prompt);
        let mut input = match readline {
            Ok(line) => {
                if state.mode == MODES::COMMAND && !line.trim().is_empty() {
                    rl.add_history_entry(line.as_str()).ok();
                }
                if state.mode != MODES::INSERT {
                    println!();
                }
                line
            },
            Err(ReadlineError::Interrupted) => {
                red_print_error();
                continue;
            },
            Err(ReadlineError::Eof) => break,
            Err(_) => {
                red_print_error();
                continue;
            }
        };
        if state.mode == MODES::COMMAND {
            input = input.trim().to_string();
            if red_is_numbers(state, &mut input) {
                continue;
            }
            if input.len() == 0 {
                let history = rl.history();
                if history.len() > 0 {
                    let last_index = history.len() - 1;
                    match history.get(last_index, SearchDirection::Forward) {
                        Ok(Some(result)) => {
                            input = result.entry.to_string();
                        },
                        Ok(None) => { red_print_error(); continue },
                        Err(_) => { red_print_error(); continue }
                    }
                } else {
                    red_print_error();
                    continue;
                }
            }
            if input.len() == 1 {
                if red_handle_single_command(state, &mut input, &mut rl) {
                    return;
                }
            } else {
                if red_handle_multi_command(state, &mut input) {
                    return;
                }
            }
        } else if state.mode == MODES::INSERT {
            if input == "." {
                state.mode = MODES::COMMAND;
                if state.line > 0 {
                    state.line -= 1;
                }
            } else {
                input = input.trim_end().to_string();
                state.content.insert(state.line, input.clone());
                state.line += 1;
                state.modified = true;
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
        start = 0;
        end = state.content.len() - 1;
        if input.len() == 2 {
            command = input.pop().unwrap();
        } else if input.chars().nth(1).unwrap() == 's' {
            input.remove(0);
            red_handle_regex(state, input, true);
            return false;
        } else {
            red_print_error()
        }
    }
    // s?
    else if input.chars().nth(0).unwrap() == 's' {
        red_handle_regex(state, input, false);
        return false;
    }
    // /?
    else if input.chars().nth(0).unwrap() == '/' {
        input.remove(0);
        let re: Regex = Regex::new(input).unwrap();
        let mut did_find: bool = false;
        while start < state.content.len() {
            if let Some(_) = re.find(state.content.get(start).unwrap()) {
                red_print_lines(state, start, start, true);
                did_find = true;
            }
            start += 1;
        }
        if !did_find {
            red_print_error();
        }
        return false;
    }
    // ,?
    else if input.chars().nth(0).unwrap() == ',' {
        if input.len() == 2 {
            start = 0;
            end = state.content.len() - 1;
            command = input.pop().unwrap();
        } else {
            command = input.pop().unwrap();
            let split: String = input.split(",").nth(1).unwrap().to_string();
            if let Ok(x) = split.parse::<usize>() {
                if x != 0 && x <= state.content.len() {
                    start = state.line;
                    end = x - 1;
                } else {
                    red_print_error();
                    return false;
                }
            } else {
                red_print_error();
                return false;
            }
        }
    }
    // $?
    else if input.chars().nth(0).unwrap().is_numeric() && comma_split.len() != 2 {
        for i in 0..input.len() {
            if !input.chars().nth(i).unwrap().is_numeric() {
                if input.chars().nth(i).unwrap() == 's' {
                    let (number, regex) = input.split_at_mut(i);
                    state.line = number.parse::<usize>().unwrap() - 1;
                    red_handle_regex(state, &mut regex.to_string(), false);
                    return false;
                }
            }
        }
        command = input.pop().unwrap();
        if let Ok(x) = input.parse::<usize>() {
            if x != 0 && x <= state.content.len() {
                start = x - 1;
                end = x - 1;
            } else if x > state.content.len() && command == 'y' {
                start = state.content.len();
                end = state.content.len();
            } else {
                red_print_error();
                return false;
            }
        } else {
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
            } else if x != 0 && x - 1 <= state.content.len() {
                start = x - 1;
            } else {
                red_print_error();
                return false;
            }
        }
        if let Ok(x) = comma_split.last().unwrap().parse::<usize>() {
            if x != 0 && x <= state.content.len() {
                end = x - 1;
            } else if x > state.content.len() && command == 'y' {
                end = x - 1;
            } else {
                red_print_error();
                return false;
            }
        } else if comma_split.last().unwrap() == &"%" {
            end = state.content.len() - 1;
        }
        if start > end {
            red_print_error();
            return false;
        }
    } else if space_split.len() == 2 {
        if let Some(x) = space_split.get(0) {
            if x.to_string() == "w" {
                red_open_file(state, space_split.get(1).unwrap().to_string(), false, true);
                red_save_file(state);
            } else {
                red_print_error();
            }
            return false;
        }
    }
    // Special cases
    else if input == "wq" {
        if state.filename.len() != 0 {
            red_save_file(state);
            return true;
        } else {
            red_print_error();
            return false;
        }
    } else {
        red_print_error();
        return false;
    }

    match command {
        'p' => {
            red_print_lines(state, start, end, false);
        }
        'n' => {
            red_print_lines(state, start, end, true);
        }
        'd' => {
            state.content.drain(start..=end);
            state.modified = true;
            state.line = start;
            if start >= state.content.len() && state.content.len() != 0 {
                state.line = state.content.len() - 1;
            } else {
                state.line = 0;
            }
        }
        'c' => {
            state.content.drain(start..=end);
            state.mode = MODES::INSERT;
            state.line = start;
            state.modified = true;
        }
        'i' if start == end && state.line == 0 => {
            state.line = start;
            state.mode = MODES::INSERT;
        }
        'i' if start == end => {
            state.line = start + 1;
            state.mode = MODES::INSERT;
        }
        'a' if start == end => {
            state.line = start;
            state.mode = MODES::INSERT;
        }
        'y' => {
            let copy = state.content.get(state.line as usize).unwrap().to_string();
            while start <= end {
                state.content.insert(start, copy.clone());
                start += 1;
            }
            if end < state.line {
                state.line += 1;
            }
            state.modified = true;
        }
        'r' => {
            let replace = state.content.get(state.line as usize).unwrap().to_string();
            state.content.drain(start..=end);
            while start <= end {
                state.content.insert(start, replace.clone());
                start += 1;
            }
            state.modified = true;
        }
        _ => red_print_error(),
    }

    return false;
}

fn red_handle_regex(state: &mut RedState, input: &mut String, whole_file: bool) {
    let mut start: usize;
    let end: usize;
    let mut did_changed: bool = false;
    let regex_cmd: ReplaceCommand<'_>;

    if whole_file {
        start = 0;
        end = state.content.len() - 1;
    } else {
        start = state.line;
        end = state.line;
    }

    if let Ok(x) = ReplaceCommand::new(input) {
        regex_cmd = x;
    } else {
        red_print_error();
        return;
    }

    while start <= end {
        let previous: &str = state.content.get(start).unwrap();
        let after: String = regex_cmd.execute(previous).into_owned();
        if previous != after {
            state.content.remove(start);
            state.content.insert(start, after);
            red_print_lines(state, start, start, true);
            did_changed = true;
            state.modified = true;
        }
        start += 1;
    }

    if !did_changed {
        red_print_error();
    }
}

fn red_print_lines(state: &mut RedState, start: usize, end: usize, numbers: bool) {
    let mut l_start: usize = start;

    while l_start <= end {
        if numbers {
            let padding = (end + 1).to_string().len() - (l_start + 1).to_string().len();
            println!(
                "{}{}    {}",
                " ".repeat(padding),
                (l_start + 1).to_string().bold().green(),
                state.content.get(l_start as usize).unwrap()
            );
        } else {
            println!("{}", state.content.get(l_start as usize).unwrap());
        }
        l_start += 1;
    }
}

fn red_handle_single_command(state: &mut RedState, input: &mut String, rl: &mut DefaultEditor) -> bool {
    let command: char = input.chars().nth(0).unwrap();
    match command {
        'P' => {
            state.prompt = !state.prompt;
        }

        ',' if state.content.len() != 0 => {
            state.line = state.content.len() - 1;
            println!("{}", state.content.get(state.line).unwrap());
        }
        '%' if state.content.len() != 0 => {
            state.line = state.content.len() - 1;
            println!("{}", state.content.get(state.line).unwrap());
        }
        'p' if state.content.len() != 0 => {
            println!("{}", state.content.get(state.line).unwrap());
        }
        'n' if state.content.len() != 0 => {
            println!(
                "{}    {}",
                (state.line + 1).to_string().bold().green(),
                state.content.get(state.line).unwrap()
            );
        }
        'q' if !state.modified => {
            return true;
        }
        'Q' => {
            return true;
        }
        'w' if state.filename.len() != 0 => red_save_file(state),

        'i' if state.content.len() != 0 => {
            state.line += 1;
            state.mode = MODES::INSERT;
        }
        'i' | 'a' => {
            state.mode = MODES::INSERT;
        }
        'c' => {
            state.content.remove(state.line);
            state.mode = MODES::INSERT;
        }
        'd' => {
            if state.content.len() == 0 {
                return false;
            }
            state.content.remove(state.line);
            if state.line >= state.content.len() && state.content.len() != 0 {
                state.line = state.content.len() - 1;
            }
            state.modified = true;
        }
        'k' => {
            if state.line != 0 {
                state.line = state.line - 1;
            }
            println!("{}", state.content.get(state.line).unwrap());
        }
        'j' => {
            if state.line + 1 < state.content.len() {
                state.line = state.line + 1
            }
            println!("{}", state.content.get(state.line).unwrap());
        }
        'e' => {
            if !state.content.is_empty() {
                let curr_line = state.content.get(state.line).unwrap().clone();
                match rl.readline_with_initial("", (&curr_line, "")) {
                    Ok(new_line) => {
                        if !new_line.is_empty() {
                            state.content[state.line] = new_line;
                            state.modified = true;
                        }
                    },
                    Err(_) => { red_print_error(); }
                }
            } else {
                red_print_error();
            }
        }
        _ => red_print_error(),
    };
    return false;
}

fn red_is_numbers(state: &mut RedState, input: &mut String) -> bool {
    if let Ok(x) = input.parse::<usize>() {
        if x <= state.content.len() && x != 0 {
            state.line = x - 1;
            red_print_lines(state, state.line, state.line, true);
        } else {
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
        prompt: true,
    };

    if args.len() > 1 {
        red_open_file(&mut state, args.get(1).unwrap().to_string(), true, false);
    }
    return state;
}

fn red_open_file(state: &mut RedState, filename: String, verbose: bool, safeguard: bool) {
    if Path::new(&filename).is_dir() {
        println!(
            "{}{}",
            filename.bold().red(),
            ": is a folder".to_string().bold().red()
        );
    } else if Path::new(&filename).exists() {
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
    } else {
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
    } else {
        println!("{}", "File is protected".to_string().bold().red());
        state.filename = "".to_string();
        return;
    }
    state.modified = false;
    state.filesize = std::fs::metadata(&state.filename).unwrap().len();
    println!("{}", state.filesize.to_string().bold().yellow());
}
