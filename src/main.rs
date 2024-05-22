#![allow(incomplete_features)]
#![feature(const_for)]
#![feature(generic_const_exprs)]
#![feature(adt_const_params)]
#![feature(inherent_associated_types)]
#![feature(const_trait_impl)]
#![feature(const_try)]
#![feature(never_type)]
#![feature(const_option)]
#![allow(dead_code)]
#![feature(let_chains)]
#![feature(try_trait_v2)]

mod algebra;
mod blade;
mod common;
mod macros;
mod maybe;
mod metric;
mod mv;
mod shape;
mod sign;

use maybe::Maybe::*;
use shape::Shape;
use sign::Sign;

fn main() {
    type GA = algebra::Complex;

    println!("Algebra Dimension: {}", GA::DIM);
    println!("Blade count: {}", GA::BASIS_BLADE_COUNT);

    let metric = GA::metric();
    println!("Metric: {metric}");

    let a = Shape(Just((Sign::Pos, [true])));
    let b = Shape(Just((Sign::Pos, [true])));

    println!("{a} {b} = {}", a.geometric(b, metric));
    println!("{a} ∧ {b} = {}", a.exterior(b, metric));
    println!("{a} ∨ {b} = {}", a.regressive(b, metric));
    println!("{a} >> {b} = {}", a.left_contraction(b, metric));
    println!("{a} << {b} = {}", a.right_contraction(b, metric));
    println!("{a} | {b} = {}", a.inner(b, metric));
    println!("{a} * {b} = {}", a.scalar(b, metric));
}
