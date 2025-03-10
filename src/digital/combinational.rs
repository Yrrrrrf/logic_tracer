//!  Combinational circuits.
//! 
//! This module contains combinational circuits, which are circuits that have no memory.
//! 
//! This module contains the following circuits:
//! - Arithmetic circuits (base for the ALU)
//!   - Half Adder
//!   - Full Adder
//!   - Half Subtractor
//!   - Full Subtractor
//! 
//! - Comparator
//! 
//! - Data Selector
//!   - Multiplexer
//!   - Demultiplexer
//! 
//! - Arithmetic Logic Unit (ALU)
//! 
//! - Signal Converter
//!   - Encoder (bin to signal) & Priority Encoder
//!   - Decoder (signal to bin) & Priority Decoder
//! 
//! - Parity Generator
//! - Parity Checker
//! 
//! 
#![allow(unused)]

// Compare this snippet from src\circuits\sequential.rs:
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
#[derive(Default, Debug, Clone)]
pub struct HalfAdder {
        
}

// todo: CHECK THIS IDEA

// Implement the Combinational ciruits using their functions to calculate the output
// The output must be a u64 (or u32, u16, u8, etc). This to handle the bit width of the circuit.


// some 