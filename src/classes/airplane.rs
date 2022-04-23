use std::collections::HashMap;


use crate::classes::seat::Seat;
use crate::classes::seat::SeatClass;
use crate::classes::seat::SeatState;

pub struct Airplane {
    pub name: String,
    pub id: String,
    pub state: String,
    pub seats: HashMap<String, Seat>,
}

impl Airplane {
    pub fn new(name: String, id: String, state: String) -> Airplane {
        Airplane {
            name,
            id,
            state, // falta fecha de inicio y de salida
            seats: HashMap::new(),
        }
    }

    pub fn add_seat(&mut self, seat: Seat) {
        self.seats.insert(seat.get_position(), seat);
    }

    pub fn number_of_seats(&self) -> usize {
        return self.seats.keys().len();
    }

    pub fn add_row_of_seats(&mut self, rows_starts: u32, rows_ends: u32, cols: u32, seat_state: SeatState, seat_class: SeatClass) {
         for i in rows_starts..= rows_ends {
            for j in 1..= cols {
                let seat_number = format!("{}-{}", i, j);
                let seat = Seat::new(seat_number, seat_state, seat_class);
                self.add_seat(seat);
            }
        }
    }
}
