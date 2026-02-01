use igraph_sys::*;

use super::Graph;
use crate::error::{Result, check};
use crate::vector::{Vector, VectorInt};

impl Graph {
    /// Community detection using the Leiden algorithm.
    ///
    /// Returns `(membership, num_clusters, quality)`.
    pub fn community_leiden(&self, resolution: f64) -> Result<(Vec<i64>, i64, f64)> {
        let mut membership = VectorInt::new()?;
        let mut nb_clusters: i64 = 0;
        let mut quality: f64 = 0.0;
        unsafe {
            check(igraph_community_leiden(
                self.as_ptr(),
                std::ptr::null(), // edge_weights
                std::ptr::null(), // vertex_out_weights
                std::ptr::null(), // vertex_in_weights
                resolution,
                0.01,  // beta
                false, // start (don't use membership as initial)
                10,    // n_iterations
                membership.as_mut_ptr(),
                &mut nb_clusters,
                &mut quality,
            ))?;
        }
        Ok((membership.to_vec(), nb_clusters, quality))
    }

    /// Community detection using label propagation.
    ///
    /// Returns membership vector.
    pub fn community_label_propagation(&self) -> Result<Vec<i64>> {
        let mut membership = VectorInt::new()?;
        unsafe {
            check(igraph_community_label_propagation(
                self.as_ptr(),
                membership.as_mut_ptr(),
                igraph_neimode_t_IGRAPH_ALL,
                std::ptr::null(), // weights
                std::ptr::null(), // initial
                std::ptr::null(), // fixed
                igraph_lpa_variant_t_IGRAPH_LPA_FAST,
            ))?;
        }
        Ok(membership.to_vec())
    }

    /// Community detection using the fast greedy algorithm.
    ///
    /// Returns `(membership, modularity_values)`.
    pub fn community_fastgreedy(&self) -> Result<(Vec<i64>, Vec<f64>)> {
        let mut membership = VectorInt::new()?;
        let mut modularity = Vector::new()?;
        unsafe {
            check(igraph_community_fastgreedy(
                self.as_ptr(),
                std::ptr::null(),     // weights
                std::ptr::null_mut(), // merges
                modularity.as_mut_ptr(),
                membership.as_mut_ptr(),
            ))?;
        }
        Ok((membership.to_vec(), modularity.to_vec()))
    }
}
