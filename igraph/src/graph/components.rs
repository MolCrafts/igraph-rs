use igraph_sys::*;

use super::Graph;
use crate::error::{Result, check};
use crate::types::Connectedness;
use crate::vector::VectorInt;

impl Graph {
    /// Check whether the graph is connected.
    pub fn is_connected(&self, mode: Connectedness) -> Result<bool> {
        let mut res: bool = false;
        unsafe {
            check(igraph_is_connected(self.as_ptr(), &mut res, mode.to_raw()))?;
        }
        Ok(res)
    }

    /// Compute connected components.
    ///
    /// Returns `(membership, component_sizes, num_components)` where
    /// `membership[v]` is the component id of vertex `v`.
    pub fn connected_components(&self, mode: Connectedness) -> Result<(Vec<i64>, Vec<i64>, i64)> {
        let mut membership = VectorInt::new()?;
        let mut csize = VectorInt::new()?;
        let mut no: i64 = 0;
        unsafe {
            check(igraph_connected_components(
                self.as_ptr(),
                membership.as_mut_ptr(),
                csize.as_mut_ptr(),
                &mut no,
                mode.to_raw(),
            ))?;
        }
        Ok((membership.to_vec(), csize.to_vec(), no))
    }
}
