#![allow(incomplete_features)]
// #![feature(const_for)]
#![feature(generic_const_exprs)]
#![feature(adt_const_params)]
#![feature(inherent_associated_types)]
#![feature(generic_associated_types)]
#![feature(const_trait_impl)]
#![feature(let_else)]
#![feature(const_try)]
#![feature(const_convert)]
#![feature(never_type)]
#![feature(const_option)]
#![allow(dead_code)]

/// Emulates `for ... in ... {}` loops in `const fn`s.
/// Should be replaced by regular `for` loops when
/// [const_for](https://github.com/rust-lang/rust/issues/87575) becomes usuable because
/// [const_trait_impl](https://github.com/rust-lang/rust/issues/67792) implements `~const` for range operators.
macro_rules! repeat {
    ($i:ident in $from:tt..$to:tt $body:block) => {{
        let mut $i = $from;
        while $i < $to {
            {
                let $i = $i;
                $body
            }
            $i += 1;
        }
    }};
}

mod algebra;
mod blade;
mod common;
mod metric;
mod mv;
mod shape;
mod sign;

use shape::Shape;
use sign::Sign;

// TODO: Remove
pub fn vanishable_shape_to_string<const N: usize>(x: Option<(Sign, Shape<N>)>) -> String {
    if let Some((sign, shape)) = x {
        format!("{sign}{shape}")
    } else {
        format!("0")
    }
}

// TODO: Remove
pub fn shape_to_string<const N: usize>((sign, shape): (Sign, Shape<N>)) -> String {
    format!("{sign}{shape}")
}

fn main() {
    type GA = algebra::Complex;

    println!("Algebra Dimension: {}", GA::DIM);
    println!("Blade count: {}", GA::BASIS_BLADE_COUNT);

    let metric = GA::metric();
    println!("Metric: {metric}");

    let a = Shape([true]);
    let b = Shape([true]);

    println!(
        "{a} {b} = {}",
        vanishable_shape_to_string(a.geometric(b, metric))
    );
    println!(
        "{a} ∧ {b} = {}",
        vanishable_shape_to_string(a.exterior(b, metric))
    );
    println!(
        "{a} ∨ {b} = {}",
        vanishable_shape_to_string(a.regressive(b, metric))
    );
    println!(
        "{a} >> {b} = {}",
        vanishable_shape_to_string(a.left_contraction(b, metric))
    );
    println!(
        "{a} << {b} = {}",
        vanishable_shape_to_string(a.right_contraction(b, metric))
    );
    println!(
        "{a} | {b} = {}",
        vanishable_shape_to_string(a.inner(b, metric))
    );
    println!(
        "{a} * {b} = {}",
        vanishable_shape_to_string(a.scalar(b, metric))
    );
}
