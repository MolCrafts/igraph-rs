use igraph_sys::*;

use super::Graph;
use crate::error::{Result, check};
use crate::vector::VectorInt;

impl Graph {
    /// Add `n` vertices to the graph.
    pub fn add_vertices(&mut self, n: i64) -> Result<()> {
        unsafe { check(igraph_add_vertices(self.as_mut_ptr(), n, std::ptr::null())) }
    }

    /// Add edges to the graph from a slice of `(from, to)` pairs.
    pub fn add_edges(&mut self, edges: &[(i64, i64)]) -> Result<()> {
        let mut ev = VectorInt::new()?;
        for &(from, to) in edges {
            unsafe {
                check(igraph_vector_int_push_back(ev.as_mut_ptr(), from))?;
                check(igraph_vector_int_push_back(ev.as_mut_ptr(), to))?;
            }
        }
        unsafe {
            check(igraph_add_edges(
                self.as_mut_ptr(),
                ev.as_ptr(),
                std::ptr::null(),
            ))
        }
    }

    /// Delete vertices by index.
    pub fn delete_vertices(&mut self, vids: &[i64]) -> Result<()> {
        let v = VectorInt::from_slice(vids)?;
        unsafe {
            check(igraph_delete_vertices(
                self.as_mut_ptr(),
                igraph_vss_vector(v.as_ptr()),
            ))
        }
    }

    /// Delete edges by edge id.
    pub fn delete_edges(&mut self, eids: &[i64]) -> Result<()> {
        let v = VectorInt::from_slice(eids)?;
        unsafe {
            check(igraph_delete_edges(
                self.as_mut_ptr(),
                igraph_ess_vector(v.as_ptr()),
            ))
        }
    }
}
