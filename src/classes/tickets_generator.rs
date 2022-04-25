use crate::classes::seat::SeatClass;

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
