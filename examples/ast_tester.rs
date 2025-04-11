//! Test script for the AST implementation with Display trait
//!
//! Run this to verify that the AST structure works correctly and displays properly

#![allow(unused)]

use dev_utils::{app_dt, debug, dlog::*, error, info, trace, warn};
use logic_tracer::ast::*;
use logic_tracer::tokens::*;

/// Test 1: Simple binary operation "23 - 45"
fn test_simple_binary_operation() {
    debug!("Test: Simple binary operation '23 - 45'");

    // Create token instances
    let num_23 = Natural::from_n(23);
    let subtract_op = MathOp::Subtract;
    let num_45 = Natural::from_n(45);

    // Create leaf nodes
    let leaf_23 = Node::leaf(num_23);
    let leaf_45 = Node::leaf(num_45);

    // Create operation node
    let mut op_node = Node::new(subtract_op);
    op_node.add_child(leaf_23);
    op_node.add_child(leaf_45);

    // Create AST
    let ast = AST::with_root(op_node);

    // Print AST structure using Display trait
    trace!("{}", ast);
}

/// Test 2: Binary operation with different number types "3.4 + 1"
fn test_binary_operation_with_different_types() {
    debug!("Test: Binary operation with different number types '3.4 + 1'");

    // Create token instances
    let real_3_4 = Real::from_n(3.4);
    let add_op = MathOp::Add;
    let num_1 = Natural::from_n(1);

    // Build tree directly with constructor methods
    let leaf_3_4 = Node::leaf(real_3_4);
    let leaf_1 = Node::leaf(num_1);

    let op_node = Node::branch(add_op, vec![leaf_3_4, leaf_1]);

    // Create AST
    let ast = AST::with_root(op_node);

    // Print AST structure using Display trait
    trace!("{}", ast);
}

/// Test 3: More complex expression "25.1 * 42 - 13"
fn test_complex_expression() {
    debug!("Test: Complex expression '25.1 * 42 - 13'");

    // Create token instances
    let real_25_1 = Real::from_n(25.1);
    let multiply_op = MathOp::Multiply;
    let num_42 = Natural::from_n(42);
    let subtract_op = MathOp::Subtract;
    let num_13 = Natural::from_n(13);

    // Create leaf nodes
    let leaf_25_1 = Node::leaf(real_25_1);
    let leaf_42 = Node::leaf(num_42);
    let leaf_13 = Node::leaf(num_13);

    // Create multiplication operation
    let mul_node = Node::branch(multiply_op, vec![leaf_25_1, leaf_42]);

    // Create subtraction as the root operation
    let root_node = Node::branch(subtract_op, vec![mul_node, leaf_13]);

    // Create AST
    let ast = AST::with_root(root_node);

    // Print AST structure using Display trait
    trace!("{}", ast);
}

/// Test 4: Building an AST for a logic expression "A & (B | C)"
fn test_logic_expression() {
    debug!("Test: Logic expression 'A & (B | C)'");

    // Create token instances
    let var_a = AlphaUpper::A;
    let var_b = AlphaUpper::B;
    let var_c = AlphaUpper::C;
    let and_op = LogicOp::And;
    let or_op = LogicOp::Or;

    // Create leaf nodes
    let leaf_a = Node::leaf(var_a);
    let leaf_b = Node::leaf(var_b);
    let leaf_c = Node::leaf(var_c);

    // Create or operation
    let or_node = Node::branch(or_op, vec![leaf_b, leaf_c]);

    // Create and operation as the root
    let root_node = Node::branch(and_op, vec![leaf_a, or_node]);

    // Create AST
    let ast = AST::with_root(root_node);

    // Print AST structure using Display trait
    trace!("{}", ast);
}

/// Test 5: Manipulating an AST
fn test_manipulating_ast() {
    debug!("Test: Manipulating an AST");

    // Create an empty AST
    let mut ast = AST::new();
    trace!("Empty AST is_empty(): {}", ast.is_empty());

    // Create and set a root node
    let root_node = Node::new(MathOp::Add);
    ast.set_root(root_node);
    trace!("After setting root, is_empty(): {}", ast.is_empty());

    // Verify we can access and modify the root
    if let Some(root) = &mut ast.root {
        root.add_child(Node::leaf(MathOp::Multiply));
        root.add_child(Node::leaf(MathOp::Divide));
        trace!("Added {} children to root", root.children.len());
        trace!("Root is_leaf(): {}", root.is_leaf());
    }

    // Print the modified AST using Display trait
    trace!("{}", ast);
}

/// Run all the tests
fn main() {
    app_dt!(file!());
    set_max_level(Level::Trace); // Set log level to Trace

    info!("Testing AST implementation with Display trait");

    // test_simple_binary_operation();
    // test_binary_operation_with_different_types();
    test_complex_expression();
    // test_logic_expression();
    // test_manipulating_ast();
}
