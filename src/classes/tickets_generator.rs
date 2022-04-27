use crate::classes::seat::SeatClass;

use std::fmt;

use rand::{
    distributions::{Distribution, Standard},
    Rng,
}; // 0.8.0

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum AirlanesNames {
    Lade,
    Flybondi,
    JetSmart,
    Aeromexico,
    AirEuropa,
    AmaszonasUruguay,
}

impl fmt::Display for AirlanesNames {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}


#[derive(Debug, Copy, Clone, PartialEq)]
pub enum Cities {
    BuenosAires,
    Bariloche,
    RioDeJaneiro,
    SantiagoDeChile,
    Lima,
    CiudadDeMexico,
    WashingtonDC,
    Barcelona,
    Roma,
}

impl fmt::Display for Cities {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

//todo move this file outside classes's folder

impl Distribution<SeatClass> for Standard {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> SeatClass {
        match rng.gen_range(0..=3) {
            0 => SeatClass::First,
            1 => SeatClass::Executive,
            2 => SeatClass::Economic,
            _ => SeatClass::Premium,
        }
    }
}

impl Distribution<AirlanesNames> for Standard {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> AirlanesNames {
        match rng.gen_range(0..=5) {
            0 => AirlanesNames::Lade,
            1 => AirlanesNames::Flybondi,
            2 => AirlanesNames::JetSmart,
            3 => AirlanesNames::Aeromexico,
            4 => AirlanesNames::AirEuropa,
            _ => AirlanesNames::AmaszonasUruguay,
        }
    }
}

impl Distribution<Cities> for Standard {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> Cities {
        match rng.gen_range(0..=8) {
            0 => Cities::BuenosAires,
            1 => Cities::Bariloche,
            2 => Cities::RioDeJaneiro,
            3 => Cities::SantiagoDeChile,
            4 => Cities::Lima,
            5 => Cities::CiudadDeMexico,
            6 => Cities::WashingtonDC,
            7 => Cities::Barcelona,
            _ => Cities::Roma,
        }
    }
}

//todo add test for the ticket generator

pub fn travel_generator() -> (Cities, Cities) {
    let outgoing_city: Cities = rand::random();
    let mut arrival_city: Cities = rand::random();
    while outgoing_city == arrival_city {
        arrival_city = rand::random();
    }
    (outgoing_city, arrival_city)
}

//todo change name to ticket_generator(), and make it a "class"
pub fn ticket() -> String {
    let seat_class_enum: SeatClass       = rand::random();
    let airlane_name_enum: AirlanesNames = rand::random();
    let (outgoing_city_enum, arrival_city_enum) = travel_generator();
    let seat_class    = seat_class_enum.to_string();
    let airlane_name  = airlane_name_enum.to_string();
    let outgoing_city = outgoing_city_enum.to_string();
    let arrival_city  = arrival_city_enum.to_string();
    let user_num: u32 = rand::random(); //todo, make for a number with 8 digits
    let user_id       = user_num.to_string();
    let seats_number  = rand::thread_rng().gen_range(1..=4).to_string();
    let ticket = format!("{},{},{},{},{},{}", airlane_name, outgoing_city, arrival_city, seat_class, seats_number, user_id);

    ticket
}

#[cfg(test)]
pub mod tests {
    use crate::classes::tickets_generator::{travel_generator};

    #[test]
    pub fn test_generate_different_cities() {
        let (outgoing_city, arrival_city) = travel_generator();
        assert_ne!(outgoing_city, arrival_city);
    }
}