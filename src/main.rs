use std::fs::File;
use std::io::{prelude::*, BufReader};

mod classes;
use classes::seat::SeatClass;
use classes::tickets_generator::{AirlanesNames, Cities};

use classes::tickets_generator::travel_generator;

fn main() {
    let input = File::open("./airlines.txt").unwrap();

    //todo implement by the funcitonal way
    let buffered = BufReader::new(input);

    for line in buffered.lines() {
        //todo init Airlines
        println!("{}", line.unwrap());
    }
    println!("---------");
    for i in 1..=20 {
        let seat_class_enum: SeatClass    = rand::random();
        let a = seat_class_enum.to_string();
        println!("aaaaa: {:?}", a);
        let airlane_name: AirlanesNames   = rand::random();
        let (outgoing_city, arrival_city) = travel_generator();
        println!("-----TICKET {:?}-----", i);
        println!("seat: {:?}", seat_class_enum);
        println!("Airplane: {:?}", airlane_name);
        println!("Outgoing city: {:?}", outgoing_city);
        println!("Arrival city: {:?}", arrival_city);
    }
    println!("---------");
    let user_id: u32 = rand::random(); //todo, make for a number with 8 digits

    println!("{:?}", user_id);
}
