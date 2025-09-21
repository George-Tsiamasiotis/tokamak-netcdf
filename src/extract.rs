//! Functions for extracting and checking data from the netCDF file.

use crate::NcError;
use crate::Result;

use ndarray::{Array1, Array2};
use netcdf::Variable;

/// Extracts a [`Variable`] from a netCDF File.
///
/// # Error
///
/// Returns an [`NcError`] if the variable not found.
pub fn extract_variable<'f>(f: &'f netcdf::File, name: &str) -> Result<Variable<'f>> {
    match f.variable(name) {
        Some(var) => Ok(var),
        None => Err(NcError::VariableNotFound(name.into())),
    }
}

/// Checks if a [`Variable`] is empty.
///
/// # Error
///
/// Returns an [`NcError`] if the variable is empty.
pub fn check_if_empty(var: &Variable) -> Result<()> {
    match var.len() {
        1.. => Ok(()),
        0 => Err(NcError::EmptyVariable(var.name().into())),
    }
}

/// Extracts a scalar value from the netCDF file.
///
/// # Error
///
/// Retruns an [`NcError`] if the variable:
///
/// - is not found,
/// - is empty,
/// - is not scalar (has dimensions).
pub fn extract_scalar(f: &netcdf::File, name: &str) -> Result<f64> {
    use crate::NcError::*;

    let var = extract_variable(f, name)?;
    check_if_empty(&var)?;

    // `var.dimensions()` is () for netcdf's scalar `Variables`. This is probably equivalent to
    // `var.len() == 0`.
    if !var.dimensions().is_empty() {
        return Err(NotScalar(name.into()));
    }

    match var.get_value::<f64, _>(..) {
        Ok(value) => Ok(value),
        Err(err) => Err(GetValuesError {
            source: err,
            name: var.name().into(),
        }),
    }
}

/// Extracts a 1D [`Variable`].
///
/// # Error
///
/// Retruns an [`NcError`] if the variable:
///
/// - is not found,
/// - is empty,
/// - is not 1-dimensional.
pub fn extract_1d_var(f: &netcdf::File, name: &str) -> Result<Array1<f64>> {
    let var = extract_variable(f, name)?;
    check_if_empty(&var)?;

    if var.dimensions().len() != 1 {
        return Err(NcError::Not1D(name.into()));
    }

    let mut data = Array1::from_elem(var.len(), f64::NAN);

    match var.get_into(data.view_mut(), ..) {
        Err(err) => Err(NcError::GetValuesError {
            source: err,
            name: name.into(),
        }),
        Ok(()) => Ok(data),
    }
}

/// Extracts a 2D [`Variable`]
///
/// Retruns an [`NcError`] if the variable:
///
/// - is not found,
/// - is empty,
/// - is not 2-dimensional.
pub fn extract_2d_var(f: &netcdf::File, name: &str) -> Result<Array2<f64>> {
    let var = extract_variable(f, name)?;
    check_if_empty(&var)?;

    if var.dimensions().len() != 2 {
        return Err(NcError::Not2D(var.name().into()));
    }

    // Dimension order is (ψ, θ).
    let dims = var.dimensions().to_vec();
    let shape = (dims[0].len(), dims[1].len());
    let mut data = Array2::<f64>::from_elem(shape, f64::NAN);

    match var.get_into(data.view_mut(), (.., ..)) {
        Err(err) => Err(NcError::GetValuesError {
            source: err,
            name: var.name().into(),
        }),
        Ok(()) => Ok(data),
    }
}
