#![allow(dead_code)]
#![allow(incomplete_features)]
#![feature(const_for)]
#![feature(generic_const_exprs)]
#![feature(generic_arg_infer)]
#![feature(min_generic_const_args)]
#![feature(adt_const_params)]
#![feature(inherent_associated_types)]
#![feature(const_trait_impl)]
#![feature(const_try)]
#![feature(never_type)]
#![feature(let_chains)]
#![feature(try_trait_v2)]
#![feature(decl_macro)]
// #![feature(effects)]

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
