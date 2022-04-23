use crate::classes::airplane::Airplane;
use crate::classes::seat::SeatClass;
use crate::classes::seat::SeatState;
use std::collections::HashMap;

pub struct Boeing {
    airplane: Airplane
}

impl Boeing {
    pub fn new() -> Airplane {
        let mut boeing = Airplane {
            name:   "Boeing 747-400".to_string(),
            id:     "0001".to_string(), //todo change
            state:  "Free".to_string(),
            seats: HashMap::new(),
        };

        // 20 First Class seats
        boeing.add_row_of_seats(1, 5, 4, SeatState::Free, SeatClass::First);
        // 180 Excecutive seats
        boeing.add_row_of_seats(6, 50, 4, SeatState::Free, SeatClass::Executive);

        boeing
    }
}

#[cfg(test)]
pub mod boeing_tests {
    use crate::classes::boeing_builder::Boeing;

    #[test]
    pub fn test_create_boeing_with_all_seats() {
        let boeing = Boeing::new();
        assert_eq!(boeing.number_of_seats(), 200);
    }
}