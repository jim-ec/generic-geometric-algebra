use crate::{
    algebra,
    basis::NonzeroBasis,
    maybe::Maybe::{Just, Nothing},
    sign::Sign,
};

type GA = algebra::Complex;

#[test]
fn test_complex_basis() {
    assert_eq!(GA::DIM, 1);
    assert_eq!(GA::BASIS_BLADE_COUNT, 2);

    let metric = GA::metric();

    let i = Just(NonzeroBasis {
        sign: Sign::Pos,
        unit: [true],
    });

    assert_eq!(
        i.geometric(i, metric),
        Just(NonzeroBasis::ONE.neg()),
        "i * i = -1"
    );
    assert_eq!(i.exterior(i, metric), Nothing, "i ∧ i = 0");
    assert_eq!(i.regressive(i, metric), i, "i ∨ i = i");
    assert_eq!(
        i.left_contraction(i, metric),
        Just(NonzeroBasis::ONE.neg()),
        "i >> i = -1"
    );
    assert_eq!(
        i.right_contraction(i, metric),
        Just(NonzeroBasis::ONE.neg()),
        "i << i = -1"
    );
    assert_eq!(
        i.inner(i, metric),
        Just(NonzeroBasis::ONE.neg()),
        "i | i = -1"
    );
    assert_eq!(
        i.scalar(i, metric),
        Just(NonzeroBasis::ONE.neg()),
        "i * i = -1"
    );
}

#[test]
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
