//! Handles netCDF file opening and [`NcData`] creation.

use std::fmt::Debug;
use std::path::PathBuf;

use crate::Result;
use crate::Scalars;

#[non_exhaustive]
#[derive(Debug)]
/// netCDF equilibrium data.
pub struct NcData {
    /// Path to netCDF file,
    pub path: PathBuf,
    /// Equilibrium's scalar values.
    pub scalars: Scalars,
}

impl NcData {
    /// Creates an [`NcData`] from a netCDF file.
    ///
    /// # Example
    ///
    /// ```no_run
    /// # use std::path::PathBuf;
    /// # use tokamak_netcdf::*;
    /// #
    /// # fn main() -> Result<()> {
    /// // Path must be relative to the directory where "cargo run" is called
    /// let path = PathBuf::from(r"./data.nc");
    /// let nc_data = NcData::from_file(&path)?;
    /// # Ok(())
    /// # }
    /// ```
    pub fn from_file(path: &PathBuf) -> Result<Self> {
        use crate::NcError::*;

        if !path.exists() {
            return Err(FileNotFound(path.clone()));
        }

        // If this fails, its due to an underlying library error
        let nc_file = match netcdf::open(path) {
            Ok(nc_file) => nc_file,
            Err(liberror) => {
                return Err(NetCDF {
                    source: liberror,
                    reason: "Error opening NetCDF file.".into(),
                });
            }
        };

        let scalars = Scalars::from_file(&nc_file)?;

        Ok(NcData {
            path: path.clone(),
            scalars,
        })
    }
}

#[cfg(test)]
mod test {
    use crate::*;
    use std::path::PathBuf;

    #[test]
    #[ignore = "needs real dataset"]
    fn test_real_nc_data() {
        let path = PathBuf::from("./reconstructed/smart_positive.nc");
        let nc_data = NcData::from_file(&path).unwrap();

        println!("{nc_data:#?}");
    }
}
