use crate::{
    common::{even, odd},
    metric::{Metric, Square},
    sign::Sign,
};

/// Encodes a factorization of a blade:
/// `A = B eᵢ` ⇔ `A[i]`
#[derive(Debug, Clone, Copy)]
pub struct Shape<const N: usize>(pub [bool; N]);

impl<const N: usize> Shape<N> {
    pub const ONE: Shape<N> = Shape([false; N]);
    pub const I: Shape<N> = Shape([true; N]);

    /// Parity of the reversion operator, rewriting its factors in reverse order.
    /// - `rev(eᵢⱼ) = eⱼᵢ = -eᵢⱼ` ⇔ `i ≠ j`
    pub const fn rev(self) -> Sign {
        let r = self.grade();
        if r > 0 && odd(r * (r - 1) / 2) {
            Sign::Neg
        } else {
            Sign::Pos
        }
    }

    /// Parity of the grade involution, reversing the sign of odd blades.
    pub const fn inv(self) -> Sign {
        if even(self.grade()) {
            Sign::Pos
        } else {
            Sign::Neg
        }
    }

    /// Clifford Conjugate
    pub const fn conj(self) -> Sign {
        self.rev().mul(self.inv())
    }

    /// Poincaré duality operator
    pub const fn dual(self) -> Shape<N> {
        let mut dual = [false; N];
        repeat!(i in 0..N {
            dual[i] = !self.0[i];
        });
        Shape(dual)
    }

    /// Compute the geometric product between two blades.
    /// - `eᵢeᵢ = 1`
    /// - `eᵢeⱼ = eᵢⱼ` ⇔ `i ≠ j`
    ///- `eᵢeⱼ = -eⱼeᵢ`
    pub const fn mul(self, rhs: Shape<N>, metric: Metric<N>) -> Option<(Sign, Shape<N>)> {
        let mut product = [false; N];
        let mut sign = Sign::Pos;
        repeat!(i in 0..N {
            if self.0[i] {
                // Since shapes do not encode any order of factorization, a sign reversal
                // must accomodate for each permutation.
                repeat!(j in 0..i {
                    if rhs.0[j] {
                        sign = sign.neg();
                    }
                });
            }
            product[i] = match (self.0[i], rhs.0[i]) {
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
                        return None
                    },
                }
                (false, false) => false,
            }
        });
        Some((sign, Shape(product)))
    }

    // Compute the exterior product between two blades.
    /// - `eᵢ ∧ eᵢ = 0`
    /// - `eᵢ ∧ eⱼ = eᵢⱼ` ⇔ `i ≠ j`
    ///- `eᵢ ∧ eⱼ = -eⱼeᵢ`
    pub const fn ext(self, rhs: Shape<N>, metric: Metric<N>) -> Option<(Sign, Shape<N>)> {
        let (sign, product) = self.mul(rhs, metric)?;
        if self.grade() + rhs.grade() == product.grade() {
            Some((sign, product))
        } else {
            None
        }
    }

    /// Contraction of `self` onto `rhs`.
    /// Intuitively, this returns the sub-blade of `rhs` which is prependicular to `self.
    pub const fn left_contraction(
        self,
        rhs: Shape<N>,
        metric: Metric<N>,
    ) -> Option<(Sign, Shape<N>)> {
        let (sign, product) = self.mul(rhs, metric)?;
        if let Some(r) = rhs.grade().checked_sub(self.grade()) && r == product.grade() {
            Some((sign, product))
        } else {
            None
        }
    }

    /// Contraction of `self` by `rhs`.
    /// `A << B = (B~ >> A~)~`
    /// Intuitively, this returns the sub-blade of `self` which is prependicular to `rhs.
    pub const fn right_contraction(
        self,
        rhs: Shape<N>,
        metric: Metric<N>,
    ) -> Option<(Sign, Shape<N>)> {
        let (sign, product) = rhs.left_contraction(self, metric)?;
        let sign = sign.mul(rhs.rev()).mul(self.rev());
        Some((sign, product))
    }

    /// Bi-directional contraction.
    pub const fn inner(self, rhs: Shape<N>, metric: Metric<N>) -> Option<(Sign, Shape<N>)> {
        let (sign, product) = self.mul(rhs, metric)?;
        if self.grade() != 0
            && rhs.grade() != 0
            && rhs.grade().abs_diff(self.grade()) == product.grade()
        {
            Some((sign, product))
        } else {
            None
        }
    }

    /// Bi-directional contraction without edge-casing scalars.
    pub const fn dot(self, rhs: Shape<N>, metric: Metric<N>) -> Option<(Sign, Shape<N>)> {
        let (sign, product) = self.mul(rhs, metric)?;
        if rhs.grade().abs_diff(self.grade()) == product.grade() {
            Some((sign, product))
        } else {
            None
        }
    }

    /// Scalar product, producing non-zero scalars only when grades match.
    /// In that case, the result can be interpreted as a metric between blades:
    /// `A~ * A` can be used as the squared norm of `A`.
    pub const fn scalar(self, rhs: Shape<N>, metric: Metric<N>) -> Option<(Sign, Shape<N>)> {
        let (sign, product) = self.mul(rhs, metric)?;
        if product.grade() == 0 {
            Some((sign, product))
        } else {
            None
        }
    }

    /// The *grade* (sometime also called *step*) of this blade, equating to the number of distinct factors.
    pub const fn grade(self) -> usize {
        let mut grade = 0;
        repeat!(i in 0..N {
            if self.0[i] {
                grade += 1;
            }
        });
        grade
    }

    pub const fn anti_grade(self) -> usize {
        N - self.grade()
    }

    /// Since shapes do not care about norms, use reversion instead of inversion.
    /// Multivectors using this operation either need to scale the result by [rhs]'s reciprocal norm,
    /// or normalize [rhs] before projecting.
    // TODO: Remove this function from [Shape]?
    pub const fn proj(self, rhs: Shape<N>, metric: Metric<N>) -> Option<(Sign, Shape<N>)> {
        let (sign_inner, product_inner) = self.left_contraction(rhs, metric)?;
        let (sign_outer, product_outer) = product_inner.left_contraction(rhs, metric)?;
        let sign = sign_inner.mul(sign_outer);
        Some((sign, product_outer))
    }
}

impl<const N: usize> std::fmt::Display for Shape<N> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "e")?;
        for i in 0..N {
            if self.0[i] {
                write!(f, "{i}")?;
            }
        }
        Ok(())
    }
}
