use crate::{
    common::{even, odd},
    sign::Sign,
};

#[derive(Debug, Clone, Copy)]
pub struct Blade<const N: usize>(Option<(Sign, [bool; N])>);

// TODO: Make this a `Signature` type.
// TODO: Make nullability and signedness a concern of the actual blade/multivector implementation.
// TODO: Multiplications are not closed under `Signature` because they return more information.
impl<const N: usize> Blade<N> {
    pub const ZERO: Blade<N> = Blade(None);
    pub const ONE: Blade<N> = Blade::new([false; N]);
    pub const I: Blade<N> = Blade::new([true; N]);

    pub const fn new(bits: [bool; N]) -> Blade<N> {
        Blade(Some((Sign::Pos, bits)))
    }

    /// Compute the geometric product between two blades.
    /// - `eᵢeᵢ = 1`
    /// - `eᵢeⱼ = eᵢⱼ` ⇔ `i ≠ j`
    ///- `eᵢeⱼ = -eⱼeᵢ`
    pub const fn mul(self, rhs: Blade<N>) -> Blade<N> {
        let lhs = self;
        let Some(lhs) = lhs.0 else { return Blade::ZERO };
        let Some(rhs) = rhs.0 else { return Blade::ZERO };

        let mut sign = lhs.0.mul(rhs.0);
        let mut result = [false; N];

        let lhs = lhs.1;
        let rhs = rhs.1;

        repeat!(i in 0..N {
            if lhs[i] {
                // Flip sign for each permutation.
                repeat!(j in 0..1 {
                    if rhs[j] {
                        sign = sign.neg();
                    }
                });
            }
            result[i] = match (lhs[i], rhs[i]) {
                (true, false) | (false, true) => true,
                (true, true) | (false, false) => false,
            }
        });

        Blade(Some((sign, result)))
    }

    // Compute the exterior product between two blades.
    /// - `eᵢ ∧ eᵢ = 0`
    /// - `eᵢ ∧ eⱼ = eᵢⱼ` ⇔ `i ≠ j`
    ///- `eᵢ ∧ eⱼ = -eⱼeᵢ`
    pub const fn ext(self, rhs: Blade<N>) -> Blade<N> {
        let product = self.mul(rhs);
        if self.grade() + rhs.grade() == product.grade() {
            product
        } else {
            Blade::ZERO
        }
    }

    /// Reversion operator, rewriting its factors in reverse order.
    /// - `rev(eᵢⱼ) = eⱼᵢ = -eᵢⱼ` ⇔ `i ≠ j`
    pub const fn rev(self) -> Blade<N> {
        let Some((mut sign, bits)) = self.0 else { return Blade::ZERO };
        let r = self.grade();
        if r > 0 && odd(r * (r - 1) / 2) {
            sign = sign.neg()
        }
        Blade(Some((sign, bits)))
    }

    /// Grade involution, reversing the sign of odd blades.
    pub const fn inv(self) -> Blade<N> {
        let Some((sign, bits)) = self.0 else { return Blade::ZERO };
        let sign = if even(self.grade()) { sign } else { sign.neg() };
        Blade(Some((sign, bits)))
    }

    /// Contraction of `self` onto `rhs`.
    pub const fn left_contraction(self, rhs: Blade<N>) -> Blade<N> {
        let product = self.mul(rhs);
        if let Some(r) = rhs.grade().checked_sub(self.grade()) && r == product.grade() {
            product
        } else {
            Blade::ZERO
        }
    }

    /// Contraction of `self` by `rhs`.
    pub const fn right_contraction(self, rhs: Blade<N>) -> Blade<N> {
        rhs.rev().left_contraction(self.rev()).rev()
    }

    /// Bi-directional contraction.
    pub const fn inner(self, rhs: Blade<N>) -> Blade<N> {
        let product = self.mul(rhs);
        if self.grade() != 0
            && rhs.grade() != 0
            && rhs.grade().abs_diff(self.grade()) == product.grade()
        {
            product
        } else {
            Blade::ZERO
        }
    }

    pub const fn dot(self, rhs: Blade<N>) -> Blade<N> {
        let product = self.mul(rhs);
        if rhs.grade().abs_diff(self.grade()) == product.grade() {
            product
        } else {
            Blade::ZERO
        }
    }

    /// Scalar product, producing non-zero scalars only when grades match.
    /// In that case, the result can be interpreted as a metric between blades:
    /// `A.rev() * A` can be used as the squared norm of `A`.
    pub const fn scalar(self, rhs: Blade<N>) -> Blade<N> {
        let product = self.mul(rhs);
        if product.grade() == 0 {
            product
        } else {
            Blade::ZERO
        }
    }

    /// The grade or step of this blade, equating to the number of distinct factors.
    pub const fn grade(self) -> usize {
        let Some((_, bits)) = self.0 else { return 0 };
        let mut grade = 0;
        repeat!(i in 0..N {
            if bits[i] {
                grade += 1;
            }
        });
        grade
    }

    pub const fn anti_grade(self) -> usize {
        N - self.grade()
    }

    // TODO: Grade projection
    pub fn proj(self) {
        todo!()
    }
}

// TODO: Remove
impl<const N: usize> std::ops::Neg for Blade<N> {
    fn neg(self) -> Self::Output {
        Blade(self.0.map(|(sign, bits)| (sign.neg(), bits)))
    }

    type Output = Blade<N>;
}

impl<const N: usize> std::fmt::Display for Blade<N> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if let Some((sign, factors)) = self.0 {
            write!(f, "{sign}1e")?;
            for i in 0..N {
                if factors[i] {
                    write!(f, "{i}")?;
                }
            }
        } else {
            write!(f, "0")?;
        }
        Ok(())
    }
}
