//! A crate for handling [`netCDF`] files from [`Tokamak`] reconstructed equilibria.
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
//! let nc_data = NcData::from_file(&path)?;
//! # Ok(())
//! # }
//! ```
//!
//! This crate requires the [`netCDF-C`] library, which is available in most linux package managers.
//!
//! `libnetcdf` can be statically linked with the 'static' feature, which is provided by the
//! [`netcdf crate`].
//!
//! [`netCDF`]: https://www.unidata.ucar.edu/software/netcdf
//! [`netCDF-C`]: https://github.com/Unidata/netcdf-c
//! [`netcdf crate`]: https://github.com/georust/netcdf
//! [`Tokamak`]: https://en.wikipedia.org/wiki/Tokamak

pub use crate::error::NcError;

mod error;
mod extract;
mod ncdata;
mod scalars;
pub mod variable_names;

pub type Result<T> = std::result::Result<T, NcError>;

pub use ncdata::NcData;
pub use scalars::Scalars;

#[doc(inline)]
pub use variable_names::*;
