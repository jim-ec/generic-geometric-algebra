use crate::{metric::Metric, shape::Shape};

const N: usize = 2;

/// A shape whose type is parametrized over its shape (`e0` != `e1` != `e01` etc.) and its metric.
///
/// Its implementation is blocked by
/// https://github.com/rust-lang/project-const-generics/issues/28#issue-1178177928
/// which would enable generic const parameter types like this:
/// ```
/// fn foo<const LEN: usize, const ARR: [u8; LEN]>() -> [u8; LEN] {
///     ARR
/// }
/// ```
#[derive(Debug, Clone, Copy)]
struct Blade<const S: Shape<N>, const M: Metric<N>>(pub f64);

impl<const S: Shape<N>, const M: Metric<N>> Blade<S, M> {
    pub const ZERO: Self = Blade(0.0);

    pub fn geometric<const T: Shape<N>>(
        self,
        rhs: Blade<T, M>,
    ) -> Option<Blade<{ foo(S, T, M) }, M>> {
        let (sign, _) = S.geometric(T, M)?;
        let factor = sign * self.0 * rhs.0;
        Some(Blade(factor))
    }
}

const fn foo(a: Shape<N>, b: Shape<N>, m: Metric<N>) -> Shape<N> {
    a.geometric(b, m).unwrap().1
}

impl<const S: Shape<{ N }>, const M: Metric<{ N }>> std::ops::Neg for Blade<S, M> {
    fn neg(self) -> Self {
        Blade(-self.0)
    }
    type Output = Self;
}
