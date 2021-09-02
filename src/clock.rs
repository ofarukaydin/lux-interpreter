use core::fmt;
use rand::Rng;
use std::time::{SystemTime, UNIX_EPOCH};

use crate::{
    callable::LuxCallable,
    interpreter::{Interpreter, RuntimeResult},
    literal::Literal,
};
#[derive(Clone, PartialEq, Debug)]
pub struct Clock {
    arity: usize,
    id: usize,
}

impl LuxCallable for Clock {
    fn call(
        &self,
        _: &mut Interpreter,
        _: Vec<Literal>,
    ) -> RuntimeResult<Literal> {
        let start = SystemTime::now();
        let since_the_epoch = start
            .duration_since(UNIX_EPOCH)
            .expect("Time went backwards");

        let secs = since_the_epoch.as_secs();

        Ok(Literal::Number(secs as f64))
    }

    fn to_str(&self) -> String {
        "<native fn>".to_string()
    }

    fn arity(&self) -> usize {
        0
    }

    fn id(&self) -> usize {
        self.id
    }
}

impl Clock {
    pub fn new() -> Self {
        let mut rng = rand::thread_rng();
        let id: usize = rng.gen();
        Clock { arity: 0, id }
    }
}

impl Default for Clock {
    fn default() -> Self {
        Self::new()
    }
}

impl fmt::Display for Clock {
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        write!(f, "<native fn>",)
    }
}
