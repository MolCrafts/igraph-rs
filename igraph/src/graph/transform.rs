use igraph_sys::*;

use super::Graph;
use crate::error::{Result, check};
use crate::types::{ToDirectedMode, ToUndirectedMode};
use crate::vector::VectorInt;

impl Graph {
    /// Simplify the graph by removing multi-edges and/or self-loops.
    pub fn simplify(&mut self, remove_multiple: bool, remove_loops: bool) -> Result<()> {
        unsafe {
            check(igraph_simplify(
                self.as_mut_ptr(),
                remove_multiple,
                remove_loops,
                std::ptr::null(), // edge_comb
            ))
        }
    }

    /// Convert an undirected graph to directed.
    pub fn to_directed(&mut self, mode: ToDirectedMode) -> Result<()> {
        unsafe { check(igraph_to_directed(self.as_mut_ptr(), mode.to_raw())) }
    }

    /// Convert a directed graph to undirected.
    pub fn to_undirected(&mut self, mode: ToUndirectedMode) -> Result<()> {
        unsafe {
            check(igraph_to_undirected(
                self.as_mut_ptr(),
                mode.to_raw(),
                std::ptr::null(), // edge_comb
            ))
        }
    }

    /// Create an induced subgraph containing only the specified vertices.
    pub fn induced_subgraph(&self, vids: &[i64]) -> Result<Graph> {
        let v = VectorInt::from_slice(vids)?;
        Graph::init_with(|res| unsafe {
            igraph_induced_subgraph(
                self.as_ptr(),
                res,
                igraph_vss_vector(v.as_ptr()),
                igraph_subgraph_implementation_t_IGRAPH_SUBGRAPH_AUTO,
            )
        })
    }

    /// Create the union of this graph and another graph.
    ///
    /// The result has max(n1, n2) vertices and the union of both edge sets.
    pub fn union(&self, other: &Graph) -> Result<Graph> {
        Graph::init_with(|res| unsafe {
            igraph_union(
                res,
                self.as_ptr(),
                other.as_ptr(),
                std::ptr::null_mut(), // edge_map1
                std::ptr::null_mut(), // edge_map2
            )
        })
    }
}
