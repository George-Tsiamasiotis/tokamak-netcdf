#![doc = include_str!("../README.md")]
//!
//! # Example
//!
//! ```no_run
//! # use std::path::PathBuf;
//! # use tokamak_netcdf::*;
//! #
//! # fn main() -> Result<()> {
//! #
//! // Path must be relative to the directory where "cargo run" is called
//! let path = PathBuf::from(r"./data.nc");
//! let nc_data = Equilibrium::from_file(&path)?;
//! # Ok(())
//! # }
//! ```
pub use crate::error::NcError;

mod equilibrium;
mod error;
pub mod extract;
pub mod variable_names;

pub type Result<T> = std::result::Result<T, NcError>;

pub use equilibrium::Equilibrium;

#[doc(inline)]
pub use extract::*;
