use std::mem::MaybeUninit;

use igraph_sys::*;

use crate::error::{Result, check};

/// Safe wrapper around `igraph_vector_int_t` (vector of `i64`).
pub struct VectorInt {
    pub(crate) inner: igraph_vector_int_t,
}

impl VectorInt {
    pub fn new() -> Result<Self> {
        let mut v = MaybeUninit::uninit();
        unsafe {
            check(igraph_vector_int_init(v.as_mut_ptr(), 0))?;
            Ok(Self {
                inner: v.assume_init(),
            })
        }
    }

    pub fn with_size(n: i64) -> Result<Self> {
        let mut v = MaybeUninit::uninit();
        unsafe {
            check(igraph_vector_int_init(v.as_mut_ptr(), n))?;
            Ok(Self {
                inner: v.assume_init(),
            })
        }
    }

    pub fn from_slice(data: &[i64]) -> Result<Self> {
        let mut v = Self::new()?;
        for &val in data {
            unsafe {
                check(igraph_vector_int_push_back(&mut v.inner, val))?;
            }
        }
        Ok(v)
    }

    pub fn len(&self) -> i64 {
        unsafe { igraph_vector_int_size(&self.inner) }
    }

    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    pub fn get(&self, pos: i64) -> i64 {
        unsafe { igraph_vector_int_get(&self.inner, pos) }
    }

    pub fn set(&mut self, pos: i64, value: i64) {
        unsafe { igraph_vector_int_set(&mut self.inner, pos, value) }
    }

    pub fn to_vec(&self) -> Vec<i64> {
        let n = self.len();
        (0..n).map(|i| self.get(i)).collect()
    }

    pub(crate) fn as_ptr(&self) -> *const igraph_vector_int_t {
        &self.inner
    }

    pub(crate) fn as_mut_ptr(&mut self) -> *mut igraph_vector_int_t {
        &mut self.inner
    }
}

impl Drop for VectorInt {
    fn drop(&mut self) {
        unsafe { igraph_vector_int_destroy(&mut self.inner) }
    }
}

// Safety: VectorInt is a self-contained heap allocation with no shared global
// state. Safe to move between threads.
unsafe impl Send for VectorInt {}

/// Safe wrapper around `igraph_vector_t` (vector of `f64`).
pub struct Vector {
    pub(crate) inner: igraph_vector_t,
}

impl Vector {
    pub fn new() -> Result<Self> {
        let mut v = MaybeUninit::uninit();
        unsafe {
            check(igraph_vector_init(v.as_mut_ptr(), 0))?;
            Ok(Self {
                inner: v.assume_init(),
            })
        }
    }

    pub fn with_size(n: i64) -> Result<Self> {
        let mut v = MaybeUninit::uninit();
        unsafe {
            check(igraph_vector_init(v.as_mut_ptr(), n))?;
            Ok(Self {
                inner: v.assume_init(),
            })
        }
    }

    pub fn from_slice(data: &[f64]) -> Result<Self> {
        let mut v = Self::new()?;
        for &val in data {
            unsafe {
                check(igraph_vector_push_back(&mut v.inner, val))?;
            }
        }
        Ok(v)
    }

    pub fn len(&self) -> i64 {
        unsafe { igraph_vector_size(&self.inner) }
    }

    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    pub fn get(&self, pos: i64) -> f64 {
        unsafe { igraph_vector_get(&self.inner, pos) }
    }

    pub fn set(&mut self, pos: i64, value: f64) {
        unsafe { igraph_vector_set(&mut self.inner, pos, value) }
    }

    pub fn to_vec(&self) -> Vec<f64> {
        let n = self.len();
        (0..n).map(|i| self.get(i)).collect()
    }

    pub(crate) fn as_ptr(&self) -> *const igraph_vector_t {
        &self.inner
    }

    pub(crate) fn as_mut_ptr(&mut self) -> *mut igraph_vector_t {
        &mut self.inner
    }
}

impl Drop for Vector {
    fn drop(&mut self) {
        unsafe { igraph_vector_destroy(&mut self.inner) }
    }
}

// Safety: Vector is a self-contained heap allocation with no shared global
// state. Safe to move between threads.
unsafe impl Send for Vector {}

/// Safe wrapper around `igraph_vector_int_list_t` (list of `VectorInt`).
pub struct VectorIntList {
    pub(crate) inner: igraph_vector_int_list_t,
}

impl VectorIntList {
    pub fn new() -> Result<Self> {
        let mut v = MaybeUninit::uninit();
        unsafe {
            check(igraph_vector_int_list_init(v.as_mut_ptr(), 0))?;
            Ok(Self {
                inner: v.assume_init(),
            })
        }
    }

    pub fn len(&self) -> i64 {
        unsafe { igraph_vector_int_list_size(&self.inner) }
    }

    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    /// Get the `pos`-th vector as a `Vec<i64>`.
    pub fn get(&self, pos: i64) -> Vec<i64> {
        unsafe {
            let vptr = igraph_vector_int_list_get_ptr(&self.inner, pos);
            let n = igraph_vector_int_size(&*vptr);
            (0..n).map(|i| igraph_vector_int_get(&*vptr, i)).collect()
        }
    }

    /// Convert the entire list to a `Vec<Vec<i64>>`.
    pub fn to_vec_of_vecs(&self) -> Vec<Vec<i64>> {
        let n = self.len();
        (0..n).map(|i| self.get(i)).collect()
    }

    pub(crate) fn as_mut_ptr(&mut self) -> *mut igraph_vector_int_list_t {
        &mut self.inner
    }
}

impl Drop for VectorIntList {
    fn drop(&mut self) {
        unsafe { igraph_vector_int_list_destroy(&mut self.inner) }
    }
}

// Safety: VectorIntList is a self-contained heap allocation with no shared
// global state. Safe to move between threads.
unsafe impl Send for VectorIntList {}
