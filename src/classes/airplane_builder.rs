use crate::classes::airplane::Airplane;
use crate::classes::seat::SeatClass;
use crate::classes::seat::SeatState;
use crate::classes::seat::Seat;
use std::collections::HashMap;

pub struct AirplaneBuilder {
    pub name: String,
    pub id: String,
    pub state: String,
    pub seats: HashMap<String, Seat>,
    nro_rows: u32,
}

impl AirplaneBuilder {
    pub fn new() -> Self {
        AirplaneBuilder {
            name:   String::from("Boeing 747-400"), //todo change
            id:     String::from("0001"), //todo change
            state:  String::from("Free"),
            seats: HashMap::new(),
            nro_rows: 1,
        }
    }

    pub fn build(self) -> Airplane {
        Airplane::new(self.name, self.id, self.state, self.seats, self.nro_rows)
    }

    pub fn with_seats(mut self, rows_ends: u32, cols: u32, seat_state: SeatState, seat_class: SeatClass) -> Self {
        for i in self.nro_rows..= (self.nro_rows + rows_ends - 1) {
            for j in 1..= cols {
                let seat_number = format!("{}-{}", i, j);
                let seat = Seat::new(seat_number, seat_state, seat_class);
                self.add_seat(seat);
            }
        }
        self.nro_rows += rows_ends;
        self
    }

    pub fn add_seat(&mut self, seat: Seat) {
        self.seats.insert(seat.get_position(), seat);
    }
}

#[cfg(test)]
pub mod boeing_tests {
    use crate::classes::airplane_builder::AirplaneBuilder;
    use crate::classes::seat::SeatState;
    use crate::classes::seat::SeatClass;

    #[test]
    pub fn test_create_airplane_builder_with_200_seats() {
        let builder = AirplaneBuilder::new()
            // 20 First Class seats
            .with_seats(5, 4, SeatState::Free, SeatClass::First)
            // 180 Excecutive seats
            .with_seats(45, 4, SeatState::Free, SeatClass::Executive);
        let boeing = builder.build();
        assert_eq!(boeing.number_of_seats(), 200);
    }

    #[test]
    pub fn test_airplane_has_50_rows() {
        let builder = AirplaneBuilder::new()
            // 20 First Class seats
            .with_seats(5, 4, SeatState::Free, SeatClass::First)
            // 180 Excecutive seats
            .with_seats(45, 4, SeatState::Free, SeatClass::Executive);
        let boeing = builder.build();
        assert_eq!(boeing.number_of_rows(), 50 + 1);
    }
}