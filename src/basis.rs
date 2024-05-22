use std::marker::ConstParamTy;

use crate::{
    common::{even, odd},
    macros::{repeat, yeet},
    maybe::Maybe::{self, Just, Nothing},
    metric::{Metric, Square},
    sign::Sign,
};

/// Encodes the basis of a blade such that `A = B eᵢ` ⇔ `A[i]`
/// The encoding is chosen in a way that is order-independent.
/// Therefore the sign of the basis is stored separately.
#[derive(Debug, Clone, Copy, PartialEq, Eq, ConstParamTy)]
pub struct NonzeroBasis<const N: usize> {
    pub sign: Sign,
    pub unit: [bool; N],
}

impl<const N: usize> NonzeroBasis<N> {
    pub const ONE: NonzeroBasis<N> = NonzeroBasis {
        sign: Sign::Pos,
        unit: [false; N],
    };
    pub const I: NonzeroBasis<N> = NonzeroBasis {
        sign: Sign::Pos,
        unit: [true; N],
    };

    /// Parity of the reversion operator, rewriting its factors in reverse order.
    /// - `rev(eᵢⱼ) = eⱼᵢ = -eᵢⱼ` ⇔ `i ≠ j`
    pub const fn reverse(self) -> Self {
        let r = self.grade();
        if r > 0 && odd(r * (r - 1) / 2) {
            self.neg()
        } else {
            self
        }
    }

    /// Parity of the grade involution, reversing the sign of odd blades.
    pub const fn involute(self) -> Self {
        if even(self.grade()) {
            self
        } else {
            self.neg()
        }
    }

    /// Clifford Conjugate
    pub const fn conjugate(self) -> Self {
        self.reverse().involute()
    }

    pub const fn neg(self) -> Self {
        NonzeroBasis {
            sign: self.sign.neg(),
            unit: self.unit,
        }
    }

    /// Poincaré duality operator
    pub const fn dual(self) -> Self {
        let mut dual = [false; N];
        repeat!(i in 0..N {
            dual[i] = !self.unit[i];
        });
        NonzeroBasis {
            sign: self.sign,
            unit: dual,
        }
    }

    /// Compute the geometric product between two blades.
    /// - `eᵢeᵢ = 1`
    /// - `eᵢeⱼ = eᵢⱼ` ⇔ `i ≠ j`
    ///- `eᵢeⱼ = -eⱼeᵢ`
    pub const fn geometric(self, rhs: Self, metric: Metric<N>) -> Basis<N> {
        let mut product = [false; N];
        let mut sign = self.sign.mul(rhs.sign);
        repeat!(i in 0..N {
            if self.unit[i] {
                // Each permutation of the factors of the geometric product will reverse the sign.
                repeat!(j in 0..i {
                    if rhs.unit[j] {
                        sign = sign.neg();
                    }
                });
            }
            product[i] = match (self.unit[i], rhs.unit[i]) {
                (true, false) | (false, true) => true,
                (true, true) => match metric.0[i] {
                    Square::Pos => {
                        // eᵢeᵢ = 1
                        false
                    },
                    Square::Neg => {
                        // eᵢeᵢ = -1
                        sign = sign.neg();
                        false
                    },
                    Square::Zero => {
                        // eᵢeᵢ = 0
                        return Nothing
                    },
                }
                (false, false) => false,
            }
        });

        Just(NonzeroBasis {
            sign,
            unit: product,
        })
    }

    // Compute the exterior product between two blades.
    /// - `eᵢ ∧ eᵢ = 0`
    /// - `eᵢ ∧ eⱼ = eᵢⱼ` ⇔ `i ≠ j`
    ///- `eᵢ ∧ eⱼ = -eⱼeᵢ`
    pub const fn exterior(self, rhs: Self, metric: Metric<N>) -> Basis<N> {
        let product = yeet!(self.geometric(rhs, metric));
        if rhs.grade() + self.grade() == product.grade() {
            Just(product)
        } else {
            Nothing
        }
    }

    // Compute the regressive product between two blades using the identity
    /// `A ∨ B = J(J(A) ∧ J(B))`
    pub const fn regressive(self, rhs: Self, metric: Metric<N>) -> Basis<N> {
        Just(yeet!(self.dual().exterior(rhs.dual(), metric)).dual())
    }

    /// Contraction of `self` onto `rhs`.
    /// Intuitively, this returns the sub-blade of `rhs` which is prependicular to `self.
    pub const fn left_contraction(self, rhs: Self, metric: Metric<N>) -> Basis<N> {
        let product = yeet!(self.geometric(rhs, metric));
        if let Some(q) = rhs.grade().checked_sub(self.grade())
            && q == product.grade()
        {
            Just(product)
        } else {
            Nothing
        }
    }

    /// Contraction of `self` by `rhs`.
    /// `A << B = (B~ >> A~)~`
    /// Intuitively, this returns the sub-blade of `self` which is prependicular to `rhs.
    pub const fn right_contraction(self, rhs: Self, metric: Metric<N>) -> Basis<N> {
        Just(yeet!(rhs.reverse().left_contraction(self.reverse(), metric)).reverse())
    }

    /// Bi-directional contraction.
    pub const fn inner(self, rhs: Self, metric: Metric<N>) -> Basis<N> {
        let product = yeet!(self.geometric(rhs, metric));
        if rhs.grade().abs_diff(self.grade()) == product.grade() {
            Just(product)
        } else {
            Nothing
        }
    }

    /// Scalar product, producing non-zero scalars only when grades match.
    /// In that case, the result can be interpreted as a metric between blades:
    /// `A~ * A` can be used as the squared norm of `A`.
    pub const fn scalar(self, rhs: Self, metric: Metric<N>) -> Basis<N> {
        let product = yeet!(self.geometric(rhs, metric));
        if product.grade() == 0 {
            Just(product)
        } else {
            Nothing
        }
    }

    /// The *grade* (Sometimes also called *step*) of this blade, equating to the number of distinct factors.
    pub const fn grade(self) -> usize {
        let mut grade = 0;
        repeat!(i in 0..N {
            if self.unit[i] {
                grade += 1;
            }
        });
        grade
    }

    pub const fn anti_grade(self) -> usize {
        N - self.grade()
    }
}

impl<const N: usize> std::fmt::Display for NonzeroBasis<N> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self.sign)?;
        if self.grade() == N {
            write!(f, "i")?;
        } else {
            write!(f, "e")?;
            for i in 0..N {
                if self.unit[i] {
                    write!(f, "{i}")?;
                }
            }
        }
        Ok(())
    }
}

/// A blade basis which can also vanish.
/// This is essentially just used because Rust currently does not properly support
/// using `?` so we have a [yeet] macro, and because we cannot even use that
/// in const arguments.
pub type Basis<const N: usize> = Maybe<NonzeroBasis<N>>;

impl<const N: usize> std::fmt::Display for Basis<N> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Nothing => write!(f, "0"),
            Just(b) => write!(f, "{}", b),
        }
    }
}

impl<const N: usize> Basis<N> {
    pub const fn geometric(self, rhs: Self, metric: Metric<N>) -> Basis<N> {
        yeet!(self).geometric(yeet!(rhs), metric)
    }

    pub const fn exterior(self, rhs: Self, metric: Metric<N>) -> Basis<N> {
        yeet!(self).exterior(yeet!(rhs), metric)
    }

    pub const fn regressive(self, rhs: Self, metric: Metric<N>) -> Basis<N> {
        yeet!(self).regressive(yeet!(rhs), metric)
    }

    pub const fn left_contraction(self, rhs: Self, metric: Metric<N>) -> Basis<N> {
        yeet!(self).left_contraction(yeet!(rhs), metric)
    }

    pub const fn right_contraction(self, rhs: Self, metric: Metric<N>) -> Basis<N> {
        yeet!(self).right_contraction(yeet!(rhs), metric)
    }

    pub const fn inner(self, rhs: Self, metric: Metric<N>) -> Basis<N> {
        yeet!(self).inner(yeet!(rhs), metric)
    }

    pub const fn scalar(self, rhs: Self, metric: Metric<N>) -> Basis<N> {
        yeet!(self).scalar(yeet!(rhs), metric)
    }

    pub const fn grade(self) -> Maybe<usize> {
        Just(yeet!(self).grade())
    }

    pub const fn anti_grade(self) -> Maybe<usize> {
        Just(yeet!(self).anti_grade())
    }

    pub const fn reverse(self) -> Self {
        Just(yeet!(self).reverse())
    }
}
