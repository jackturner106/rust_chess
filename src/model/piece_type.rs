#[derive(PartialEq, Eq, Debug, Copy, Clone)]
pub(crate) enum PieceType {
    Pawn,
    Rook,
    Knight,
    Bishop,
    King,
    Queen,
    Empty,
}
