use igraph_sys::igraph_error_type_t;

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug, Clone, Copy, PartialEq, Eq, thiserror::Error)]
pub enum Error {
    #[error("generic igraph failure")]
    Failure,
    #[error("out of memory")]
    NoMemory,
    #[error("parse error")]
    ParseError,
    #[error("invalid value")]
    InvalidValue,
    #[error("element already exists")]
    Exists,
    #[error("invalid vertex id")]
    InvalidVertexId,
    #[error("invalid edge id")]
    InvalidEdgeId,
    #[error("invalid mode argument")]
    InvalidMode,
    #[error("file operation error")]
    FileError,
    #[error("unimplemented function")]
    Unimplemented,
    #[error("interrupted")]
    Interrupted,
    #[error("algorithm diverged")]
    Diverged,
    #[error("ARPACK error")]
    ArpackError,
    #[error("negative cycle found")]
    NegativeCycle,
    #[error("internal error")]
    Internal,
    #[error("attribute combination error")]
    AttributeCombine,
    #[error("integer overflow")]
    Overflow,
    #[error("integer underflow")]
    Underflow,
    #[error("random walk stuck")]
    RandomWalkStuck,
    #[error("stop signal")]
    Stop,
    #[error("value out of range")]
    Range,
    #[error("no solution found")]
    NoSolution,
    #[error("unknown igraph error code: {0}")]
    Unknown(u32),
}

#[allow(non_upper_case_globals)]
pub(crate) fn check(code: igraph_error_type_t) -> Result<()> {
    use igraph_sys::*;
    match code {
        igraph_error_type_t_IGRAPH_SUCCESS => Ok(()),
        igraph_error_type_t_IGRAPH_FAILURE => Err(Error::Failure),
        igraph_error_type_t_IGRAPH_ENOMEM => Err(Error::NoMemory),
        igraph_error_type_t_IGRAPH_PARSEERROR => Err(Error::ParseError),
        igraph_error_type_t_IGRAPH_EINVAL => Err(Error::InvalidValue),
        igraph_error_type_t_IGRAPH_EXISTS => Err(Error::Exists),
        igraph_error_type_t_IGRAPH_EINVVID => Err(Error::InvalidVertexId),
        igraph_error_type_t_IGRAPH_EINVEID => Err(Error::InvalidEdgeId),
        igraph_error_type_t_IGRAPH_EINVMODE => Err(Error::InvalidMode),
        igraph_error_type_t_IGRAPH_EFILE => Err(Error::FileError),
        igraph_error_type_t_IGRAPH_UNIMPLEMENTED => Err(Error::Unimplemented),
        igraph_error_type_t_IGRAPH_INTERRUPTED => Err(Error::Interrupted),
        igraph_error_type_t_IGRAPH_DIVERGED => Err(Error::Diverged),
        igraph_error_type_t_IGRAPH_EARPACK => Err(Error::ArpackError),
        igraph_error_type_t_IGRAPH_ENEGCYCLE => Err(Error::NegativeCycle),
        igraph_error_type_t_IGRAPH_EINTERNAL => Err(Error::Internal),
        igraph_error_type_t_IGRAPH_EATTRCOMBINE => Err(Error::AttributeCombine),
        igraph_error_type_t_IGRAPH_EOVERFLOW => Err(Error::Overflow),
        igraph_error_type_t_IGRAPH_EUNDERFLOW => Err(Error::Underflow),
        igraph_error_type_t_IGRAPH_ERWSTUCK => Err(Error::RandomWalkStuck),
        igraph_error_type_t_IGRAPH_STOP => Err(Error::Stop),
        igraph_error_type_t_IGRAPH_ERANGE => Err(Error::Range),
        igraph_error_type_t_IGRAPH_ENOSOL => Err(Error::NoSolution),
        other => Err(Error::Unknown(other)),
    }
}
