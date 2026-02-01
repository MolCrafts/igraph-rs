use igraph_sys::*;

use super::Graph;
use crate::error::{Result, check};
use crate::matrix::Matrix;
use crate::types::NeighborMode;

impl Graph {
    /// Compute shortest path distances between all vertex pairs.
    ///
    /// Returns a matrix where element `[i][j]` is the distance from vertex `i` to vertex `j`.
    pub fn distances(&self, mode: NeighborMode) -> Result<Vec<Vec<f64>>> {
        let n = self.vcount();
        let mut res = Matrix::new(n, n)?;
        unsafe {
            check(igraph_distances(
                self.as_ptr(),
                std::ptr::null(), // weights
                res.as_mut_ptr(),
                igraph_vss_all(),
                igraph_vss_all(),
                mode.to_raw(),
            ))?;
        }
        Ok(res.to_vec_of_vecs())
    }

    /// Compute the diameter (longest shortest path) of the graph.
    pub fn diameter(&self, directed: bool) -> Result<f64> {
        let mut res: f64 = 0.0;
        unsafe {
            check(igraph_diameter(
                self.as_ptr(),
                std::ptr::null(), // weights
                &mut res,
                std::ptr::null_mut(), // from
                std::ptr::null_mut(), // to
                std::ptr::null_mut(), // vertex_path
                std::ptr::null_mut(), // edge_path
                directed,
                true, // unconn: return infinity for disconnected
            ))?;
        }
        Ok(res)
    }
}
