pub struct Metric<const N: usize>(pub [Square; N]);

pub enum Square {
    Pos,
    Neg,
    Zero,
}
