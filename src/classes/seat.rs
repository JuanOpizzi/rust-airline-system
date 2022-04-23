#[derive(Debug, Copy, Clone)]
pub enum SeatState {
    Occupied,
    Free,
    Disabled,
}

#[derive(Debug, Copy, Clone)]
pub enum SeatClass {
    First,
    Executive,
    Economic,
    Premium,
}

pub struct Seat {
    position: String,
    state: String,
    class: String,
}


impl Seat {
    pub fn new(position: String, state: SeatState, class: SeatClass) -> Seat {
        let my_position = position;
        let my_state = Seat::match_state(state);
        let my_class = Seat::match_class(class);

        Seat {
            position: my_position,
            state: my_state,
            class: my_class
        }
    }

    pub fn get_state(&self) -> String {
        self.state.clone()
    }

    pub fn get_class(&self) -> String {
        self.class.clone()
    }


    pub fn get_position(&self) -> String {
        self.position.clone()
    }

    pub fn match_state(state: SeatState) -> String {
        match state {
            SeatState::Occupied => return String::from("Occupied"),
            SeatState::Free     => return String::from("Free"),
            SeatState::Disabled => return String::from("Disabled"),
        }
    }

    pub fn match_class(class: SeatClass) -> String {
        match class {
            SeatClass::First     => return String::from("First Class"),
            SeatClass::Executive => return String::from("Executive Class"),
            SeatClass::Economic  => return String::from("Economic Class"),
            SeatClass::Premium   => return String::from("Premium Class"),
        }
    }

    pub fn change_state(&mut self, new_state: SeatState) {
        self.state = Seat::match_state(new_state);
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
        assert_eq!(seat.get_state(), "Free".to_string());
    }

    #[test]
    pub fn test_create_economic_seat() {
        let seat = Seat::new("11-11".to_string(), SeatState::Free, SeatClass::Economic);
        assert_eq!(seat.get_class(), "Economic Class".to_string());
    }

    #[test]
    pub fn test_create_premium_seat() {
        let premium_seat = Seat::new("11-11".to_string(), SeatState::Free, SeatClass::Premium);
        assert_eq!(premium_seat.get_class(), "Premium Class".to_string());
    }

    #[test]
    pub fn test_change_seat_to_occupied() {
        let mut occupied_seat = Seat::new("11-11".to_string(), SeatState::Free, SeatClass::Premium);
        occupied_seat.change_state(SeatState::Occupied);
        assert_eq!(occupied_seat.get_state(), "Occupied".to_string());
    }

    #[test]
    pub fn test_change_seat_to_disabled() {
        let mut disabled_seat = Seat::new("11-11".to_string(), SeatState::Free, SeatClass::Premium);
        disabled_seat.change_state(SeatState::Disabled);
        assert_eq!(disabled_seat.get_state(), "Disabled".to_string());
    }

    #[test]
    pub fn test_valid_second_seat() {
        let seat = Seat::new("11-12".to_string(), SeatState::Free, SeatClass::Economic);
        assert_eq!(seat.get_position(), "11-12".to_string());
    }

}