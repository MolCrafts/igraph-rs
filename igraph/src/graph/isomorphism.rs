use igraph_sys::*;

use super::Graph;
use crate::error::{Result, check};
use crate::vector::VectorIntList;

impl Graph {
    /// Count the number of subisomorphisms of `pattern` in this graph using VF2.
    pub fn count_subisomorphisms_vf2(&self, pattern: &Graph) -> Result<i64> {
        let mut count: i64 = 0;
        unsafe {
            check(igraph_count_subisomorphisms_vf2(
                self.as_ptr(),
                pattern.as_ptr(),
                std::ptr::null(), // vertex_color1
                std::ptr::null(), // vertex_color2
                std::ptr::null(), // edge_color1
                std::ptr::null(), // edge_color2
                &mut count,
                None, // node_compat_fn
                None, // edge_compat_fn
                std::ptr::null_mut(),
            ))?;
        }
        Ok(count)
    }

    /// Get all subisomorphism mappings of `pattern` in this graph using VF2.
    ///
    /// Returns a list of mappings, where each mapping is a vector of vertex
    /// indices in `self` corresponding to the vertices of `pattern`.
    pub fn get_subisomorphisms_vf2(&self, pattern: &Graph) -> Result<Vec<Vec<i64>>> {
        let mut maps = VectorIntList::new()?;
        unsafe {
            check(igraph_get_subisomorphisms_vf2(
                self.as_ptr(),
                pattern.as_ptr(),
                std::ptr::null(), // vertex_color1
                std::ptr::null(), // vertex_color2
                std::ptr::null(), // edge_color1
                std::ptr::null(), // edge_color2
                maps.as_mut_ptr(),
                None, // node_compat_fn
                None, // edge_compat_fn
                std::ptr::null_mut(),
            ))?;
        }
        Ok(maps.to_vec_of_vecs())
    }
}
