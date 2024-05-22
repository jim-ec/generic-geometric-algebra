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
pub struct Basis<const N: usize>(pub Maybe<(Sign, [bool; N])>);

impl<const N: usize> Basis<N> {
    pub const ZERO: Basis<N> = Basis(Nothing);
    pub const ONE: Basis<N> = Basis(Just((Sign::Pos, [false; N])));
    pub const I: Basis<N> = Basis(Just((Sign::Pos, [true; N])));

    /// Parity of the reversion operator, rewriting its factors in reverse order.
    /// - `rev(eᵢⱼ) = eⱼᵢ = -eᵢⱼ` ⇔ `i ≠ j`
    pub const fn reverse(self) -> Basis<N> {
        if let Just(r) = self.grade()
            && r > 0
            && odd(r * (r - 1) / 2)
        {
            self.neg()
        } else {
            self
        }
    }

    /// Parity of the grade involution, reversing the sign of odd blades.
    pub const fn involute(self) -> Basis<N> {
        if let Just(r) = self.grade()
            && even(r)
        {
            self
        } else {
            self.neg()
        }
    }

    /// Clifford Conjugate
    pub const fn conjugate(self) -> Basis<N> {
        self.reverse().involute()
    }

    pub const fn neg(self) -> Basis<N> {
        let Just((sign, factors)) = self.0 else {
            return Basis::ZERO;
        };
        Basis(Just((sign.neg(), factors)))
    }

    /// Poincaré duality operator
    pub const fn dual(self) -> Basis<N> {
        let Just(rhs) = self.0 else {
            return Basis::ZERO;
        };
        let mut dual = [false; N];
        repeat!(i in 0..N {
            dual[i] = !rhs.1[i];
        });
        Basis(Just((rhs.0, dual)))
    }

    /// Compute the geometric product between two blades.
    /// - `eᵢeᵢ = 1`
    /// - `eᵢeⱼ = eᵢⱼ` ⇔ `i ≠ j`
    ///- `eᵢeⱼ = -eⱼeᵢ`
    pub const fn geometric(self, rhs: Basis<N>, metric: Metric<N>) -> Basis<N> {
        let Just(lhs) = self.0 else {
            return Basis::ZERO;
        };
        let Just(rhs) = rhs.0 else { return Basis::ZERO };
        let mut product = [false; N];
        let mut sign = lhs.0.mul(rhs.0);
        repeat!(i in 0..N {
            if lhs.1[i] {
                // Each permutation of the factors of the geometric product will reverse the sign.
                repeat!(j in 0..i {
                    if rhs.1[j] {
                        sign = sign.neg();
                    }
                });
            }
            product[i] = match (lhs.1[i], rhs.1[i]) {
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
                        return Basis::ZERO
                    },
                }
                (false, false) => false,
            }
        });
        Basis(Just((sign, product)))
    }

    // Compute the exterior product between two blades.
    /// - `eᵢ ∧ eᵢ = 0`
    /// - `eᵢ ∧ eⱼ = eᵢⱼ` ⇔ `i ≠ j`
    ///- `eᵢ ∧ eⱼ = -eⱼeᵢ`
    pub const fn exterior(self, rhs: Basis<N>, metric: Metric<N>) -> Basis<N> {
        let product = self.geometric(rhs, metric);

        if let Just(r) = rhs.grade()
            && let Just(l) = self.grade()
            && let Just(p) = product.grade()
            && r + l == p
        {
            product
        } else {
            Basis::ZERO
        }
    }

    // Compute the regressive product between two blades using the identity
    /// `A ∨ B = J(J(A) ∧ J(B))`
    pub const fn regressive(self, rhs: Basis<N>, metric: Metric<N>) -> Basis<N> {
        self.dual().exterior(rhs.dual(), metric).dual()
    }

    /// Contraction of `self` onto `rhs`.
    /// Intuitively, this returns the sub-blade of `rhs` which is prependicular to `self.
    pub const fn left_contraction(self, rhs: Basis<N>, metric: Metric<N>) -> Basis<N> {
        let product = self.geometric(rhs, metric);

        if let Just(r) = rhs.grade()
            && let Just(l) = self.grade()
            && let Just(p) = product.grade()
            && let Some(q) = r.checked_sub(l)
            && q == p
        {
            product
        } else {
            Basis::ZERO
        }
    }

    /// Contraction of `self` by `rhs`.
    /// `A << B = (B~ >> A~)~`
    /// Intuitively, this returns the sub-blade of `self` which is prependicular to `rhs.
    pub const fn right_contraction(self, rhs: Basis<N>, metric: Metric<N>) -> Basis<N> {
        rhs.reverse()
            .left_contraction(self.reverse(), metric)
            .reverse()
    }

    /// Bi-directional contraction.
    pub const fn inner(self, rhs: Basis<N>, metric: Metric<N>) -> Basis<N> {
        let product = self.geometric(rhs, metric);

        if let Just(r) = rhs.grade()
            && let Just(l) = self.grade()
            && let Just(p) = product.grade()
            && r.abs_diff(l) == p
        {
            product
        } else {
            Basis::ZERO
        }
    }

    /// Scalar product, producing non-zero scalars only when grades match.
    /// In that case, the result can be interpreted as a metric between blades:
    /// `A~ * A` can be used as the squared norm of `A`.
    pub const fn scalar(self, rhs: Basis<N>, metric: Metric<N>) -> Basis<N> {
        let product = self.geometric(rhs, metric);
        if let Just(p) = product.grade()
            && p == 0
        {
            product
        } else {
            Basis::ZERO
        }
    }

    /// The *grade* (Sometimes also called *step*) of this blade, equating to the number of distinct factors.
    /// Returns [Nothing] if this basis vanishes.
    pub const fn grade(self) -> Maybe<usize> {
        let (_, factors) = yeet!(self.0);
        let mut grade = 0;
        repeat!(i in 0..N {
            if factors[i] {
                grade += 1;
            }
        });
        Just(grade)
    }

    pub const fn anti_grade(self) -> Maybe<usize> {
        Just(N - yeet!(self.grade()))
    }
}

impl<const N: usize> std::fmt::Display for Basis<N> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let (Just(r), Just((sign, factors))) = (self.grade(), self.0) else {
            return write!(f, "0");
        };
        write!(f, "{sign}")?;
        if r == N {
            write!(f, "i")?;
        } else {
            write!(f, "e")?;
            for i in 0..N {
                if factors[i] {
                    write!(f, "{i}")?;
                }
            }
        }
        Ok(())
    }
}
