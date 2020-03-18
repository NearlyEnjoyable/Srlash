//Parse module
/* Parses a given file and returns the frames */

use std::path::{Path, PathBuf};
use std::fs;

use clap::{Arg, App};
use clap::{crate_version, crate_authors, crate_description};

pub fn cli() -> (PathBuf, usize, bool) {
    let cli = App::new("Srlash")
        .version(crate_version!()) /* Macro to get cargo crate info. No need to update here */
        .author(crate_authors!())
        .about(crate_description!())
        .arg(Arg::with_name("Cache")
            .short("c")
            .long("cache")
            .help("Enables caching thus improving performance. Recommended"))
        .arg(Arg::with_name("File")
            .short("f")
            .long("file")
            .required(true)
            .index(1)
            .takes_value(true)
            .value_name("FILE")
            .help("Specifies input file"))
        .arg(Arg::with_name("Index")
            .short("i")
            .long("index")
            .required(true)
            .index(2)
            .takes_value(true)
            .value_name("INDEX")
            .help("Index art number from set file. Must be number"))
        .get_matches();

    let mut file = PathBuf::new();
    file.push(cli.value_of("File").unwrap());

    if !file.is_absolute() {
        /* Then make it absolute! */
        let mut tmp_file = PathBuf::new();
        tmp_file.push(String::from(format!("{}{}", std::env::current_dir().unwrap()
                                        .to_str().unwrap(), file.to_str().unwrap())));
        file = tmp_file;
    }

    let index = cli.value_of("Index").unwrap().parse().expect("Please use a number value for indexing.");

    (file, index, cli.is_present("Cache"))
}

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
