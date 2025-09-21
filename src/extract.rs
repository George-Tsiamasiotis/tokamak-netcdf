//! Functions for extracting and checking data from the netCDF file.

use crate::NcError;
use crate::Result;

use ndarray::{Array1, Array2, ArrayView1, Axis, array};
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

/// Extracts a variable from the NetCDF file and prepends the first value.
///
/// The first value is the closest to the magnetic axis at index 0.
pub fn extract_var_with_first_axis_value(f: &netcdf::File, name: &str) -> Result<Array1<f64>> {
    let arr: Array1<f64> = extract_1d_var(f, name)?;
    extract_var_with_axis_value(f, name, arr[0])
}

/// Extracts a variable from the NetCDF file and prepends `element` at index 0.
pub fn extract_var_with_axis_value(
    f: &netcdf::File,
    name: &str,
    element: f64,
) -> Result<Array1<f64>> {
    let arr: Array1<f64> = extract_1d_var(f, name)?;
    let view = ArrayView1::from(&arr);
    let mut prepended: Array1<f64> = array![element];

    // This is not expected to fail since both arrays are guranteed to be of the same shape (1,).
    match prepended.append(Axis(0), view) {
        Ok(()) => Ok(prepended),
        Err(_) => unreachable!("Shape mismatch in prepending axis value."),
    }
}

#[cfg(test)]
mod test {
    use super::*;

    static VAR_LENGTH: usize = 5;

    /// Creates a phony NetCDF file for use across the tests.
    fn phony_netcdf() -> std::result::Result<netcdf::FileMut, netcdf::Error> {
        let path = std::env::temp_dir().join("phony.nc");
        let path_str = path.to_str().unwrap();

        let mut f = netcdf::create(path_str)?;
        std::fs::remove_file(path).unwrap();

        f.add_dimension("dim1", VAR_LENGTH)?;
        f.add_dimension("dim2", VAR_LENGTH)?;
        f.add_variable::<f64>("var", &["dim2"])?;

        f.add_dimension("empty_dim", 0)?;
        f.add_variable::<f64>("empty_var", &["empty_dim"])?;

        f.add_variable::<f64>("2dvar", &["dim1", "dim2"])?;
        f.add_variable::<f64>("float_var", &["dim1"])?;

        f.add_variable::<i32>("number", &[])?
            .put_values(&[18], ..)?;
        Ok(f)
    }

    #[test]
    fn test_axis_value() {
        let mut f = phony_netcdf().unwrap();
        let data: [f64; VAR_LENGTH] = [2.0, 3.0, 4.0, 5.0, 6.0];

        f.variable_mut("float_var")
            .expect("Error extracting mutable variable.")
            .put_values(&data, ..)
            .expect("Error putting values to variable");

        assert_eq!(
            Array1::<f64>::from_vec(vec![1.0, 2.0, 3.0, 4.0, 5.0, 6.0]),
            extract_var_with_axis_value(&f, "float_var", 1.0).unwrap()
        );
        assert_eq!(
            Array1::<f64>::from_vec(vec![2.0, 2.0, 3.0, 4.0, 5.0, 6.0]),
            extract_var_with_first_axis_value(&f, "float_var").unwrap()
        );
    }
}
