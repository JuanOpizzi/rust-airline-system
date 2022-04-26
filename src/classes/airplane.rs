use std::collections::HashMap;


use crate::classes::seat::Seat;

pub struct Airplane {
    pub name: String,
    pub id: String,
    pub state: String,
    pub seats: HashMap<String, Seat>,
    pub nro_rows: u32,
}

impl Airplane {
    pub fn new(name: String, id: String, state: String, seats: HashMap<String, Seat>, nro_rows: u32) -> Airplane {
        Airplane {
            name,
            id,
            state, //ToDo: add start and departure date
            seats,
            nro_rows,
        }
    }

    #[allow(dead_code)]
    pub fn add_seat(&mut self, seat: Seat) {
        self.seats.insert(seat.get_position(), seat);
    }

    pub fn number_of_seats(&self) -> usize {
        return self.seats.keys().len();
    }

    pub fn number_of_rows(&self) -> u32 {
        return self.nro_rows;
    }
}
