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
#![allow(dead_code)]
#[allow(unused_attributes)]

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


/// # Flip Flop
/// 
/// A flip flop is a circuit that has two stable states and can be used to store state information.
/// 
/// A flip flop is a sequential circuit that has two inputs and two outputs.
/// 
/// The two inputs are:
/// - Data input (D)
/// - Clock input (CLK)
/// 
/// The two outputs are:
/// - Q
/// - Q̅
///     
/// The flip flop has two stable states:
/// - Set state (Q = 1, Q̅ = 0)
/// - Reset state (Q = 0, Q̅ = 1)
/// 

#[derive(Debug, Clone)]
pub struct FlipFlop {
    d: bool,
    clk: bool,
    q: bool,
    not_q: bool,
}

impl FlipFlop {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn new_with_input(d: bool, clk: bool) -> Self {
        Self {
            d,
            clk,
            q: false,
            not_q: true,
        }
    }
}



impl Default for FlipFlop {
    fn default() -> Self {
        Self {
            d: false,
            clk: false,
            q: false,
            not_q: true,
        }
    }
}