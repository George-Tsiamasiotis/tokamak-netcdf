use std::fmt::Debug;
use std::path::PathBuf;

use ndarray::{Array1, Array2};

use crate::Result;

#[allow(unused_imports)] // Needed for documentation fields.
use crate::variable_names::*;

#[derive(Debug)]
/// Tokamak Equilibrium Representation.
///
/// Provides methods for extracting the needed fields from the netCDF file.
pub struct Equilibrium {
    /// Path to netCDF file,
    pub path: PathBuf,
    /// The netCDF file.
    pub file: netcdf::File,
}

impl Equilibrium {
    /// Creates an equilibrium representation from a netCDF file.
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
    /// let eq = Equilibrium::from_file(&path)?;
    /// # Ok(())
    /// # }
    /// ```
    pub fn from_file(path: &PathBuf) -> Result<Self> {
        use crate::NcError::*;

        if !path.exists() {
            return Err(FileNotFound(path.clone()));
        }

        // If this fails, its due to an underlying library error
        let file = match netcdf::open(path) {
            Ok(nc_file) => nc_file,
            Err(liberror) => {
                return Err(NetCDF {
                    source: liberror,
                    reason: "Error opening NetCDF file.".into(),
                });
            }
        };

        Ok(Self {
            path: path.clone(),
            file,
        })
    }

    /// Returns a scalar variable form the netCDF file.
    ///
    /// Available fields are [`B_AXIS`], [`R_AXIS`], [`Z_AXIS`], [`PSI_POL_AXIS`],
    /// [`PSI_POL_EDGE`] and [`PHI_TOR_EDGE`], which are defined in [`crate::variable_names`].
    ///
    /// # Example
    ///
    /// ```no_run
    /// # use std::path::PathBuf;
    /// # use tokamak_netcdf::*;
    /// # use tokamak_netcdf::variable_names::*;
    /// #
    /// # fn main() -> Result<()> {
    /// let path = PathBuf::from(r"./data.nc");
    /// let eq = Equilibrium::from_file(&path)?;
    /// let psi = eq.get_scalar(B_AXIS);
    /// # Ok(())
    /// # }
    /// ```
    pub fn get_scalar(&self, name: &str) -> Result<f64> {
        use crate::variable_names::*;

        match name {
            B_AXIS | R_AXIS | Z_AXIS | PSI_POL_AXIS | PSI_POL_EDGE | PHI_TOR_EDGE => {
                crate::extract_scalar(&self.file, name)
            }
            _ => Err(crate::NcError::VariableNotFound(name.into())),
        }
    }

    /// Returns a 1-dimensional variable form the netCDF file.
    ///
    /// Available fields are [`PSI_COORD`], [`THETA_COORD`], [`Q_FACTOR`], [`CURRENT_G`] and
    /// [`CURRENT_I`], which are defined in [`crate::variable_names`].
    ///
    /// # Example
    ///
    /// ```no_run
    /// # use std::path::PathBuf;
    /// # use tokamak_netcdf::*;
    /// # use tokamak_netcdf::variable_names::*;
    /// #
    /// # fn main() -> Result<()> {
    /// let path = PathBuf::from(r"./data.nc");
    /// let eq = Equilibrium::from_file(&path)?;
    /// let psi = eq.get_1d(PSI_COORD);
    /// # Ok(())
    /// # }
    /// ```
    pub fn get_1d(&self, name: &str) -> Result<Array1<f64>> {
        use crate::variable_names::*;

        match name {
            PSI_COORD | THETA_COORD | Q_FACTOR | CURRENT_G | CURRENT_I => {
                crate::extract_1d_var(&self.file, name)
            }
            _ => Err(crate::NcError::VariableNotFound(name.into())),
        }
    }

    /// Returns a 2-dimensional variable form the netCDF file.
    ///
    /// Available fields are [`B_FIELD`], [`DB_DTHETA`], [`DB_DPSI`] and [`D2B_DPSI2`], which
    /// are defined in [`crate::variable_names`].
    ///
    /// # Example
    ///
    /// ```ignore
    /// # use std::path::PathBuf;
    /// # use tokamak_netcdf::*;
    /// # use tokamak_netcdf::variable_names::*;
    /// #
    /// # fn main() -> Result<()> {
    /// let path = PathBuf::from(r"./data.nc");
    /// let eq = Equilibrium::from_file(&path)?;
    /// let psi = eq.get_2d(B_FIELD);
    /// # Ok(())
    /// # }
    /// ```
    pub fn get_2d(&self, name: &str) -> Result<Array2<f64>> {
        use crate::variable_names::*;

        match name {
            B_FIELD | DB_DTHETA | DB_DPSI | D2B_DPSI2 => todo!(),
            _ => Err(crate::NcError::VariableNotFound(name.into())),
        }
    }
}

#[cfg(test)]
mod test {
    use crate::Equilibrium;
    use crate::variable_names::*;
    use std::path::PathBuf;

    #[test]
    #[ignore = "needs real dataset"]
    fn test_real_nc_data() {
        let path = PathBuf::from("./reconstructed/smart_positive.nc");
        let file = Equilibrium::from_file(&path).unwrap();

        file.get_scalar(B_AXIS).unwrap();
        file.get_scalar(R_AXIS).unwrap();
        file.get_scalar(Z_AXIS).unwrap();

        file.get_1d(PSI_COORD).unwrap();
        file.get_1d(THETA_COORD).unwrap();

        file.get_1d(Q_FACTOR).unwrap();
        file.get_1d(CURRENT_I).unwrap();
        file.get_1d(CURRENT_G).unwrap();
    }
}
