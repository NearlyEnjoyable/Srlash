/*
    Srlash - Splash screen written in Rust
    Colored output
    Reads file as frames which are separated.
    It's kinda shitty because it clears every time
    Can be used as bootsplash but migh need to configur
        tty colors and fonts to look good.

    Arguments: file; index of file (Maybe more?: looping time)
*/

use std::process::Command;
use std::thread;
use std::sync::mpsc::{Receiver, sync_channel};
use std::io::Read;
use std::fmt::Write;
use std::process::Stdio;
//use std::env; //for future use

mod parse;

const TS: [&str; 18] = /* Terminal Sequences */ [
    "\x1b[0m", // All attributes off
    "\x1b[38;5;0m", // Terminal set black
    "\x1b[38;5;1m", // dark red
    "\x1b[38;5;2m", // darkgreen
    "\x1b[38;5;3m", // dark yellow
    "\x1b[38;5;4m", // dark blue
    "\x1b[38;5;5m", // dark purple
    "\x1b[38;5;6m", // dark cyan
    "\x1b[38;5;7m", // light gray
    "\x1b[8m", // THIS sets hidden attribute
    "\x1b[38;5;8m", // grey <-^ see the difference?
    "\x1b[38;5;9m", // red
    "\x1b[38;5;10m",// green
    "\x1b[38;5;11m",// yellow
    "\x1b[38;5;12m",// blue
    "\x1b[38;5;13m",// purple
    "\x1b[38;5;14m",// cyan
    "\x1b[38;5;15m",// white
];

static mut WIDE: usize = 0;
static mut TALL: usize = 0;
static mut RX_PONTER: *const Receiver<bool> = 0 as *const Receiver<bool>; // Why am I doing this??

fn main() {
    //let args = env::args();
    let (tx, rx) = sync_channel::<bool>(0);
    thread::spawn(move|| { //get and assign terminal size
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
        tx.send(true).unwrap();
    });
    unsafe {
        RX_PONTER = &rx as *const Receiver<bool>;
    }
    let path = std::path::Path::new("/root/.config/srlash"); //The file to be read TODO: env arg
    let art = parse::get_art(path, 3); //Get the desired art
    println!("\x1b[?25l"); // Hide terminal cursor
    match art {
        parse::Chunk::Picture(x) => draw(&x),
        parse::Chunk::Moving(y) => animate(y),
    }
    println!("\x1b[?25h"); // Show cursor
    Command::new("clear").spawn().expect("Program paniced"); // Call "clear" -> no artifacts
    
}

fn draw(art: &str) {
    let mut f = std::fs::File::open("/dev/urandom").unwrap(); //get random number !UNIX-ONLY!
    let max = art.lines().map(|x| x.len()).max().unwrap();
    let neg_line_num = art.lines().count() as isize * -1 ;
    let mut itr = TS.iter().cycle();
    let mut earlier_seq = TS[0];
    print!("\x1b[2J");
    unsafe {
        (*RX_PONTER).recv().unwrap(); // All of this to not pass WIDE and TALL. Worth it.
        WIDE = ((WIDE as isize -max as isize) / 2) as usize;
        TALL = ((TALL as isize - (neg_line_num * -1)) / 2) as usize;
    }
    loop {
        let seq = itr.next().unwrap();
        for frame_offset in neg_line_num..max as isize {
            let mut str_buf: String = String::with_capacity(art.len() + (neg_line_num * -13) as usize);
            unsafe {print!("\x1b[2J\x1b[H{:\n>hpad$}","", hpad=TALL);}// clear screen and set cursor to home (0;0)
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
                             &text[0..offset], earlier_seq, &text[offset..], vpad=WIDE);
                }
            }
            print!("{}", str_buf);
            thread::sleep(std::time::Duration::from_millis(14)); // So terminal has time to clear -> artifacts
        }
        earlier_seq = seq;
    }
}

fn animate(art: Vec<String>) {
    print!("\x1b[2J");
    loop {
        let art = &art;
        for i in art {
            thread::sleep(std::time::Duration::from_millis(300));
            print!("{}", i); //this formatting is the way to go
            print!("\x1b[2J"); //clears screen
        }
    }
}
