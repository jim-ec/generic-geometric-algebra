// TODO: Separate into non-metric algebra and metric algebra types, since many operations do not care about the metrics?
pub struct Algebra<const N: usize>();

/// Basis elements for G(2, 0, 0):
/// - `#1`: e
/// - `#2`: e0, e1
/// - `#1`: e01
impl<const N: usize> Algebra<N> {
    // const K: usize = basis_blade_count(N);
    // const fn storage_size() -> usize {
    //     todo!()
    // }
}
