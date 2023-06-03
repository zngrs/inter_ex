mod assets;
mod seed;

use seed::get_from_seed;
use termion::terminal_size;

fn main() -> Result<(), std::io::Error> {
    println!("{}", termion::clear::All); // clear terminal
    let seed: u64 = 1235;
    seed::get_from_seed(seed, 2, 4, 9);
    seed::get_from_seed(seed + 1, 2, 4, 9);

    // Get the terminal dimensions
    let dim = terminal_size()?;
    let term_width = dim.0;
    let term_height = dim.1;

    let mut screen_as_vecs: Vec<Vec<String>> = Vec::new();
    let mut status_bar: Vec<String> = Vec::new();
    status_bar = set_status_bar(vec!["Have fun!".to_string()]);

    for _ in 0..term_height - status_bar.len() as u16 {
        let mut horizontal: Vec<String> = Vec::new();
        for _ in 0..term_width {
            horizontal.push(" ".to_string());
        }
        screen_as_vecs.push(horizontal);
    }

    print_screen(screen_as_vecs, status_bar);

    for row in assets::assets() {
        println!("{}", row);
    }

    Ok(())
}

fn print_screen(screen: Vec<Vec<String>>, status_bar: Vec<String>) {
    for row in screen {
        for char in row {
            print!("{}", char)
        }
        println!();
    }
    for row in status_bar {
        println!("{}", row)
    }
}

fn set_status_bar(texts: Vec<String>) -> Vec<String> {
    let mut returned: Vec<String> = Vec::new();
    for row in texts {
        returned.push(row);
    }
    return returned;
}
