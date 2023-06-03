mod assets;
mod seed;

use crossterm::terminal::{self, size};
use seed::get_from_seed;
use std::io::{self, Write};

fn main() -> Result<(), io::Error> {
    let mut stdout = io::stdout();
    write!(
        stdout,
        "{}",
        crossterm::terminal::Clear(crossterm::terminal::ClearType::All)
    )?;
    stdout.flush()?;

    let seed: u64 = 1235;
    seed::get_from_seed(seed, 2, 4, 9);
    seed::get_from_seed(seed + 1, 2, 4, 9);

    let dim = size()?;
    let term_width = dim.0 as usize;
    let term_height = dim.1 as usize;

    let mut screen_as_vecs: Vec<Vec<String>> = Vec::new();
    let mut status_bar: Vec<String> = Vec::new();
    status_bar = set_status_bar(vec!["Have fun!".to_string()]);

    for _ in 0..term_height - status_bar.len() {
        let mut horizontal: Vec<String> = Vec::new();
        for _ in 0..term_width {
            horizontal.push(" ".to_string());
        }
        screen_as_vecs.push(horizontal);
    }

    print_screen(&mut stdout, &screen_as_vecs, &status_bar)?;

    Ok(())
}

fn print_screen(
    stdout: &mut io::Stdout,
    screen: &[Vec<String>],
    status_bar: &[String],
) -> Result<(), io::Error> {
    for row in screen {
        for char in row {
            write!(stdout, "{}", char)?;
        }
        writeln!(stdout)?;
    }
    for row in status_bar {
        writeln!(stdout, "{}", row)?;
    }
    stdout.flush()?;
    Ok(())
}

fn set_status_bar(texts: Vec<String>) -> Vec<String> {
    let mut returned: Vec<String> = Vec::new();
    for row in texts {
        returned.push(row);
    }
    returned
}
