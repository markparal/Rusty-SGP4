// Common constants and functions

// ------------------
// External Libraries
// ------------------

// ------------------
// Internal Libraries
// ------------------

// -------
// Structs
// -------

/// World Geodetic System (WGS) parameters
///
/// This struct contains the important Earth parameters defined by different WGS standards (ex: WGS-72, WGS-84)
///
/// References:
/// - [Revisiting Spacetrack Report #3: Rev 3 by Vallado et al](https://celestrak.org/publications/AIAA/2006-6753/AIAA-2006-6753-Rev3.pdf)
pub struct Wgs {
    /// Earth's standard gravitational parameter \[km^3/s^2\]
    pub mu: f64,

    /// Earth's equatorial radius \[km\]
    pub r_earth_eq: f64,

    /// Earth's J2 harmonic \[\]
    pub j2: f64,

    /// k2 constant \[Earth Radii^2\]
    pub k2: f64,

    /// Earth's J3 harmonic \[\]
    pub j3: f64,

    /// Earth's J4 harmonic \[\]
    pub j4: f64,

    /// k4 constant \[Earth Radii^4\]
    pub k4: f64,

    /// The square root of the standard gravitational parameter  \[Earth radii^1.5 / min\]
    pub ke: f64

    /// The inverse of ke \[min / Earth radii^1.5\]
    pub tumin: f64,
}

// ---------
// Enums
// ---------

// ---------
// Constants
// ---------

/// Fundamental and derived constants for WGS-72
///
/// mu: 398600.8 - Standard gravitational parameter, a product of the gravitational constant and the body's mass \[km^3 / s^2\]
///
/// r_earth_eq: 6378.135 - The Earth's equatorial radius \[km\]
///
/// j2: 0.001082616 - Second zonal harmonic (Earth's oblateness). Represents the equatorial bulge
///
/// k2: 0.000541308 - k2 constant 0.5 * j2 \[Earth Radii^2\]
///
/// j3: -0.00000253881 - Third zonal harmonic (Pear-shaped component)
///
/// j4: -0.00000165597 - Fourth zonal harmonic (Symmetric "squatness")
///
/// k4: 0.00000062098875 - k4 constant -3/8 * j4 \[Earth Radii^4\]
///
/// ke: 0.07436691613317 - The square root of the standard gravitational parameter \[Earth radii^1.5 / min\]
///
/// tumin: 13.44683969695931 - The inverse of ke \[min / Earth radii^1.5\]
///
/// References:
/// - [Revisiting Spacetrack Report #3: Rev 3 by Vallado et al](https://celestrak.org/publications/AIAA/2006-6753/AIAA-2006-6753-Rev3.pdf)
const WGS72: Wgs = Wgs {
    mu: 398600.8,
    r_earth_eq: 6378.135,
    j2: 0.001082616,
    k2: 0.000541308,
    j3: -0.00000253881,
    j4: -0.00000165597,
    k4: 0.00000062098875,
    ke: 0.07436691613317,
    tumin: 13.44683969695931
};

/// Fundamental and derived constants for WGS-84
///
/// mu: 398600.5 - Standard gravitational parameter, a product of the gravitational constant and the body's mass \[km^3 / s^2\]
///
/// r_earth_eq: 6378.137 - The Earth's equatorial radius \[km\]
///
/// j2: 0.00108262998905 - Second zonal harmonic (Earth's oblateness). Represents the equatorial bulge
///
/// k2: 0.000541314994525 - k2 constant 0.5 * j2 \[Earth Radii^2\]
///
/// j3: -0.00000253215306 - Third zonal harmonic (Pear-shaped component)
///
/// j4: -0.00000161098761 - Fourth zonal harmonic (Symmetric "squatness")
///
/// k4: 0.0000006041203538 - k4 constant -3/8 * j4 \[Earth Radii^4\]
///
/// ke: 0.07436685316871 - The square root of the standard gravitational parameter \[Earth radii^1.5 / min\]
///
/// tumin: 13.44685108204498 - The inverse of ke \[min / Earth radii^1.5\]
/// 
/// References:
/// - [Revisiting Spacetrack Report #3: Rev 3 by Vallado et al](https://celestrak.org/publications/AIAA/2006-6753/AIAA-2006-6753-Rev3.pdf)
const WGS84: Wgs = Wgs {
    mu: 398600.5,
    r_earth_eq: 6378.137,
    j2: 0.00108262998905,
    k2: 0.000541314994525,
    j3: -0.00000253215306,
    j4: -0.00000161098761,
    k4: 0.0000006041203538,
    ke: 0.07436685316871,
    tumin: 13.44685108204498
};

// ---------
// Functions
// ---------

/// Convert an angle from degrees to radians.
///
/// # Arguments
/// * `theta` - The angle in degrees
///
/// # Returns
/// * `theta_rad` - The angle in radians
///
/// # Examples
/// ```rust
/// // Define some angle in degrees
/// let theta = 90.0; // Degrees
///
/// // Convert the angle to radians
/// let theta_rad = deg2rad(theta);
/// 
/// // Assert the value is equal to the correct value
/// assert!((theta_rad - PI / 2.0).abs() < 1e-12);
/// ```
pub fn deg2rad(theta: f64) -> f64 {
    // Convert to radians
    let theta_rad = PI / 180.0 * theta;

    // Return theta in radians
    return theta_rad;
}

/// TODO
pub fn calc_period(a: f64, mu: f64) -> (f64) {
    // Calculate time period in minutes
    period = 2 * PI * (a.powi(3) / mu).sqrt() / 60.;

    return period;
}

// ----------
// Unit Tests
// ----------