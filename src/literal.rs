
use std::{
    cmp::Ordering,
    fmt,
    hash::{Hash, Hasher},
    ops,
};

use crate::{callable::LuxCallable, clock::Clock, function::LuxFunction};
#[derive(Debug, Clone, Copy)]
pub struct Float(pub f64);

impl PartialEq for Float {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}

impl Eq for Float {}

impl fmt::Display for Float {
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        write!(f, "{}", self.0.to_string())
    }
}

impl Hash for Float {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.0.to_bits().hash(state);
    }
}

impl PartialOrd for Float {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.0.partial_cmp(&other.0)
    }
}

impl ops::Add<Float> for Float {
    type Output = Float;

    fn add(self, _rhs: Float) -> Float {
        Float(self.0 + _rhs.0)
    }
}

impl ops::Mul<Float> for Float {
    type Output = Float;

    fn mul(self, _rhs: Float) -> Float {
        Float(self.0 * _rhs.0)
    }
}

impl ops::Div<Float> for Float {
    type Output = Float;

    fn div(self, _rhs: Float) -> Float {
        Float(self.0 / _rhs.0)
    }
}

impl ops::Sub<Float> for Float {
    type Output = Float;

    fn sub(self, _rhs: Float) -> Float {
        Float(self.0 - _rhs.0)
    }
}

impl ops::Neg for Float {
    type Output = Float;

    fn neg(self) -> Float {
        Float(self.0.neg())
    }
}

#[derive(PartialEq, Debug, Clone, Eq, Hash)]
pub enum Literal {
    Number(Float),
    String(String),
    Bool(bool),
    Nil,
    Clock(Clock),
    Function(Box<LuxFunction>),
}

impl fmt::Display for Literal {
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        write!(f, "{}", self.to_str())
    }
}

unsafe impl Send for Literal {}

impl Literal {
    fn to_str(&self) -> String {
        match self {
            Literal::Number(num) => num.to_string(),
            Literal::String(str) => str.to_string(),
            Literal::Nil => r#"nil"#.to_string(),
            Literal::Bool(bool) => bool.to_string(),
            Literal::Clock(clock) => clock.to_string(),
            Literal::Function(func) => func.to_str(),
        }
    }

    pub fn is_truthy(&self) -> bool {
        match self {
            Literal::Bool(bool) => *bool,
            Literal::Nil => false,
            _ => true,
        }
    }
}
