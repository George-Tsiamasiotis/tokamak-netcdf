//! Custom Error Type

use std::path::PathBuf;

#[non_exhaustive]
#[derive(thiserror::Error, Debug)]
/// Various error types.
pub enum NcError {
    /// Given path is not a file.
    #[error("'{0}': File not found.")]
    FileNotFound(PathBuf),

    /// Errors from the wrapped netcdf library.
    #[error("Wrapped library error: {reason}")]
    NetCDF {
        #[source]
        source: netcdf::Error,
        reason: Box<str>,
    },

    /// Variable does not exist.
    #[error("'{0}' variable not found.")]
    VariableNotFound(Box<str>),

    /// Variable is empty.
    #[error("'{0}' variable is empty.")]
    EmptyVariable(Box<str>),

    /// Expected scalar value, found array.
    #[error("'{0}' variable is not a scalar.")]
    NotScalar(Box<str>),

    /// Attempted to extract non-1D variable as 1D.
    #[error("'{0}' variable is not 1-dimensional")]
    Not1D(Box<str>),

    /// Attempted to extract non-2D variable as 2D.
    #[error("'{0}' variable is not 2-dimensional")]
    Not2D(Box<str>),

    /// Errors from `netcdf::Variable::get_<>()` functions
    #[error("Error extracting values from '{name}' variable: {source}")]
    GetValuesError {
        #[source]
        source: netcdf::Error,
        name: Box<str>,
    },
}
