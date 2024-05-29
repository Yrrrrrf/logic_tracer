pub mod combinational;
pub mod sequential;

//! Circuits module. (src\circuits\circuit.rs)
//! 
//! This module contains the main circuit enum.
//! 
//! The circuit enum can be either [`Sequential`] or [`Combinational`].
//! 
//! [`Sequential`]: crate::circuits::sequential::Sequential
//! [`Combinational`]: crate::circuits::combinational::Combinational
//! 
//! 
//! 
//! # Sequential circuits.
//! 
//! This module contains sequential circuits, which are circuits that have memory.
//! 
//! This module contains the following circuits:
//! - Flip Flop
//! - Latch
//! 
//! - Register
//! - Counter
//! - Shift Register (Serial & Parallel)

pub use combinational::*;
pub use sequential::*;


/// This is the main circuit enum.
/// 
/// Recived a generic T that would be a [`Combinational`] or a [`Sequential`] circuit.
/// (Combinational and Sequential are traits)
///
#[derive()]
pub enum Circuit {
    Sequential(Box<dyn Sequential>),
    Combinational(Box<dyn Combinational>),
    // Explaning the code above:
    // It means that the Circuit enum can be either Sequential or Combinational.

    // If will be Sequential, it will receive a Box<dyn Sequential> (a box of a trait object that implements the Sequential trait).
    // If will be Combinational, it will receive a Box<dyn Combinational> (a box of a trait object that implements the Combinational trait).
}

impl Circuit {
    pub fn new<T>(circuit: T) -> Self
    where
        T: Combinational + 'static,
    {
        Self::Combinational(Box::new(circuit))
    }

    pub fn new_sequential<T>(circuit: T) -> Self
    where
        T: Sequential + 'static,
    {
        Self::Sequential(Box::new(circuit))
    }

    pub fn calculate(&mut self) {
        match self {
            Self::Sequential(circuit) => circuit.calculate(),
            Self::Combinational(circuit) => circuit.calculate(),
        }
    }

    pub fn get_input(&self) -> Vec<bool> {
        match self {
            Self::Sequential(circuit) => circuit.get_input(),
            Self::Combinational(circuit) => circuit.get_input(),
        }
    }

    pub fn set_input(&mut self, input_line: Vec<bool>) {
        match self {
            Self::Sequential(circuit) => circuit.set_input(input_line),
            Self::Combinational(circuit) => circuit.set_input(input_line),
        }
    }

    pub fn get_output(&self) -> bool {
        match self {
            Self::Sequential(circuit) => circuit.get_output(),
            Self::Combinational(circuit) => circuit.get_output(),
        }
    }
}
