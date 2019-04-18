//Parse module
/* Parses a given file and returns the frames */

use std::path::Path;
use std::fs;

pub fn get_art(path: &Path, index: usize) -> Chunk {
    let buffer = fs::read_to_string(path).expect("File can't be opened or missing");
    let target = buffer.split("::separator::").nth(index).expect("Index out of bounds");
    if let true = target.contains("::frame::") {
        Chunk::Moving(target.split("::frame::").map(|x| x.to_owned()).collect())
    } else {
        Chunk::Picture(target.to_owned())
    }
}
pub enum Chunk {
    Picture(String), //if it's a still image
    Moving(Vec<String>), //if it should be iterated through (ASCII animation)
}
