use std::{
    marker::ConstParamTy,
    ops::{ControlFlow, FromResidual, Try},
};

/// Reimplements `Option` but with the `ConstParamTy` trait.
#[derive(Debug, Clone, Copy, PartialEq, Eq, ConstParamTy)]
pub enum Maybe<T> {
    Just(T),
    Nothing,
}

impl<T> FromResidual<()> for Maybe<T> {
    fn from_residual(_: ()) -> Self {
        Self::Nothing
    }
}

impl<T> Try for Maybe<T> {
    type Output = T;
    type Residual = ();

    fn from_output(output: Self::Output) -> Self {
        Maybe::Just(output)
    }

    fn branch(self) -> ControlFlow<Self::Residual, Self::Output> {
        match self {
            Maybe::Just(output) => ControlFlow::Continue(output),
            Maybe::Nothing => ControlFlow::Break(()),
        }
    }
}
