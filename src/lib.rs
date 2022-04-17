// i_o - my input/output library
// 20220402

//use std::io::Write;
use colored::Colorize;
use crossterm::{cursor, execute};
use getch::Getch;
use std::io;
use std::io::prelude::*;
use std::io::{stdout, Write};

pub fn cls() {
    std::process::Command::new("clear").status().unwrap();
}

pub fn cmove(x: u16, y: u16) {
    execute!(stdout(), cursor::MoveTo(x, y)).unwrap();
}

pub fn get_int(prompt: &str) -> i32 {
    loop {
        let mut buffer = String::new();
        print!("{}", prompt);

        std::io::stdout().flush().unwrap();

        std::io::stdin()
            .read_line(&mut buffer)
            .expect("Failed to read line");

        let buffer: i32 = match buffer.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };
        return buffer;
    }
}

pub fn get_float(prompt: &str) -> f32 {
    loop {
        let mut buffer = String::new();
        print!("{}", prompt);

        std::io::stdout().flush().unwrap();

        std::io::stdin()
            .read_line(&mut buffer)
            .expect("Failed to read line");

        let buffer: f32 = match buffer.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };
        return buffer;
    }
}

pub fn get_string(prompt: &str) -> String {
    let mut buffer = String::new();
    print!("{}", prompt);

    std::io::stdout().flush().unwrap();

    std::io::stdin()
        .read_line(&mut buffer)
        .expect("Failed to read line");

    while buffer.ends_with('\n') || buffer.ends_with('\r') {
        buffer.pop();
    }
    return buffer;
}

pub fn get_string_default(prompt: &str, default: &str) -> String {
    let mut buffer = String::new();
    print!("{} [{}]: ", prompt, default);

    std::io::stdout().flush().unwrap();

    std::io::stdin()
        .read_line(&mut buffer)
        .expect("Failed to read line");

    while buffer.ends_with('\n') || buffer.ends_with('\r') {
        buffer.pop();
    }

    if buffer.eq("") {
        return default.to_string();
    } else {
        return buffer;
    }
}

pub fn horiz_line() {
    for _i in 0..80 {
        print!("-");
    }
    println!("");
}

pub fn horiz_line_blue() {
    for _i in 0..80 {
        print!("{}", "-".blue().bold());
    }
    println!("");
}

pub fn menu(items: &Vec<&str>) -> u8 {
    println!("");
    println!("Option Menu:");
    for (i, item) in items.iter().enumerate() {
        println!("    {}) {}", i + 1, item);
    }

    println!("");
    print!("Select Option: ");
    io::stdout().flush().unwrap();

    let mut _a: u8 = 0;
    let menu_len = items.len();
    loop {
        let g = Getch::new();
        _a = g.getch().unwrap();
        if _a <= 48 || _a > (48 + menu_len as u8) {
            continue;
        }
        break;
    }

    println!("");

    _a - 48
}

// menu_horiz - example use
//
// let keys = vec!["a", "r", "e", "d", "s", "m", "q"];
// let menu_items = vec!["Add", "Remove", "Edit", "Details", "Summary", "Menu", "Quit"];
// let val = menu_horiz(keys, menu_items);
//
pub fn menu_horiz(keys: &Vec<&str>, items: &Vec<&str>) -> char {
    let (_width, height) = tsize();
    cmove(0, height - 2);

    horiz_line();
    for (i, item) in items.iter().enumerate() {
        print!("{:>4}:{}", keys[i].green(), item);
    }
    execute!(stdout(), cursor::Hide).unwrap();
    io::stdout().flush().unwrap();

    let mut _a: u8 = 0;
    let keys_len = keys.len();
    loop {
        let mut flag = false;
        let g = Getch::new();
        _a = g.getch().unwrap();

        for i in 0..keys_len {
            let ch = keys[i].chars().nth(0).unwrap();
            if (_a as char) == ch {
                flag = true;
                break;
            }
        }
        if flag == true {
            break;
        }
    }

    println!("");

    _a as char
}

pub fn pause() {
    let mut stdin = io::stdin();
    let mut stdout = io::stdout();
    write!(stdout, "Press ENTER key to continue...").unwrap();
    stdout.flush().unwrap();
    let _ = stdin.read(&mut [0u8]).unwrap();
}

pub fn pause_any() {
    println!("Press any key to continue...");
    let g = Getch::new();
    let _keypress = g.getch().unwrap();
}

pub fn print_title(title_string: &str) {
    for c in title_string.chars() {
        print!("{}", " ");
        print!("{}", c);
    }
    println!("");
    horiz_line();
}

pub fn print_title_blue(title_string: &str) {
    horiz_line_blue();
    for c in title_string.chars() {
        print!("{}", " ");
        print!("{}", c.to_string().blue().bold());
    }
    println!("");
    horiz_line_blue();
    println!("");
}

pub fn splash_screen(line1: &str, line2: &str) {
    //const VERSION: &str = env!("CARGO_PKG_VERSION");

    cls();
    let (width, height) = tsize();

    let line1_length: u16 = line1.len() as u16;
    cmove(width / 2 - line1_length / 2, height / 2 - 1);
    println!("{}", line1.bold());

    let line2_length: u16 = line2.len() as u16;
    cmove(width / 2 - line2_length / 2, height / 2 + 1);
    println!("{}", line2);

    execute!(stdout(), cursor::Hide).unwrap();

    // pause for splash screen
    //let one_sec = std::time::Duration::from_millis(1000);
    let dur = std::time::Duration::new(2, 0);
    std::thread::sleep(dur);
    cls();

    execute!(stdout(), cursor::Show).unwrap();
}

pub fn timestamp() -> String {
    let now = chrono::Local::now();
    return now.to_string();
}

pub fn tsize() -> (u16, u16) {
    let mut width: u16 = 0;
    let mut height: u16 = 0;

    if let Some((w, h)) = term_size::dimensions() {
        width = w as u16;
        height = h as u16;
    } else {
        println!("Unable to determine term size");
    }

    (width, height)
}
