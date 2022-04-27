extern crate airline;

use airline::classes::airplane_builder::AirplaneBuilder;
use airline::classes::seat::SeatState;
use airline::classes::seat::SeatClass;

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
