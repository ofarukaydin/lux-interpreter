use crate::{
    interpreter::{Interpreter, RuntimeResult},
    literal::Literal,
};
use core::fmt;

pub trait LuxCallable: LuxCallableClone {
    fn call(
        &self,
        interpreter: &mut Interpreter,
        arguments: Vec<Literal>,
    ) -> RuntimeResult<Literal>;
    fn to_str(&self) -> String;

    fn arity(&self) -> usize;

    fn id(&self) -> usize;
}

pub trait LuxCallableClone {
    fn clone_box(&self) -> Box<dyn LuxCallable>;
}

impl<T> LuxCallableClone for T
where
    T: 'static + LuxCallable + Clone,
{
    fn clone_box(&self) -> Box<dyn LuxCallable> {
        Box::new(self.clone())
    }
}

impl Clone for Box<dyn LuxCallable> {
    fn clone(&self) -> Box<dyn LuxCallable> {
        self.clone_box()
    }
}

impl fmt::Debug for dyn LuxCallable {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "{}", self.to_str())
    }
}

impl fmt::Display for dyn LuxCallable {
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        write!(f, "{}", self.to_str())
    }
}

impl PartialEq for dyn LuxCallable {
    fn eq(&self, other: &dyn LuxCallable) -> bool {
        self.id() == other.id()
    }
}
