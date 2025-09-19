//! The names each variable is expected to appear in the netCDF file.

// ================== Scalars ==================

/// Magnetic field strength on the axis **in \[T\]**.
pub const B_AXIS: &str = "Baxis";
/// The tokamak's major radius **in \[m\]**.
pub const R_AXIS: &str = "raxis";
/// The tokamak's `Z` coordinate of the axis **in \[m\]**.
pub const Z_AXIS: &str = "zaxis";
/// Poloidal flux at at axis **in \[Tm²\]**.
pub const PSI_POL_AXIS: &str = "psi_pol_axis";
// WARN: Description/name might be wrong.
/// Poloidal flux at the edge **in \[Tm²\]**.
pub const PSI_POL_EDGE: &str = "psi_pol_edge";
// WARN: Description/name might be wrong.
/// Toroidal flux at the edge **in \[Tm²\]**.
pub const PHI_TOR_EDGE: &str = "phi_tor_edge";

// ================= Coordinates =================

/// The boozer poloidal angle **in \[rads\]**.
pub const PSI_COORD: &str = "psi";
/// The toroidal flux **in Normalized Units**.
pub const THETA_COORD: &str = "boozer_theta";

// ================ 1D Variables ================

/// q(ψ): The safety factor.
pub const Q_FACTOR: &str = "q";
/// g(ψ): The covariant toroidal B filed component (plasma current) **in Normalized Units**.
pub const CURRENT_G: &str = "g_norm";
/// I(ψ): The covariant poloidal B filed component (plasma current) **in Normalized Units**.
pub const CURRENT_I: &str = "I_norm";

// ================ 2D Variables ================

/// B(ψ, θ): The magnetic field strength in **in Normalized Units**.
pub const B_FIELD: &str = "b_field_norm";
/// dB(𝜓, 𝜃)/d𝜃: The first derivative of `B` with respect to boozer theta.
pub const DB_DTHETA: &str = "db_dtheta_norm";
/// dB(𝜓, 𝜃)/d𝜓: The first derivative of `B` with respect to psi.
pub const DB_DPSI: &str = "db_dpsi_norm";
/// d²B(𝜓, 𝜃)/d𝜓²: The second derivative of `B` with respect to psi.
pub const D2B_DPSI2: &str = "d2b_dpsi2_norm";

/// R(ψ, θ): The `R` coordinate with respect to boozer coordinates **in \[m\]**.
pub const R: &str = "R";
/// Z(ψ, θ): The `Z` coordinate with respect to boozer coordinates **in \[m\]**.
pub const Z: &str = "R";
