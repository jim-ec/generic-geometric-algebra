use crate::{
    common::pow,
    metric::{Metric, Square},
};

/// A geometric algebra:
/// - `P`: Positive dimensions
/// - `Q`: Negative dimensions
/// - `R`: Degenerate dimensions
#[derive(Clone, Copy, Debug)]
pub struct Algebra<const P: usize, const Q: usize, const R: usize>();

pub type Real = Algebra<0, 0, 0>;
pub type Hyperbolic = Algebra<1, 0, 0>;
pub type Complex = Algebra<0, 1, 0>;
pub type Dual = Algebra<0, 0, 1>;
pub type VGA2 = Algebra<2, 0, 0>;
pub type VGA3 = Algebra<3, 0, 0>;
pub type PGA2 = Algebra<2, 0, 1>;
pub type PGA3 = Algebra<3, 0, 1>;
pub type CGA2 = Algebra<3, 1, 0>;
pub type CGA3 = Algebra<4, 1, 0>;

impl<const P: usize, const Q: usize, const R: usize> Algebra<P, Q, R> {
    pub const DIM: usize = P + Q + R;
    pub const BASIS_BLADE_COUNT: usize = pow(2, Self::DIM);

    pub const fn metric() -> Metric<{ Self::DIM }> {
        let mut squares = [Square::Pos; Self::DIM];
        repeat!(i in P..{P + Q} {
            squares[i] = Square::Neg;
        });
        repeat!(i in {P + Q}..{Self::DIM} {
            squares[i] = Square::Zero;
        });
        Metric(squares)
    }
}

// #[derive(Clone, Copy, Debug)]
// pub struct Algebra(pub usize, pub usize, pub usize);
// pub const Complex: Algebra = Algebra(0, 1, 0);

// impl Algebra {
//     pub const fn dim(self) -> usize {
//         self.0 + self.1 + self.2
//     }

//     pub type Metric = Metric<{ Algebra::dim(self) }>;
// }
