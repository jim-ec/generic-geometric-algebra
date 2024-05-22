#![allow(incomplete_features)]
// #![feature(const_for)]
#![feature(generic_const_exprs)]
#![feature(adt_const_params)]
#![feature(inherent_associated_types)]
#![feature(const_trait_impl)]
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

fn main() {
    type GA = algebra::Complex;

    println!("Algebra Dimension: {}", GA::DIM);
    println!("Blade count: {}", GA::BASIS_BLADE_COUNT);

    let metric = GA::metric();
    println!("Metric: {metric}");

    let a = Shape(Some((Sign::Pos, [true])));
    let b = Shape(Some((Sign::Pos, [true])));

    println!("{a} {b} = {}", a.geometric(b, metric));
    println!("{a} ∧ {b} = {}", a.exterior(b, metric));
    println!("{a} ∨ {b} = {}", a.regressive(b, metric));
    println!("{a} >> {b} = {}", a.left_contraction(b, metric));
    println!("{a} << {b} = {}", a.right_contraction(b, metric));
    println!("{a} | {b} = {}", a.inner(b, metric));
    println!("{a} * {b} = {}", a.scalar(b, metric));
}
