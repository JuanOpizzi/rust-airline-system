use std::fmt;

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum SeatState {
    Occupied,
    Free,
    Disabled,
}

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum SeatClass {
    First,
    Executive,
    Economic,
    Premium,
}

impl fmt::Display for SeatClass {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

pub struct Seat {
    position: String,
    state: SeatState,
    class: SeatClass,
}


impl Seat {
    pub fn new(position: String, state: SeatState, class: SeatClass) -> Seat {
        let my_position = position;

        Seat {
            position: my_position,
            state,
            class
        }
    }

    pub fn get_state(&self) -> SeatState {
        self.state.clone()
    }

    pub fn get_class(&self) -> SeatClass {
        self.class.clone()
    }


    pub fn get_position(&self) -> String {
        self.position.clone()
    }

    pub fn change_state(&mut self, new_state: SeatState) {
        self.state = new_state;
    }

}


#[cfg(test)]
pub mod tests {
    use crate::classes::seat::Seat;
    use crate::classes::seat::SeatState;
    use crate::classes::seat::SeatClass;

    #[test]
    pub fn test_create_free_seat() {
        let seat = Seat::new("11-11".to_string(), SeatState::Free, SeatClass::Economic);
        assert_eq!(seat.get_state(), SeatState::Free);
    }

    #[test]
    pub fn test_create_economic_seat() {
        let seat = Seat::new("11-11".to_string(), SeatState::Free, SeatClass::Economic);
        assert_eq!(seat.get_class(), SeatClass::Economic);
    }

    #[test]
    pub fn test_create_premium_seat() {
        let premium_seat = Seat::new("11-11".to_string(), SeatState::Free, SeatClass::Premium);
        assert_eq!(premium_seat.get_class(), SeatClass::Premium);
    }

    #[test]
    pub fn test_change_seat_to_occupied() {
        let mut occupied_seat = Seat::new("11-11".to_string(), SeatState::Free, SeatClass::Premium);
        occupied_seat.change_state(SeatState::Occupied);
        assert_eq!(occupied_seat.get_state(), SeatState::Occupied);
    }

    #[test]
    pub fn test_change_seat_to_disabled() {
        let mut disabled_seat = Seat::new("11-11".to_string(), SeatState::Free, SeatClass::Premium);
        disabled_seat.change_state(SeatState::Disabled);
        assert_eq!(disabled_seat.get_state(), SeatState::Disabled);
    }

    #[test]
    pub fn test_valid_second_seat() {
        let seat = Seat::new("11-12".to_string(), SeatState::Free, SeatClass::Economic);
        assert_eq!(seat.get_position(), "11-12".to_string());
    }

}