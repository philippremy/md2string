use std::env;
use std::fs::File;
use std::io::{BufRead, BufReader, Read};
use std::path::PathBuf;
use std::process::exit;
use arboard::Clipboard;

fn main() {

    let args: Vec<String> = env::args().collect();
    if args.len() != 3 {
        eprintln!("Wrong number of command line arguments.\nExpected input is md2string --i <PATH_TO_MD_FILE>");
        exit(0);
    }
    let path = PathBuf::from(args.get(2).unwrap());
    if !path.exists() {
        eprintln!("File specified does not exist.");
        exit(0);
    }
    let md_file = match File::options().read(true).write(false).open(path) {
        Ok(file) => file,
        Err(err) => {
            eprintln!("Could not open MD file: {:?}", err);
            exit(-1);
        }
    };
    let mut file_buf_reader = BufReader::new(md_file);
    let mut markdown_string: String = String::new();
    for line in file_buf_reader.lines() {
        let mut line = line.unwrap();
        if line.len() != 0 {
            line.truncate(line.len());
        }
        line.push_str(r"\n");
        markdown_string.push_str(line.as_str());
    }
    let mut clipboard = match Clipboard::new() {
        Ok(clip) => clip,
        Err(err) => {
            eprintln!("Failed to get the OS clipboard: {:?}", err);
            exit(-1);
        }
    };
    match clipboard.set_text(markdown_string.clone()) {
        Ok(()) => {},
        Err(err) => {
            eprintln!("Failed to set Text to the Clipboard. Trying to print to the screen only... (Error: {:?})", err);
        }
    }
    println!("{}", markdown_string.clone());
}
