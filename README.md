# tokamak-netcdf

A crate for handling [`netCDF`] files from [`Tokamak`] reconstructed equilibria.

This crate requires the [`netCDF-C`] library, which is available in most linux package managers.

`libnetcdf` can be statically linked with the 'static' feature, which is provided by the
[`netcdf crate`].

[`netCDF`]: https://www.unidata.ucar.edu/software/netcdf
[`netCDF-C`]: https://github.com/Unidata/netcdf-c
[`netcdf crate`]: https://github.com/georust/netcdf
[`Tokamak`]: https://en.wikipedia.org/wiki/Tokamak