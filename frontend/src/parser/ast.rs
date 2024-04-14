use crate::parser::Statement;

/// Represents an AST node.
#[derive(Debug, PartialEq, Clone)]
pub struct Node {
    /// Represents a statement.
    pub statement: Statement,

    /// Represents the next node in the AST.
    pub next: Option<Box<Node>>,
}

/// Represents an AST.
#[derive(Debug, PartialEq, Clone)]
pub struct Ast {
    /// Represents the nodes in the AST.
    nodes: Vec<Node>,
}

impl Ast {
    /// Creates a new AST.
    pub fn new() -> Self {
        Self { nodes: Vec::new() }
    }

    /// Adds a node to the AST.
    pub fn add_node(&mut self, node: Node) {
        self.nodes.push(node);
    }

    /// Returns the nodes in the AST.
    pub fn nodes(&self) -> &Vec<Node> {
        &self.nodes
    }
}
