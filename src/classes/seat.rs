#[derive(Debug, Copy, Clone)]
pub enum SeatState {
    Ocupied,
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

//#[derive(Clone)]
pub struct Seat {
    position: String,
    state: String,
    class: String,
}


impl Seat {
    pub fn new(position: String, state: SeatState, class: SeatClass) -> Seat {
        let my_position = position;
        let my_state;
        let my_class;

        match state {
            SeatState::Ocupied  => my_state = String::from("Ocupied"),
            SeatState::Free     => my_state = String::from("Free"),
            SeatState::Disabled => my_state = String::from("Disabled"),
        }

        match class {
            SeatClass::First     => my_class = String::from("First Class"),
            SeatClass::Executive => my_class = String::from("Executive Class"),
            SeatClass::Economic  => my_class = String::from("Economic Class"),
            SeatClass::Premium   => my_class = String::from("Premium Class"),
        }

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
    pub fn test_create_second_seat() {
        let seat = Seat::new("11-12".to_string(), SeatState::Free, SeatClass::Economic);
        assert_eq!(seat.get_position(), "11-12".to_string());
    }

}