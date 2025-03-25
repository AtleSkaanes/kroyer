use std::io::{BufRead as _, IsTerminal};

pub fn read_stdin() -> Option<String> {
    if !std::io::stdin().is_terminal() {
        let str = std::io::stdin()
            .lock()
            .lines()
            .fold(String::new(), |acc, line| {
                acc + &line.unwrap_or_default() + "\n"
            });
        Some(str)
    } else {
        None
    }
}
