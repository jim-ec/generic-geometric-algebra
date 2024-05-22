use std::marker::ConstParamTy;

#[derive(Clone, Copy, Debug, PartialEq, Eq, ConstParamTy)]
pub struct Metric<const N: usize>(pub [Square; N]);

#[derive(Clone, Copy, Debug, PartialEq, Eq, ConstParamTy)]
pub enum Square {
    Pos,
    Neg,
    Zero,
}

impl<const N: usize> std::fmt::Display for Metric<N> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        for (i, square) in self.0.into_iter().enumerate() {
            write!(
                f,
                "e{i}²={} ",
                match square {
                    Square::Pos => "1",
                    Square::Neg => "-1",
                    Square::Zero => "0",
                }
            )?;
        }
        Ok(())
    }
}
