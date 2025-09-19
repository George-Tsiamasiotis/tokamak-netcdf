use crate::extract::{extract_1d_var, extract_scalar};
use crate::{B_AXIS, PHI_TOR_EDGE, PSI_COORD, PSI_POL_AXIS, PSI_POL_EDGE, R_AXIS, Z_AXIS};
use crate::{NcError, Result};

use ndarray::Array1;

#[non_exhaustive]
#[derive(Debug)]
/// Equilibrium scalar values.
pub struct Scalars {
    /// Magnetic filed strength on the axis **in \[T\]**.
    pub baxis: f64,
    /// The tokamak's major radius **in \[m\]**.
    pub raxis: f64,
    /// Last closed surface **in Normalized Units**.
    pub psi_wall: f64,
    /// The tokamak's Z coordinate on the axis **in \[m\]**.
    pub zaxis: f64,
    /// Poloidal flux at at axis **in \[Tm²\]**.
    pub psi_pol_axis: f64,
    // WARN: Description/name might be wrong.
    /// Poloidal flux at the edge **in \[Tm²\]**.
    pub psi_pol_edge: f64,
    // WARN: Description/name might be wrong.
    /// Toroidal flux at the edge **in \[Tm²\]**.
    pub phi_tor_edge: f64,
}

impl Scalars {
    /// Creates a [`Scalars`] containing the needed scalar values from the netCDF file.
    pub(crate) fn from_file(f: &netcdf::File) -> Result<Self> {
        let psi: Array1<f64> = extract_1d_var(f, "psi")?;
        debug_assert!(psi.iter().is_sorted());

        Ok(Self {
            baxis: extract_scalar(f, B_AXIS)?,
            raxis: extract_scalar(f, R_AXIS)?,
            zaxis: extract_scalar(f, Z_AXIS)?,
            psi_pol_axis: extract_scalar(f, PSI_POL_AXIS)?,
            psi_pol_edge: extract_scalar(f, PSI_POL_EDGE)?,
            phi_tor_edge: extract_scalar(f, PHI_TOR_EDGE)?,
            psi_wall: *psi.last().ok_or(NcError::EmptyVariable(PSI_COORD.into()))?,
        })
    }
}
