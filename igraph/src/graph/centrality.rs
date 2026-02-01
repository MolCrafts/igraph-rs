use igraph_sys::*;

use super::Graph;
use crate::error::{Result, check};
use crate::vector::Vector;

impl Graph {
    /// Compute betweenness centrality for all vertices.
    pub fn betweenness(&self, directed: bool) -> Result<Vec<f64>> {
        let mut res = Vector::new()?;
        unsafe {
            check(igraph_betweenness(
                self.as_ptr(),
                std::ptr::null(), // weights
                res.as_mut_ptr(),
                igraph_vss_all(),
                directed,
                false, // normalized
            ))?;
        }
        Ok(res.to_vec())
    }

    /// Compute closeness centrality for all vertices.
    pub fn closeness(&self, mode: crate::types::NeighborMode) -> Result<Vec<f64>> {
        let mut res = Vector::new()?;
        unsafe {
            check(igraph_closeness(
                self.as_ptr(),
                res.as_mut_ptr(),
                std::ptr::null_mut(), // reachable_count
                std::ptr::null_mut(), // all_reachable
                igraph_vss_all(),
                mode.to_raw(),
                std::ptr::null(), // weights
                true,             // normalized
            ))?;
        }
        Ok(res.to_vec())
    }

    /// Compute PageRank for all vertices.
    pub fn pagerank(&self, damping: f64) -> Result<Vec<f64>> {
        let mut res = Vector::new()?;
        let mut eigenvalue: f64 = 0.0;
        unsafe {
            check(igraph_pagerank(
                self.as_ptr(),
                std::ptr::null(), // weights
                res.as_mut_ptr(),
                &mut eigenvalue,
                damping,
                true, // directed
                igraph_vss_all(),
                igraph_pagerank_algo_t_IGRAPH_PAGERANK_ALGO_PRPACK,
                std::ptr::null_mut(), // options
            ))?;
        }
        Ok(res.to_vec())
    }
}
