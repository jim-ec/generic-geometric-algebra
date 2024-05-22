#![allow(dead_code)]
#![allow(incomplete_features)]
#![feature(const_for)]
#![feature(generic_const_exprs)]
#![feature(adt_const_params)]
#![feature(inherent_associated_types)]
#![feature(const_trait_impl)]
#![feature(const_try)]
#![feature(never_type)]
#![feature(const_option)]
#![feature(let_chains)]
#![feature(try_trait_v2)]
#![feature(decl_macro)]

mod algebra;
mod basis;
mod blade;
mod common;
mod macros;
mod maybe;
mod metric;
mod mv;
mod sign;

#[cfg(test)]
mod test;

use basis::NonzeroBasis;
use sign::Sign;

fn main() {
    type GA = algebra::Complex;

    println!("Algebra Dimension: {}", GA::DIM);
    println!("Blade count: {}", GA::BASIS_BLADE_COUNT);

    let metric = GA::metric();
    println!("Metric: {metric}");

    let a = NonzeroBasis {
        sign: Sign::Pos,
        unit: [true],
    };
    let b = NonzeroBasis {
        sign: Sign::Pos,
        unit: [true],
    };

    println!("{a} {b} = {}", a.geometric(b, metric));
    println!("{a} ∧ {b} = {}", a.exterior(b, metric));
    println!("{a} ∨ {b} = {}", a.regressive(b, metric));
    println!("{a} >> {b} = {}", a.left_contraction(b, metric));
    println!("{a} << {b} = {}", a.right_contraction(b, metric));
    println!("{a} | {b} = {}", a.inner(b, metric));
    println!("{a} * {b} = {}", a.scalar(b, metric));
}
