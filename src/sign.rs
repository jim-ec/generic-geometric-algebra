#[derive(PartialEq, Eq, Debug, Clone, Copy)]
pub enum Sign {
    Pos,
    Neg,
}

impl Sign {
    pub const fn neg(self) -> Sign {
        match self {
            Sign::Pos => Sign::Neg,
            Sign::Neg => Sign::Pos,
        }
    }

    pub const fn mul(self, rhs: Sign) -> Sign {
        match (self, rhs) {
            (Sign::Pos, Sign::Pos) => Sign::Pos,
            (Sign::Pos, Sign::Neg) => Sign::Neg,
            (Sign::Neg, Sign::Pos) => Sign::Neg,
            (Sign::Neg, Sign::Neg) => Sign::Pos,
        }
    }
}

impl std::ops::Mul<f64> for Sign {
    type Output = f64;

    fn mul(self, rhs: f64) -> Self::Output {
        match self {
            Sign::Pos => rhs,
            Sign::Neg => -rhs,
        }
    }
}

impl std::fmt::Display for Sign {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Sign::Pos => Ok(()),
            Sign::Neg => write!(f, "-"),
        }
    }
}
