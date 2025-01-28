use crate::{basis::Basis, maybe::Maybe::Just, metric::Metric};

const N: usize = 2;

/// A blade whose type is parametrized over its basis (e.g. `e0`) and its metric.
///
/// Its implementation is blocked by
/// https://github.com/rust-lang/project-const-generics/issues/28#issue-1178177928
/// which would enable generic const parameter types like this:
/// ```compile_fail
/// fn foo<const LEN: usize, const ARR: [u8; LEN]>() -> [u8; LEN] {
///     ARR
/// }
/// ```
#[derive(Debug, Clone, Copy)]
struct Blade<const B: Basis<N>, const M: Metric<N>>(pub f64);

impl<const B: Basis<N>, const M: Metric<N>> Blade<B, M> {
    pub const ZERO: Self = Blade(0.0);
    pub const ONE: Self = Blade(1.0);

    pub fn scale(self) -> f64 {
        let Just(basis) = B else { return 0.0 };
        basis.sign * self.0
    }

    pub fn geometric<const T: Basis<N>>(self, rhs: Blade<T, M>) -> Blade<{ B.geometric(T, M) }, M> {
        Blade(self.0 * rhs.0)
    }

    pub fn exterior<const T: Basis<N>>(self, rhs: Blade<T, M>) -> Blade<{ B.exterior(T, M) }, M> {
        Blade(self.0 * rhs.0)
    }

    pub fn left_contraction<const T: Basis<N>>(
        self,
        rhs: Blade<T, M>,
    ) -> Blade<{ B.left_contraction(T, M) }, M> {
        Blade(self.0 * rhs.0)
    }

    pub fn right_contraction<const T: Basis<N>>(
        self,
        rhs: Blade<T, M>,
    ) -> Blade<{ B.right_contraction(T, M) }, M> {
        Blade(self.0 * rhs.0)
    }

    pub fn inner<const T: Basis<N>>(self, rhs: Blade<T, M>) -> Blade<{ B.inner(T, M) }, M> {
        Blade(self.0 * rhs.0)
    }

    pub fn scalar<const T: Basis<N>>(self, rhs: Blade<T, M>) -> Blade<{ B.scalar(T, M) }, M> {
        Blade(self.0 * rhs.0)
    }

    // pub fn norm_squared(self) -> f64
    // where
    //     [;{S.scalar(S, M)}]:,
    // {
    //     self.scalar(self);
    //     todo!()
    // }

    /// Computes `A` projected onto `B` using the formular `(A >> B^-1) >> B`.
    /// TODO: Currently, this ignores the norm of [rhs] in `B^-1`.
    /// Multivectors using this operation either need to scale the result by [rhs]'s reciprocal norm,
    /// or normalize [rhs] before projecting.
    pub fn project<const T: Basis<N>>(
        self,
        rhs: Blade<T, M>,
    ) -> Blade<{ B.left_contraction(T, M).reverse().left_contraction(T, M) }, M> {
        Blade(self.0 * rhs.0)
    }
}

// This does not work because we cannot be generic over `rhs`:
// impl<const S: Basis<N>, const T: Basis<N>, const M: Metric<N>> std::ops::Mul for Blade<S, M> {
//     type Output = Blade<{ S.geometric(T, M) }, M>;
//     fn mul(self, rhs: Blade<T, M>) -> Self::Output {
//         Blade(self.0 * rhs.0)
//     }
// }

impl<const S: Basis<{ N }>, const M: Metric<{ N }>> std::ops::Neg for Blade<S, M> {
    fn neg(self) -> Self {
        Blade(-self.0)
    }
    type Output = Self;
}
