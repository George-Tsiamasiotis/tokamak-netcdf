//! Functions for extracting and checking data from the netCDF file.

use crate::NcError;
use crate::Result;

use ndarray::Array1;
use netcdf::{NcTypeDescriptor, Variable};

/// Extracts a [`Variable`] from a netCDF File.
pub(crate) fn extract_variable<'f>(f: &'f netcdf::File, name: &str) -> Result<Variable<'f>> {
    match f.variable(name) {
        Some(var) => Ok(var),
        None => Err(NcError::VariableNotFound(name.into())),
    }
}

/// Checks if a [`Variable`] is empty.
fn check_if_empty(var: &Variable) -> Result<()> {
    match var.len() {
        1.. => Ok(()),
        0 => Err(NcError::EmptyVariable(var.name().into())),
    }
}

pub(crate) fn extract_scalar<T>(f: &netcdf::File, name: &str) -> Result<T>
where
    T: NcTypeDescriptor + Copy,
{
    use crate::NcError::*;

    let var = extract_variable(f, name)?;
    check_if_empty(&var)?;

    // `var.dimensions()` is () for netcdf's scalar `Variables`. This is probably equivalent to
    // `var.len() == 0`.
    if !var.dimensions().is_empty() {
        return Err(NotScalar(name.into()));
    }

    match var.get_value::<T, _>(..) {
        Ok(value) => Ok(value),
        Err(err) => Err(GetValuesError {
            source: err,
            name: var.name().into(),
        }),
    }
}

/// Extracts a 1D [`Variable`].
pub(crate) fn extract_1d_var<T>(f: &netcdf::File, name: &str) -> Result<Array1<T>>
where
    T: NcTypeDescriptor + Copy + Default,
{
    let var = extract_variable(f, name)?;
    check_if_empty(&var)?;

    if var.dimensions().len() != 1 {
        return Err(NcError::Not1D(name.into()));
    }

    let mut data = Array1::<T>::default(var.len());

    match var.get_into(data.view_mut(), ..) {
        Ok(()) => Ok(data),
        Err(err) => Err(NcError::GetValuesError {
            source: err,
            name: name.into(),
        }),
    }
}
