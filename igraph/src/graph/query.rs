use igraph_sys::*;

use super::Graph;
use crate::error::{Result, check};
use crate::types::{Loops, NeighborMode};
use crate::vector::VectorInt;

impl Graph {
    /// Return the number of vertices.
    pub fn vcount(&self) -> i64 {
        unsafe { igraph_vcount(self.as_ptr()) }
    }

    /// Return the number of edges.
    pub fn ecount(&self) -> i64 {
        unsafe { igraph_ecount(self.as_ptr()) }
    }

    /// Return whether the graph is directed.
    pub fn is_directed(&self) -> bool {
        unsafe { igraph_is_directed(self.as_ptr()) }
    }

    /// Return the neighbors of vertex `vid`.
    pub fn neighbors(&self, vid: i64, mode: NeighborMode) -> Result<Vec<i64>> {
        let mut neis = VectorInt::new()?;
        unsafe {
            check(igraph_neighbors(
                self.as_ptr(),
                neis.as_mut_ptr(),
                vid,
                mode.to_raw(),
                igraph_loops_t_IGRAPH_LOOPS_TWICE,
                true, // include multiple edges
            ))?;
        }
        Ok(neis.to_vec())
    }

    /// Return the degree of all vertices.
    pub fn degree(&self, mode: NeighborMode, loops: Loops) -> Result<Vec<i64>> {
        let mut res = VectorInt::new()?;
        unsafe {
            check(igraph_degree(
                self.as_ptr(),
                res.as_mut_ptr(),
                igraph_vss_all(),
                mode.to_raw(),
                loops.to_raw(),
            ))?;
        }
        Ok(res.to_vec())
    }

    /// Return the endpoints of edge `eid` as `(from, to)`.
    pub fn edge(&self, eid: i64) -> Result<(i64, i64)> {
        let mut from: i64 = 0;
        let mut to: i64 = 0;
        unsafe {
            check(igraph_edge(self.as_ptr(), eid, &mut from, &mut to))?;
        }
        Ok((from, to))
    }

    /// Return whether vertices `v1` and `v2` are adjacent (connected by an edge).
    pub fn are_adjacent(&self, v1: i64, v2: i64) -> Result<bool> {
        let mut res: bool = false;
        unsafe {
            check(igraph_are_adjacent(self.as_ptr(), v1, v2, &mut res))?;
        }
        Ok(res)
    }

    /// Return the edge list as a vector of `(from, to)` pairs.
    pub fn get_edgelist(&self) -> Result<Vec<(i64, i64)>> {
        let mut ev = VectorInt::new()?;
        unsafe {
            check(igraph_get_edgelist(self.as_ptr(), ev.as_mut_ptr(), false))?;
        }
        let flat = ev.to_vec();
        Ok(flat.chunks_exact(2).map(|c| (c[0], c[1])).collect())
    }
}
