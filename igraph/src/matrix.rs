use std::mem::MaybeUninit;

use igraph_sys::*;

use crate::error::{Result, check};

/// Safe wrapper around `igraph_matrix_t` (matrix of `f64`).
pub struct Matrix {
    pub(crate) inner: igraph_matrix_t,
}

impl Matrix {
    pub fn new(nrow: i64, ncol: i64) -> Result<Self> {
        let mut m = MaybeUninit::uninit();
        unsafe {
            check(igraph_matrix_init(m.as_mut_ptr(), nrow, ncol))?;
            Ok(Self {
                inner: m.assume_init(),
            })
        }
    }

    pub fn nrow(&self) -> i64 {
        unsafe { igraph_matrix_nrow(&self.inner) }
    }

    pub fn ncol(&self) -> i64 {
        unsafe { igraph_matrix_ncol(&self.inner) }
    }

    pub fn get(&self, row: i64, col: i64) -> f64 {
        unsafe { igraph_matrix_get(&self.inner, row, col) }
    }

    pub fn to_vec_of_vecs(&self) -> Vec<Vec<f64>> {
        let nrow = self.nrow();
        let ncol = self.ncol();
        (0..nrow)
            .map(|r| (0..ncol).map(|c| self.get(r, c)).collect())
            .collect()
    }

    pub(crate) fn as_mut_ptr(&mut self) -> *mut igraph_matrix_t {
        &mut self.inner
    }
}

impl Drop for Matrix {
    fn drop(&mut self) {
        unsafe { igraph_matrix_destroy(&mut self.inner) }
    }
}

// Safety: Matrix is a self-contained heap allocation with no shared global
// state. Safe to move between threads.
unsafe impl Send for Matrix {}
