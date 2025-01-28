/// Emulates `for ... in ... {}` loops in `const fn`s.
/// Should be replaced by regular `for` loops when
/// [const_for](https://github.com/rust-lang/rust/issues/87575) becomes usuable because
/// [const_trait_impl](https://github.com/rust-lang/rust/issues/67792) implements `~const` for range operators.
pub macro repeat($i:ident in $from:tt..$to:tt $body:block) {
    let mut $i = $from;
    while $i < $to {
        {
            let $i = $i;
            $body
        }
        $i += 1;
    }

    // for $i in $from..$to {
    //     $body
    // }
}

/// Emulates the `?` operator for `Maybe` values. Can be used inside constant functions.
///
/// No longer needed when this compiles:
/// ```compile_fail
/// pub const fn foo(n: Option<usize>) -> Option<usize> {
///     let x=  n?;
///     Some(x + 1)
/// }
/// ```
pub macro yeet($x:expr) {
    match $x {
        crate::maybe::Maybe::Just(x) => x,
        crate::maybe::Maybe::Nothing => return crate::maybe::Maybe::Nothing,
    }
}
