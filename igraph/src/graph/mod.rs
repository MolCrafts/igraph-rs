mod centrality;
mod community;
mod components;
mod constructors;
mod isomorphism;
mod mutation;
mod paths;
mod query;
mod transform;

use std::mem::MaybeUninit;

use igraph_sys::*;

/// A safe wrapper around the igraph graph type.
///
/// `Graph` owns an `igraph_t` and automatically frees it when dropped.
/// All methods provide safe access to igraph functionality without
/// exposing raw pointers.
pub struct Graph {
    pub(crate) inner: igraph_t,
}

impl Graph {
    /// Create a `Graph` from a raw `igraph_t` that has already been initialized.
    ///
    /// # Safety
    /// The caller must ensure `raw` was properly initialized by an igraph function.
    pub(crate) unsafe fn from_raw(raw: igraph_t) -> Self {
        Self { inner: raw }
    }

    /// Helper to create a graph via a closure that initializes an `igraph_t`.
    pub(crate) fn init_with(
        f: impl FnOnce(*mut igraph_t) -> igraph_error_type_t,
    ) -> crate::error::Result<Self> {
        let mut g = MaybeUninit::uninit();
        let code = f(g.as_mut_ptr());
        crate::error::check(code)?;
        Ok(unsafe { Self::from_raw(g.assume_init()) })
    }

    pub(crate) fn as_ptr(&self) -> *const igraph_t {
        &self.inner
    }

    pub(crate) fn as_mut_ptr(&mut self) -> *mut igraph_t {
        &mut self.inner
    }
}

impl Drop for Graph {
    fn drop(&mut self) {
        unsafe { igraph_destroy(&mut self.inner) }
    }
}

// Safety: With IGRAPH_ENABLE_TLS=ON, igraph's global state (error handlers,
// RNG, progress handlers) is thread-local, so an owned `Graph` can safely be
// moved to another thread. We intentionally do NOT implement `Sync` because
// `igraph_t` contains an internal property cache that may be mutated by
// seemingly read-only C functions, making concurrent `&Graph` access unsafe.
unsafe impl Send for Graph {}
