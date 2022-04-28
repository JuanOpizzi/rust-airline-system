extern crate airline;


use airline::classes::seat::Seat;
use airline::classes::seat::SeatState;
use airline::classes::seat::SeatClass;

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