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
