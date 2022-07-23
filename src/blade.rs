use crate::{metric::Metric, shape::Shape};

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
struct Blade<const N: usize, const S: Shape<{ N }>, const M: Metric<{ N }>>(pub f64);

impl<const N: usize, const S: Shape<{ N }>, const M: Metric<{ N }>> Blade<N, S, M> {
    pub const ZERO: Self = Blade(0.0);

    pub fn geometric(self, rhs: Self) -> Self {
        todo!()
    }
}

impl<const N: usize, const S: Shape<{ N }>, const M: Metric<{ N }>> std::ops::Neg
    for Blade<N, S, M>
{
    fn neg(self) -> Self {
        Blade(-self.0)
    }
    type Output = Self;
}
