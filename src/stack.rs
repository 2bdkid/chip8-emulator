use std::fmt;
use std::error::Error;

pub struct Stack {
    array: [u16; 16],
    sp: usize,
}

#[derive(Copy, Clone, Debug)]
pub enum StackError {
    Empty,
    Full,
}

impl fmt::Display for StackError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            StackError::Empty => write!(f, "Empty stack"),
            StackError::Full => write!(f, "Full stack")
        }
    }
}

impl Error for StackError {}

impl Stack {
    fn new() -> Stack {
        Stack {
            array: [0; 16],
            sp: 0,
        }
    }

    pub fn push(&mut self, value: u16) -> Result<(), StackError> {
        if self.sp < 16 {
            self.array[self.sp] = value;
            self.sp += 1;
            Ok(())
        } else {
            Err(StackError::Full)
        }
    }

    pub fn pop(&mut self) -> Result<u16, StackError> {
        if self.sp > 0 {
            self.sp -= 1;
            Ok(self.array[self.sp])
        } else {
            Err(StackError::Empty)
        }
    }
}
