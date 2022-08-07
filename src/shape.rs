use crate::{
    common::{even, odd},
    metric::{Metric, Square},
    sign::Sign,
};

/// Encodes a factorization of a blade:
/// `A = B eᵢ` ⇔ `A[i]`
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Shape<const N: usize>(pub Option<(Sign, [bool; N])>);

impl<const N: usize> Shape<N> {
    pub const ZERO: Shape<N> = Shape(None);
    pub const ONE: Shape<N> = Shape(Some((Sign::Pos, [false; N])));
    pub const I: Shape<N> = Shape(Some((Sign::Pos, [true; N])));

    // pub const N: usize = N;
    pub const fn foo(self) -> usize {
        N
    }

    /// Parity of the reversion operator, rewriting its factors in reverse order.
    /// - `rev(eᵢⱼ) = eⱼᵢ = -eᵢⱼ` ⇔ `i ≠ j`
    pub const fn reverse(self) -> Shape<N> {
        if let Some(r) = self.grade() && r > 0 && odd(r * (r - 1) / 2) {
            self.neg()
        } else {
            self
        }
    }

    /// Parity of the grade involution, reversing the sign of odd blades.
    pub const fn involute(self) -> Shape<N> {
        if let Some(r) = self.grade() && even(r) {
            self
        } else {
            self.neg()
        }
    }

    /// Clifford Conjugate
    pub const fn conjugate(self) -> Shape<N> {
        self.reverse().involute()
    }

    pub const fn neg(self) -> Shape<N> {
        let Some((sign, factors)) = self.0 else { return Shape::ZERO };
        Shape(Some((sign.neg(), factors)))
    }

    /// Poincaré duality operator
    pub const fn dual(self) -> Shape<N> {
        let Some(rhs) = self.0 else { return Shape::ZERO };
        let mut dual = [false; N];
        repeat!(i in 0..N {
            dual[i] = !rhs.1[i];
        });
        Shape(Some((rhs.0, dual)))
    }

    /// Compute the geometric product between two blades.
    /// - `eᵢeᵢ = 1`
    /// - `eᵢeⱼ = eᵢⱼ` ⇔ `i ≠ j`
    ///- `eᵢeⱼ = -eⱼeᵢ`
    pub const fn geometric(self, rhs: Shape<N>, metric: Metric<N>) -> Shape<N> {
        let Some(lhs) = self.0 else { return Shape::ZERO };
        let Some(rhs) = rhs.0 else { return Shape::ZERO };
        let mut product = [false; N];
        let mut sign = lhs.0.mul(rhs.0);
        repeat!(i in 0..N {
            if lhs.1[i] {
                // Since shapes do not encode any order of factorization, a sign reversal
                // must accomodate for each permutation.
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
                        return Shape::ZERO
                    },
                }
                (false, false) => false,
            }
        });
        Shape(Some((sign, product)))
    }

    // Compute the exterior product between two blades.
    /// - `eᵢ ∧ eᵢ = 0`
    /// - `eᵢ ∧ eⱼ = eᵢⱼ` ⇔ `i ≠ j`
    ///- `eᵢ ∧ eⱼ = -eⱼeᵢ`
    pub const fn exterior(self, rhs: Shape<N>, metric: Metric<N>) -> Shape<N> {
        let product = self.geometric(rhs, metric);

        if let Some(r) = rhs.grade()
            && let Some(l) = self.grade()
            && let Some(p) =product.grade()
            && r + l == p
        {
            product
        } else {
            Shape::ZERO
        }
    }

    // Compute the regressive product between two blades using the identity
    /// `A ∨ B = J(J(A) ∧ J(B))`
    pub const fn regressive(self, rhs: Shape<N>, metric: Metric<N>) -> Shape<N> {
        self.dual().exterior(rhs.dual(), metric).dual()
    }

    /// Contraction of `self` onto `rhs`.
    /// Intuitively, this returns the sub-blade of `rhs` which is prependicular to `self.
    pub const fn left_contraction(self, rhs: Shape<N>, metric: Metric<N>) -> Shape<N> {
        let product = self.geometric(rhs, metric);

        if let Some(r) = rhs.grade()
            && let Some(l) = self.grade()
            && let Some(p) = product.grade()
            && let Some(q) = r.checked_sub(l)
            && q == p
        {
            product
        } else {
            Shape::ZERO
        }
    }

    /// Contraction of `self` by `rhs`.
    /// `A << B = (B~ >> A~)~`
    /// Intuitively, this returns the sub-blade of `self` which is prependicular to `rhs.
    pub const fn right_contraction(self, rhs: Shape<N>, metric: Metric<N>) -> Shape<N> {
        rhs.reverse()
            .left_contraction(self.reverse(), metric)
            .reverse()
    }

    /// Bi-directional contraction.
    pub const fn inner(self, rhs: Shape<N>, metric: Metric<N>) -> Shape<N> {
        let product = self.geometric(rhs, metric);

        if let Some(r) = rhs.grade()
            && let Some(l) = self.grade()
            && let Some(p) = product.grade()
            && r.abs_diff(l) == p
        {
            product
        } else {
            Shape::ZERO
        }
    }

    /// Scalar product, producing non-zero scalars only when grades match.
    /// In that case, the result can be interpreted as a metric between blades:
    /// `A~ * A` can be used as the squared norm of `A`.
    pub const fn scalar(self, rhs: Shape<N>, metric: Metric<N>) -> Shape<N> {
        let product = self.geometric(rhs, metric);
        if let Some(p) = product.grade() && p == 0 {
            product
        } else {
            Shape::ZERO
        }
    }

    /// The *grade* (sometime also called *step*) of this blade, equating to the number of distinct factors.
    /// Returns [None] if this shape is vanishing.
    pub const fn grade(self) -> Option<usize> {
        let (_, factors) = self.0?;
        let mut grade = 0;
        repeat!(i in 0..N {
            if factors[i] {
                grade += 1;
            }
        });
        Some(grade)
    }

    pub const fn anti_grade(self) -> Option<usize> {
        Some(N - self.grade()?)
    }
}

impl<const N: usize> std::fmt::Display for Shape<N> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let (Some(r), Some((sign, factors))) = (self.grade(), self.0) else {
            return write!(f, "0")
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
