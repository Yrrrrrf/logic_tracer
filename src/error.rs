//! # Error types
//! 
//! This module contains the error types used in the application.


/// Specify the error types that can occur in the application.
/// It works as a wrapper around the standard library's `Result` type.
pub type Result<T> = std::result::Result<T, AppError>;
// ^ Wrapper (en) == Envoltura (es) == Enveloppe (fr)

#[derive(Debug)]
pub enum AppError {

}
