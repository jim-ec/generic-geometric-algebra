use crate::{
    common::{even, odd},
    sign::Sign,
};

/// Encodes a factorization of a blade:
/// `A = B eᵢ` ⇔ `A[i]`
#[derive(Debug, Clone, Copy)]
pub struct Shape<const N: usize>(pub [bool; N]);

impl<const N: usize> Shape<N> {
    pub const ONE: Shape<N> = Shape([false; N]);
    pub const I: Shape<N> = Shape([true; N]);

    /// Compute the geometric product between two blades.
    /// - `eᵢeᵢ = 1`
    /// - `eᵢeⱼ = eᵢⱼ` ⇔ `i ≠ j`
    ///- `eᵢeⱼ = -eⱼeᵢ`
    pub const fn mul(self, rhs: Shape<N>) -> (Sign, Shape<N>) {
        let mut product = [false; N];
        let mut sign = Sign::Pos;
        repeat!(i in 0..N {
            if self.0[i] {
                // Flip sign for each permutation.
                repeat!(j in 0..1 {
                    if rhs.0[j] {
                        sign = sign.neg();
                    }
                });
            }
            product[i] = match (self.0[i], rhs.0[i]) {
                (true, false) | (false, true) => true,
                (true, true) | (false, false) => false,
            }
        });
        (sign, Shape(product))
    }

    // Compute the exterior product between two blades.
    /// - `eᵢ ∧ eᵢ = 0`
    /// - `eᵢ ∧ eⱼ = eᵢⱼ` ⇔ `i ≠ j`
    ///- `eᵢ ∧ eⱼ = -eⱼeᵢ`
    pub const fn ext(self, rhs: Shape<N>) -> Option<(Sign, Shape<N>)> {
        let (sign, product) = self.mul(rhs);
        if self.grade() + rhs.grade() == product.grade() {
            Some((sign, product))
        } else {
            None
        }
    }

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

    /// Contraction of `self` onto `rhs`.
    /// Intuitively, this returns the sub-blade of `rhs` which is prependicular to `self.
    pub const fn left_contraction(self, rhs: Shape<N>) -> Option<(Sign, Shape<N>)> {
        let (sign, product) = self.mul(rhs);
        if let Some(r) = rhs.grade().checked_sub(self.grade()) && r == product.grade() {
            Some((sign, product))
        } else {
            None
        }
    }

    /// Contraction of `self` by `rhs`.
    /// `A << B = (B~ >> A~)~`
    /// Intuitively, this returns the sub-blade of `self` which is prependicular to `rhs.
    pub const fn right_contraction(self, rhs: Shape<N>) -> Option<(Sign, Shape<N>)> {
        let (sign, product) = rhs.left_contraction(self)?;
        let sign = sign.mul(rhs.rev()).mul(self.rev());
        Some((sign, product))
    }

    /// Bi-directional contraction.
    pub const fn inner(self, rhs: Shape<N>) -> Option<(Sign, Shape<N>)> {
        let (sign, product) = self.mul(rhs);
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
    pub const fn dot(self, rhs: Shape<N>) -> Option<(Sign, Shape<N>)> {
        let (sign, product) = self.mul(rhs);
        if rhs.grade().abs_diff(self.grade()) == product.grade() {
            Some((sign, product))
        } else {
            None
        }
    }

    /// Scalar product, producing non-zero scalars only when grades match.
    /// In that case, the result can be interpreted as a metric between blades:
    /// `A~ * A` can be used as the squared norm of `A`.
    pub const fn scalar(self, rhs: Shape<N>) -> Option<(Sign, Shape<N>)> {
        let (sign, product) = self.mul(rhs);
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
}

impl<const N: usize> std::fmt::Display for Shape<N> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "e")?;
        for i in 0..N {
            if self.0[i] {
                write!(f, "{i}")?;
            }
        }
        Ok(())
    }
}

pub fn shape_to_string<const N: usize>(x: Option<(Sign, Shape<N>)>) -> String {
    if let Some((sign, shape)) = x {
        format!("{sign}{shape}")
    } else {
        format!("0")
    }
}
