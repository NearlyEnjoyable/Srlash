/*
    Srlash - Splash screen written in Rust
    Colored output
    Reads file as frames which are separated.

    TODO: Maybe add caching to tmp
    neofetch like system statistics
    Use raw libc for SIGTERM. It's a shame Rust stdlib is unable to do basic signalling :(
    change color change ripple
    Arguments: file, index, caching, Terminal sequences (Maybe more?: looping time)
*/

use ctrlc;

use std::process::Command;
use std::thread;

use std::io::Read;
use std::fmt::Write;
use std::process::Stdio;

mod parse;

const TS: [&str; 18] = /* Terminal Sequences */ [
    "\x1b[0m", // All attributes off
    "\x1b[38;5;0m", // Terminal set black
    "\x1b[38;5;1m", // dark red
    "\x1b[38;5;2m", // darkgreen
    "\x1b[38;5;3m", // dark yellow
    "\x1b[38;5;4m", // dark blu
    "\x1b[38;5;5m", // dark purple
    "\x1b[38;5;6m", // dark cyan
    "\x1b[38;5;7m", // light gray
    "\x1b[8m", // THIS sets hidden attribute
    "\x1b[38;5;8m", // grey
    "\x1b[38;5;9m", // red
    "\x1b[38;5;10m",// green
    "\x1b[38;5;11m",// yellow
    "\x1b[38;5;12m",// blue
    "\x1b[38;5;13m",// purple
    "\x1b[38;5;14m",// cyan
    "\x1b[38;5;15m",// white

];

const CLEAR_SCREEN: &str = "\x1b[2J";
const HIDE_CURSOR: &str = "\x1b[?25l";
const SHOW_CURSOR: &str = "\x1b[?25h";
const REINIT_SCREEN: &str = "\x1bc";

static mut WIDE: usize = 0;
static mut TALL: usize = 0;

fn main() {
    let (file, index, cache) = parse::cli();
    let output = String::from_utf8(Command::new("stty")
        .arg("size")
        .arg("-F")
        .arg("/dev/stderr")
        .stderr(Stdio::inherit())
        .output()
        .unwrap().stdout).unwrap();
    let mut formatted_output = output.split_whitespace().map(|x| x.parse::<usize>().unwrap());
    unsafe {
        TALL = formatted_output.next().unwrap();
        WIDE = formatted_output.next().unwrap();
    }

    /* Set SIGTERM handler to restore cursor */
    ctrlc::set_handler(move || {
        println!("{}", REINIT_SCREEN);
        std::process::exit(0);
    }).expect("Error setting SIGTERM handler");

    let path = std::path::Path::new(&file); //TODO: default file location where binary is run from
    let art = parse::get_art(path, index); //Get the desired art
    println!("{}", HIDE_CURSOR);
    match art {
        parse::Chunk::Picture(x) => draw(&x),
        parse::Chunk::Moving(y) => animate(y),
    }
    println!("{}", SHOW_CURSOR);
}

fn draw(art: &str) {
    let mut f = std::fs::File::open("/dev/urandom").unwrap(); //get random number !UNIX-ONLY!
    let max = art.lines().map(|x| x.len()).max().unwrap();
    let neg_line_num = art.lines().count() as isize * -1 ;
    let mut itr = TS.iter().cycle();
    let mut earlier_seq = TS[0];
    print!("{}", CLEAR_SCREEN);

    let mut str_buf: String = String::with_capacity(art.len() + (neg_line_num * -13) as usize);
    loop { // TODO: Should rework cursor jumping with ANSI save cursor position.
        let seq = itr.next().unwrap();
        for frame_offset in neg_line_num..max as isize {
            str_buf.clear();
            unsafe {print!("{}\x1b[H{:\n>hpad$}", CLEAR_SCREEN,"", hpad=TALL);}// clear screen and set cursor to home (0;0)
            for (line_offset, text) in art.lines().enumerate() {
                let mut rand_buf: [u8; 1] = [0];
                f.read_exact(&mut rand_buf[0..1]).unwrap();
                let rand: isize = (rand_buf[0] % 5) as isize - 2; //so it doesn't use new memory
                let offset: usize = if frame_offset + rand + (line_offset as isize) < 0 {
                    0
                } else if frame_offset + rand + (line_offset as isize) > text.len() as isize {
                    text.len()
                } else {
                    (frame_offset as isize + rand + line_offset as isize) as usize //all good
                };
                unsafe { // because of globals
                    write!(&mut str_buf, "\x1b[0m{:>vpad$}{}{}\x1b[0m{}{}\n","", seq,
                             &text[0..offset], earlier_seq, &text[offset..], vpad=WIDE).unwrap();
                }
            }
            print!("{}", str_buf);
            thread::sleep(std::time::Duration::from_millis(14)); // So terminal has time to clear -> artifacts
        }
        earlier_seq = seq;
    }
}

fn animate(art: Vec<String>) {
    print!("{}", CLEAR_SCREEN);
    loop {
        let art = &art;
        for i in art {
            thread::sleep(std::time::Duration::from_millis(300));
            print!("{}{}", i, CLEAR_SCREEN);
        }
    }
}
