#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ParseError {
    BidLevelOutOfBounds,
    TooShort,
    TooLong,
    BidLevelNotAnInteger,
    SuitNotValid,
    RankNotValid,
    SeatNotValid,
}
