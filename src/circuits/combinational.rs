//!  Combinational circuits.
//! 
//! This module contains combinational circuits, which are circuits that have no memory.
//! 
//! This module contains the following circuits:
//! - Half Adder
//! - Full Adder
//! 
//! - Half Subtractor
//! - Full Subtractor
//! 
//! - Multiplexer
//! - Demultiplexer
//! 
//! - Encoder
//! - Decoder
//! 
//! - Priority Encoder
//! - Priority Decoder
//! 
//! - Comparator
//! 
//! - Parity Generator
//! - Parity Checker
//! 
//! - Arithmetic Logic Unit
//! 
#![allow(dead_code)]

// Compare this snippet from src\circuits\sequential.rs:
// use logic_tracer::gpio::GPIO;


pub enum CombinationalCircuit {
    HalfAdder,
    FullAdder,
    HalfSubtractor,
    FullSubtractor,
    Multiplexer,
    Demultiplexer,
    Encoder,
    Decoder,
    PriorityEncoder,
    PriorityDecoder,
    Comparator,
    ParityGenerator,
    ParityChecker,
    ArithmeticLogicUnit,
}


impl CombinationalCircuit {
    fn new() -> CombinationalCircuit {
        CombinationalCircuit::HalfAdder
    }    
}


#[derive(Debug, Clone)]
pub struct HalfAdder {
    a: bool,
    b: bool,
    sum: bool,
    carry: bool,
}

impl HalfAdder {
    pub fn new(a: bool, b: bool) -> HalfAdder {
        HalfAdder {
            a,
            b,
            sum: a ^ b,
            carry: a & b,
        }
    }

    pub fn set_a(&mut self, a: bool) {
        self.a = a;
        self.update_output();
    }

    pub fn set_b(&mut self, b: bool) {
        self.b = b;
        self.update_output();
    }

    fn update_output(&mut self) {
        self.sum = self.a ^ self.b;
        self.carry = self.a & self.b;
    }

    pub fn get_sum(&self) -> bool {
        self.sum
    }

    pub fn get_carry(&self) -> bool {
        self.carry
    }


}

