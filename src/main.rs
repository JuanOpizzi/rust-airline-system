use std::fs::File;
use std::io::{prelude::*};

mod classes;
use classes::tickets_generator::{ticket};

const LINES_TO_WRITE: i32 = 1000;
const FILES_TO_WRITE: i32 = 5;

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
    let file_range = 0..FILES_TO_WRITE;
    file_range.into_iter().map(|i| {
        format!("data/tickets_{}.txt", i)
    })
    .flat_map(File::create)
    .for_each(|mut file|{
        for _ in 1..=LINES_TO_WRITE {
            let ticket = ticket();
            writeln!(file, "{}", ticket);
        }
    });
    Ok(())
}