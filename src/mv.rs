// use std::fmt::Write;

// use itertools::Itertools;

// #[derive(Debug, Clone, Copy)]
// pub struct MV<const N: usize>(pub [f64; basis_blade_count(N)])
// where
//     [f64; basis_blade_count(N)]:;

// impl<const N: usize> MV<N>
// where
//     [f64; basis_blade_count(N)]:,
// {
//     pub fn add(self, rhs: MV<N>) -> MV<N> {
//         let mut result = [0.0; basis_blade_count(N)];
//         for i in 0..basis_blade_count(N) {
//             result[i] = self.0[i] + rhs.0[i];
//         }
//         MV(result)
//     }

//     pub fn sub(self, rhs: MV<N>) -> MV<N> {
//         let mut result = [0.0; basis_blade_count(N)];
//         for i in 0..basis_blade_count(N) {
//             result[i] = self.0[i] - rhs.0[i];
//         }
//         MV(result)
//     }

//     pub fn scale(self, rhs: f64) -> MV<N> {
//         let mut result = [0.0; basis_blade_count(N)];
//         for i in 0..basis_blade_count(N) {
//             result[i] = self.0[i] * rhs;
//         }
//         MV(result)
//     }

//     pub fn mul(self, _rhs: f64) -> MV<N> {
//         todo!()
//     }
// }

// impl<const N: usize> std::fmt::Display for MV<N>
// where
//     [f64; basis_blade_count(N)]:,
// {
//     fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
//         let mut factors = Vec::new();

//         for i in 0..basis_blade_count(N) {
//             let value = self.0[i];
//             if value == 0.0 {
//                 continue;
//             }

//             let mut suffix = String::new();

//             for k in 0..N {
//                 if i & (1 << k) != 0 {
//                     write!(&mut suffix, "{k}")?;
//                 }
//             }

//             factors.push((value, suffix));
//         }

//         factors.sort_by(|(_, s1), (_, s2)| s1.len().cmp(&s2.len()));

//         let display = factors
//             .into_iter()
//             .map(|(value, suffix)| format!("{}e{}", value, suffix))
//             .join(" + ");

//         write!(f, "{display}")
//     }
// }
