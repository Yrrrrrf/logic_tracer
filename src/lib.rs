//! # Logic Tracer
//! 
//! `logic_tracer` is a library for tracing the logic of a logic propopsition.
//! 
//! Also creates a truth table and a Karnaugh map (if possible).
//! 
//! ## Example
//! 
//! ```
//! use logic_tracer::logic_tracer;
//! ```
//! 
//! ## Features
//! 
//! - Creates a truth table
//! - Creates a Karnaugh map
//! - Creates a logic trace
//! - Reduces the logic proposition to its simplest form
//! 


#[cfg(test)]  // Only compiles when running tests
mod tests {

    #[test]  // Indicates that this is a test
    fn test_test() {
        assert_eq!(2 + 2, 4);
    }

    #[test]  // Indicates that this is a test
    fn test_test2() {
        assert_eq!(2 + 2, 4);
    }
}