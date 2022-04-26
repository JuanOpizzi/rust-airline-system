use std::fs::File;
use std::io::{prelude::*};

mod classes;
use classes::tickets_generator::{ticket};

const LINES_TO_WRITE: i32 = 20;

fn main() {
    /*let input = File::open("./airlines.txt").unwrap();
    //let buffered = BufReader::new(input);
    //todo implement by the funcitonal way
    for line in buffered.lines() {
        //todo init Airlines
        println!("{}", line.unwrap());
    }*/
    let _ = file_writer();

}

fn file_writer() -> std::io::Result<()> {
    //? Do I need to close the file?
    let mut file = File::create("data/tickets.txt").unwrap(); //todo change unwrap
    for _ in 1..=LINES_TO_WRITE {
        let ticket = ticket();
        writeln!(file, "{}", ticket).unwrap();
    }
    Ok(())
}