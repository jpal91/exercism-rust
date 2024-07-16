#![allow(unused)]
use std::collections::HashMap;

pub type Attrs = HashMap<String, String>;

#[derive(Default, Clone, PartialEq)]
pub struct Graph {
    pub nodes: Vec<Node>,
    pub edges: Vec<Edge>,
    pub attrs: Attrs
}

#[derive(Clone, Debug, PartialEq)]
pub struct Node {
    name: String,
    pub attrs: Attrs
}

#[derive(Clone, Debug, PartialEq)]
pub struct Edge {
    name: (String, String),
    pub attrs: Attrs
}

impl Graph {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn with_nodes(mut self, nodes: &[Node]) -> Self {
        self.nodes.extend(nodes.to_vec());
        self
    }

    pub fn with_edges(mut self, edges: &[Edge]) -> Self {
        self.edges.extend(edges.to_vec());
        self
    }

    pub fn node(&self, name: &str) -> Option<&Node> {
        self.nodes.iter().find(|n| n.name.as_str() == name)
    }

}

impl Node {
    pub fn new(name: &str) -> Self {
        Self {
            name: name.to_string(),
            attrs: HashMap::new()
        }
    }

}

impl Edge {

    pub fn new(key: &str, value: &str) -> Self {
        Self {
            name: (key.to_string(), value.to_string()),
            attrs: HashMap::new()
        }
    }
}

pub trait Attributes {
    fn get_attrs(&self) -> &Attrs;

    fn get_mut_attrs(&mut self) -> &mut Attrs;

    fn with_attrs(mut self, attrs: &[(&str, &str)]) -> Self 
    where
        Self: Clone + PartialEq
    {
        let att_iter = attrs
            .into_iter()
            .map(|(k, v)| (k.to_string(), v.to_string()));
        
        self.get_mut_attrs()
            .extend(att_iter);

        self
    }

    fn attr(&self, key: &str) -> Option<&str> {
        self.get_attrs().get(key).map(|v| v.as_str())
    }
}

impl Attributes for Edge {
    fn get_attrs(&self) -> &Attrs {
        &self.attrs
    }

    fn get_mut_attrs(&mut self) -> &mut Attrs {
        &mut self.attrs
    }
}

impl Attributes for Node {
    fn get_attrs(&self) -> &Attrs {
        &self.attrs
    }

    fn get_mut_attrs(&mut self) -> &mut Attrs {
        &mut self.attrs
    }
}

impl Attributes for Graph {
    fn get_attrs(&self) -> &Attrs {
        &self.attrs
    }

    fn get_mut_attrs(&mut self) -> &mut Attrs {
        &mut self.attrs
    }
}

pub mod graph {
    pub use super::*;

    pub mod graph_items {
        pub use super::{Node, Edge, Attrs};

        pub mod edge {
            pub use super::{Edge, Attrs};
        }

        pub mod node {
            pub use super::Node;
        }
    }
}
