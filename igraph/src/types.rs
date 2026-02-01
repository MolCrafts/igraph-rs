use igraph_sys::*;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum NeighborMode {
    Out,
    In,
    All,
}

impl NeighborMode {
    pub(crate) fn to_raw(self) -> igraph_neimode_t {
        match self {
            NeighborMode::Out => igraph_neimode_t_IGRAPH_OUT,
            NeighborMode::In => igraph_neimode_t_IGRAPH_IN,
            NeighborMode::All => igraph_neimode_t_IGRAPH_ALL,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Connectedness {
    Weak,
    Strong,
}

impl Connectedness {
    pub(crate) fn to_raw(self) -> igraph_connectedness_t {
        match self {
            Connectedness::Weak => igraph_connectedness_t_IGRAPH_WEAK,
            Connectedness::Strong => igraph_connectedness_t_IGRAPH_STRONG,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Loops {
    No,
    Twice,
    Once,
}

impl Loops {
    pub(crate) fn to_raw(self) -> igraph_loops_t {
        match self {
            Loops::No => igraph_loops_t_IGRAPH_NO_LOOPS,
            Loops::Twice => igraph_loops_t_IGRAPH_LOOPS_TWICE,
            Loops::Once => igraph_loops_t_IGRAPH_LOOPS_ONCE,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum StarMode {
    Out,
    In,
    Undirected,
    Mutual,
}

impl StarMode {
    pub(crate) fn to_raw(self) -> igraph_star_mode_t {
        match self {
            StarMode::Out => igraph_star_mode_t_IGRAPH_STAR_OUT,
            StarMode::In => igraph_star_mode_t_IGRAPH_STAR_IN,
            StarMode::Undirected => igraph_star_mode_t_IGRAPH_STAR_UNDIRECTED,
            StarMode::Mutual => igraph_star_mode_t_IGRAPH_STAR_MUTUAL,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TreeMode {
    Out,
    In,
    Undirected,
}

impl TreeMode {
    pub(crate) fn to_raw(self) -> igraph_tree_mode_t {
        match self {
            TreeMode::Out => igraph_tree_mode_t_IGRAPH_TREE_OUT,
            TreeMode::In => igraph_tree_mode_t_IGRAPH_TREE_IN,
            TreeMode::Undirected => igraph_tree_mode_t_IGRAPH_TREE_UNDIRECTED,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ToDirectedMode {
    Arbitrary,
    Mutual,
    Random,
    Acyclic,
}

impl ToDirectedMode {
    pub(crate) fn to_raw(self) -> igraph_to_directed_t {
        match self {
            ToDirectedMode::Arbitrary => igraph_to_directed_t_IGRAPH_TO_DIRECTED_ARBITRARY,
            ToDirectedMode::Mutual => igraph_to_directed_t_IGRAPH_TO_DIRECTED_MUTUAL,
            ToDirectedMode::Random => igraph_to_directed_t_IGRAPH_TO_DIRECTED_RANDOM,
            ToDirectedMode::Acyclic => igraph_to_directed_t_IGRAPH_TO_DIRECTED_ACYCLIC,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ToUndirectedMode {
    Each,
    Collapse,
    Mutual,
}

impl ToUndirectedMode {
    pub(crate) fn to_raw(self) -> igraph_to_undirected_t {
        match self {
            ToUndirectedMode::Each => igraph_to_undirected_t_IGRAPH_TO_UNDIRECTED_EACH,
            ToUndirectedMode::Collapse => igraph_to_undirected_t_IGRAPH_TO_UNDIRECTED_COLLAPSE,
            ToUndirectedMode::Mutual => igraph_to_undirected_t_IGRAPH_TO_UNDIRECTED_MUTUAL,
        }
    }
}
