//! Sequential circuits.
//! 
//! This module contains sequential circuits, which are circuits that have memory.
//! 
//! This module contains the following circuits:
//! - Flip Flop
//! - Latch
//! 
//! - Register
//! - Counter
//! 
//! - Shift Register
//! - Ring Counter
//! 
//! - Random Number Generator
//! - Timer
//! 
//! - Memory
//! 
//! Theoretical Sequential Circuits:
//! - Programmable Logic Array (PLA)
//! - Finite State Machine (FSM)
#![allow(unused)]
// Compare this snippet from src\circuits\sequential.rs:

pub trait Sequential {
    fn calculate(&mut self);

    fn get_input(&self) -> Vec<bool>;
    fn set_input(&mut self, input_line: Vec<bool>);
    fn get_output(&self) -> bool;
    // fn get_input_as_u8(&self) -> u8;
    // fn get_output_as_u8(&self) -> u8; (...)
    // fn get_output_as_u32(&self) -> u32;  To make it more generic (bit width)
}
