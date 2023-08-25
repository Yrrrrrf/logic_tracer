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
pub trait Combinational {
    fn calculate(&mut self);

    fn get_input(&self) -> Vec<bool>;
    fn set_input(&mut self, input_line: Vec<bool>);
    fn get_output(&self) -> bool;
    // fn get_input_as_u8(&self) -> u8;
    // fn get_output_as_u8(&self) -> u8; (...)
    // fn get_output_as_u32(&self) -> u32;  To make it more generic (bit width)


}


/// # Half Adder
#[derive(Debug, Clone)]
pub struct HalfAdder {
    a: bool,
    b: bool,
    sum: bool,
    carry: bool,
}

impl HalfAdder {
    // pub fn new() -> Self {
    //     Self::default()
    // }

    pub fn new_with_input(a: bool, b: bool) -> Self {
        Self {
            a,
            b,
            sum: a ^ b,
            carry: a & b,
        }
    }
}


impl Combinational for HalfAdder {
    fn calculate(&mut self) {
        self.sum = self.a ^ self.b;
        self.carry = self.a & self.b;
    }

    fn get_input(&self) -> Vec<bool> {
        vec![self.a, self.b]
    }

    fn set_input(&mut self, input_line: Vec<bool>) {
        self.a = input_line[0];
        self.b = input_line[1];
    }

    fn get_output(&self) -> bool {
        self.sum
    }
}

