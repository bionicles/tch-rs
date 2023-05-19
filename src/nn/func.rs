//! Layers defined by closures.
use crate::{TchError, Tensor};

/// A layer defined by a simple closure.
pub struct Func<'a> {
    f: Box<dyn 'a + Fn(&Tensor) -> Result<Tensor, TchError> + Send>,
}

impl<'a> std::fmt::Debug for Func<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "func")
    }
}

pub fn func<'a, F>(f: F) -> Func<'a>
where
    F: 'a + Fn(&Tensor) -> Result<Tensor, TchError> + Send,
{
    Func { f: Box::new(f) }
}

impl<'a> super::module::Module for Func<'a> {
    fn forward(&self, xs: &Tensor) -> Result<Tensor, TchError> {
        (*self.f)(xs)
    }
}

/// A layer defined by a closure with an additional training parameter.
#[allow(clippy::type_complexity)]
pub struct FuncT<'a> {
    f: Box<dyn 'a + Fn(&Tensor, bool) -> Tensor + Send>,
}

impl<'a> std::fmt::Debug for FuncT<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "funcT")
    }
}

pub fn func_t<'a, F>(f: F) -> FuncT<'a>
where
    F: 'a + Fn(&Tensor, bool) -> Result<Tensor, TchError> + Send,
{
    FuncT { f: Box::new(f) }
}

impl<'a> super::module::ModuleT for FuncT<'a> {
    fn forward_t(&self, xs: &Tensor, train: bool) -> Result<Tensor, TchError> {
        (*self.f)(xs, train)
    }
}
