use std::ffi::CString;

use igraph_sys::*;

use super::Graph;
use crate::error::{Result, check};
use crate::types::{StarMode, TreeMode};
use crate::vector::VectorInt;

impl Graph {
    /// Create an empty graph with `n` vertices and no edges.
    pub fn empty(n: i64, directed: bool) -> Result<Self> {
        Self::init_with(|g| unsafe { igraph_empty(g, n, directed) })
    }

    /// Create a graph from edge pairs.
    ///
    /// `edges` is a slice of `(from, to)` pairs. `n` is the number of vertices
    /// (use 0 to infer from edge list).
    pub fn from_edges(edges: &[(i64, i64)], n: i64, directed: bool) -> Result<Self> {
        let mut ev = VectorInt::new()?;
        for &(from, to) in edges {
            unsafe {
                check(igraph_vector_int_push_back(ev.as_mut_ptr(), from))?;
                check(igraph_vector_int_push_back(ev.as_mut_ptr(), to))?;
            }
        }
        Self::init_with(|g| unsafe { igraph_create(g, ev.as_ptr(), n, directed) })
    }

    /// Create a famous graph by name (e.g. "Petersen", "Tutte", "Zachary").
    pub fn famous(name: &str) -> Result<Self> {
        let cname = CString::new(name).map_err(|_| crate::error::Error::InvalidValue)?;
        Self::init_with(|g| unsafe { igraph_famous(g, cname.as_ptr()) })
    }

    /// Create a ring (cycle) graph.
    pub fn ring(n: i64, directed: bool, mutual: bool, circular: bool) -> Result<Self> {
        Self::init_with(|g| unsafe { igraph_ring(g, n, directed, mutual, circular) })
    }

    /// Create a star graph.
    pub fn star(n: i64, mode: StarMode, center: i64) -> Result<Self> {
        Self::init_with(|g| unsafe { igraph_star(g, n, mode.to_raw(), center) })
    }

    /// Create a full (complete) graph.
    pub fn full(n: i64, directed: bool, loops: bool) -> Result<Self> {
        Self::init_with(|g| unsafe { igraph_full(g, n, directed, loops) })
    }

    /// Create a k-ary tree.
    pub fn kary_tree(n: i64, children: i64, mode: TreeMode) -> Result<Self> {
        Self::init_with(|g| unsafe { igraph_kary_tree(g, n, children, mode.to_raw()) })
    }

    /// Create an Erdos-Renyi G(n,p) random graph.
    pub fn erdos_renyi_gnp(n: i64, p: f64, directed: bool, loops: bool) -> Result<Self> {
        let edge_types = if loops {
            IGRAPH_LOOPS_SW as igraph_edge_type_sw_t
        } else {
            IGRAPH_SIMPLE_SW as igraph_edge_type_sw_t
        };
        Self::init_with(|g| unsafe {
            igraph_erdos_renyi_game_gnp(g, n, p, directed, edge_types, false)
        })
    }

    /// Create an Erdos-Renyi G(n,m) random graph.
    pub fn erdos_renyi_gnm(n: i64, m: i64, directed: bool, loops: bool) -> Result<Self> {
        let edge_types = if loops {
            IGRAPH_LOOPS_SW as igraph_edge_type_sw_t
        } else {
            IGRAPH_SIMPLE_SW as igraph_edge_type_sw_t
        };
        Self::init_with(|g| unsafe {
            igraph_erdos_renyi_game_gnm(g, n, m, directed, edge_types, false)
        })
    }

    /// Create a Barabasi-Albert preferential attachment graph.
    pub fn barabasi(n: i64, m: i64, directed: bool) -> Result<Self> {
        Self::init_with(|g| unsafe {
            igraph_barabasi_game(
                g,
                n,
                1.0, // power
                m,
                std::ptr::null(), // outseq
                false,            // outpref
                1.0,              // A
                directed,
                igraph_barabasi_algorithm_t_IGRAPH_BARABASI_PSUMTREE,
                std::ptr::null(), // start_from
            )
        })
    }
}
