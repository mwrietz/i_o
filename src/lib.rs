// i_o - my input/output library
// 20220417

use colored::Colorize;
use crossterm::{cursor, execute};
use getch::Getch;
use std::io;
use std::io::prelude::*;
use std::io::{stdout, Write};

pub struct Frame {
    pub title: String,
    pub title_color: String,
    pub x: u16,
    pub y: u16,
    pub w: u16,
    pub h: u16,
}

impl Frame {
    pub fn display(&self) {

        let ul = "╭".white();
        let ur = "╮".white();
        let ll = "╰".white();
        let lr = "╯".white();
        let hor = "─".white();
        let ver = "│".white();

        // draw top horizontal
        cmove(self.x, self.y);
        print!("{}", ul);
        for _i in 0..(self.w-2) {
            print!("{}", hor);
        }
        print!("{}", ur);

        // draw middle
        for i in 0..(self.h-1) {
            cmove(self.x, self.y+i+1);
            print!("{}", ver);
            for _j in 0..(self.w-2) {
                print!(" ");
            }
            print!("{}", ver);
        }

        // draw bottom horizontal
        cmove(self.x, self.y+self.h);
        print!("{}", ll);
        for _i in 0..(self.w-2) {
            print!("{}", hor);
        }
        println!("{}", lr);

        if self.title.len() > 0 {
            // print title 
            cmove(self.x+2, self.y);
            if self.title_color == "red" {
                print!(" {} ", self.title.red());
            }
            if self.title_color == "green" {
                print!(" {} ", self.title.green());
            }
            if self.title_color == "blue" {
                print!(" {} ", self.title.blue());
            }
            if self.title_color == "yellow" {
                print!(" {} ", self.title.yellow());
            }
            if self.title_color == "purple" {
                print!(" {} ", self.title.purple());
            }
            if self.title_color == "white" {
                print!(" {} ", self.title.white());
            }
        }
    }
}

pub struct Window {
    pub x: u16,
    pub y: u16,
    pub w: u16,
    pub h: u16,
    pub title: String,
    pub title_color: String,
}

pub fn cls() {
    std::process::Command::new("clear").status().unwrap();
}

pub fn cmove(x: u16, y: u16) {
    execute!(stdout(), cursor::MoveTo(x, y)).unwrap();
}

pub fn dialog_box_get_string(width: u16, height: u16, title: &str, prompt: &str) -> String {
    let (term_width, term_height) = tsize();
    let x = (term_width - width)/2;
    let y = (term_height - height)/2;

    let ul = "╭";
    let ur = "╮";
    let ll = "╰";
    let lr = "╯";

    let hor = "─";
    let ver = "│";

    // draw top horizontal
    cmove(x, y);
    print!("{}", ul);
    for _i in 0..(width-2) {
        print!("{}", hor);
    }
    print!("{}", ur);

    // draw middle
    for i in 0..(height-1) {
        cmove(x, y+i+1);
        print!("{}", ver);
        for _j in 0..(width-2) {
            print!(" ");
        }
        print!("{}", ver);
    }

    // draw bottom horizontal
    cmove(x, y+height);
    print!("{}", ll);
    for _i in 0..(width-2) {
        print!("{}", hor);
    }
    println!("{}", lr);

    // print title and get string
    cmove(x+2, y);
    print!(" {} ", title.red());
    cmove(x+3, y+2);
    let s = get_string(prompt);

    s
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

pub fn get_float(prompt: &str) -> f64 {
    loop {
        let mut buffer = String::new();
        print!("{}", prompt);

        std::io::stdout().flush().unwrap();

        std::io::stdin()
            .read_line(&mut buffer)
            .expect("Failed to read line");

        let buffer: f64 = match buffer.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };
        return buffer;
    }
}

// this function is not working
pub fn get_float_default(prompt: &str, default: f64) -> f64 {
    loop {
        let mut buffer = String::new();
        print!("{} [{:.3}]: ", prompt, default);

        std::io::stdout().flush().unwrap();

        std::io::stdin()
            .read_line(&mut buffer)
            .expect("Failed to read line");

        // this if statment is not working
        if buffer.eq("\n") {
            return default;
        }

        let buffer: f64 = match buffer.trim().parse() {
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
        print!("{}", "─".blue().bold());
    }
    println!("");
}

pub fn menu(menu_title: &str, items: &Vec<&str>) -> u8 {
    println!("{}", menu_title);
    for (i, item) in items.iter().enumerate() {
        println!("    {}) {}", i + 1, item);
    }

    println!("");
    print!("Selection: ");
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

    _a as char
}

pub fn menu_horiz_blue(keys: &Vec<&str>, items: &Vec<&str>) -> char {
    let (_width, height) = tsize();
    cmove(0, height - 2);

    horiz_line_blue();
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
    let (w, h) = tsize();
    let message = "Press any key to continue...".blue();
    let message_len: u16 = message.len() as u16;
    cmove((w - message_len)/2, h - 2);
    print!("{}", message);
    std::io::stdout().flush().unwrap();
    let g = Getch::new();
    let _keypress = g.getch().unwrap();
    cls();
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
    println!("");
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
    let size = crossterm::terminal::size();
    let (w, h) = match size {
        Ok((w, h)) => (w, h),
        Err(error) => panic!("tsize error: {:?}", error),
    };
    (w, h)
}

pub fn window(wdw: &Window) {

    let ul = "┌".white();
    let ur = "┐".white();
    let ll = "└".white();
    let lr = "┘".white();
    let hor = "─".white();
    let ver = "│".white();

    // draw top horizontal
    cmove(wdw.x, wdw.y);
    print!("{}", ul);
    for _i in 0..(wdw.w-2) {
        print!("{}", hor);
    }
    print!("{}", ur);

    // draw middle
    for i in 0..(wdw.h-1) {
        cmove(wdw.x, wdw.y+i+1);
        print!("{}", ver);
        for _j in 0..(wdw.w-2) {
            print!(" ");
        }
        print!("{}", ver);
    }

    // draw bottom horizontal
    cmove(wdw.x, wdw.y+wdw.h);
    print!("{}", ll);
    for _i in 0..(wdw.w-2) {
        print!("{}", hor);
    }
    println!("{}", lr);

    // print title and get string
    cmove(wdw.x+2, wdw.y);
    if wdw.title_color == "red" {
        print!(" {} ", wdw.title.red());
    }
    if wdw.title_color == "green" {
        print!(" {} ", wdw.title.green());
    }
    if wdw.title_color == "blue" {
        print!(" {} ", wdw.title.blue());
    }
    if wdw.title_color == "yellow" {
        print!(" {} ", wdw.title.yellow());
    }
    if wdw.title_color == "purple" {
        print!(" {} ", wdw.title.purple());
    }
    if wdw.title_color == "white" {
        print!(" {} ", wdw.title.white());
    }
}
