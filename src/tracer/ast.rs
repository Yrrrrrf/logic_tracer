//! Abstract Syntax Tree (AST) module for Logic Tracer.
//!
//! This module defines a tree structure for representing parsed expressions
//! using trait objects to support any token type without modification.

use crate::tracer::tokens::*;
use std::fmt::{self, Debug, Display, Formatter};

/// A node in the Abstract Syntax Tree
#[derive(Debug)]
pub struct Node {
    /// The token contained in this node (as a trait object)
    pub token: Box<dyn Token>,

    /// Child nodes of this node
    pub children: Vec<Node>,
}

impl Node {
    /// Creates a new node with the given token
    pub fn new<T: Token + 'static>(token: T) -> Self {
        Self {
            token: Box::new(token),
            children: Vec::new(),
        }
    }

    /// Creates a new leaf node (no children) with the given token
    pub fn leaf<T: Token + 'static>(token: T) -> Self {
        Self {
            token: Box::new(token),
            children: Vec::new(),
        }
    }

    /// Creates a new branch node with the given token and children
    pub fn branch<T: Token + 'static>(token: T, children: Vec<Node>) -> Self {
        Self {
            token: Box::new(token),
            children,
        }
    }

    /// Adds a child node to this node
    pub fn add_child(&mut self, child: Node) {
        self.children.push(child);
    }

    /// Returns true if this node is a leaf (has no children)
    pub fn is_leaf(&self) -> bool {
        self.children.is_empty()
    }
}

impl Display for Node {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        fn format_node(
            node: &Node,
            f: &mut Formatter<'_>,
            indent: usize,
            leaf_counter: &mut usize,
            prefix: &str,
        ) -> fmt::Result {
            let tabs = "\t".repeat(indent);

            // Determine if leaf or node
            let node_type = if node.is_leaf() {
                let leaf_num = *leaf_counter;
                *leaf_counter += 1;
                format!("L{}", leaf_num)
            } else {
                "Node".to_string()
            };

            // Write the node with its prefix
            writeln!(
                f,
                "{}{}{}: {}",
                tabs,
                prefix,
                node_type,
                node.token.to_string()
            )?;

            // Process children
            for (i, child) in node.children.iter().enumerate() {
                let child_prefix = format!("Child {}: ", i + 1);
                format_node(child, f, indent + 1, leaf_counter, &child_prefix)?;
            }

            Ok(())
        }

        let mut leaf_counter = 1;
        format_node(self, f, 0, &mut leaf_counter, "")
    }
}

impl Clone for Node {
    fn clone(&self) -> Self {
        let token_str = self.token.to_string();
        let placeholder_token = PlaceholderToken(token_str);
        Self {
            token: Box::new(placeholder_token),
            children: self.children.clone(),
        }
    }
}

/// A simple placeholder token used for cloning
#[derive(Debug, Clone)]
struct PlaceholderToken(String);

impl Token for PlaceholderToken {
    fn from_str<S: Into<String>>(_string: S) -> Option<Self> {
        None // Placeholder tokens can't be created from strings
    }

    fn to_string(&self) -> String {
        self.0.clone()
    }
}

/// Represents a complete Abstract Syntax Tree
#[derive(Debug, Clone)]
pub struct AST {
    pub root: Option<Node>,
}

impl Display for AST {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match &self.root {
            Some(root) => write!(f, "{}", root),
            None => write!(f, "Empty AST"),
        }
    }
}

impl AST {
    pub fn new() -> Self {
        Self { root: None }
    }

    pub fn with_root(root: Node) -> Self {
        Self { root: Some(root) }
    }

    pub fn set_root(&mut self, root: Node) {
        self.root = Some(root);
    }

    pub fn is_empty(&self) -> bool {
        self.root.is_none()
    }
}
