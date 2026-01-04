// Module for propagating TLEs with SGP4

// ------------------
// External Libraries
// ------------------
use std::f64::consts::PI;

// ------------------
// Internal Libraries
// ------------------
use crate::tle::Tle;

// -------
// Structs
// -------

/// World Geodetic System (WGS) parameters
///
/// This struct contains the important Earth parameters defined by different WGS standards (ex: WGS-72, WGS-84)
///
/// References:
/// - [Revisiting Spacetrack Report #3: Rev 3](https://celestrak.org/publications/AIAA/2006-6753/AIAA-2006-6753-Rev3.pdf)
pub struct Wgs {
    /// Earth's standard gravitational parameter \[km^3/s^2\]
    pub mu: f64,

    /// Earth's equatorial radius \[km\]
    pub r_earth_eq: f64,

    /// Earth's J2 harmonic \[\]
    pub j2: f64,

    /// Earth's J3 harmonic \[\]
    pub j3: f64,

    /// Earth's J4 harmonic \[\]
    pub j4: f64,

    /// Minutes in one “time unit” \[min\]
    pub tumin: f64,

    /// The reciprocal of minutes in one “time unit” \[1/min\]
    pub xke: f64
}

/// Struct
pub struct ThirdBodyEffects {

}

/// Simplified General Perturbations 4 (SGP4) parameters
///
/// This struct contains the epoch parameters which are necessary to propagate the state vectors of a satellite with SGP4
///
/// References:
/// - [Revisiting Spacetrack Report #3: Rev 3](https://celestrak.org/publications/AIAA/2006-6753/AIAA-2006-6753-Rev3.pdf)
pub struct Sgp4 {
}

// -----
// Enums
// -----

// ---------
// Constants
// ---------

/// A conversion from rev/day to rad/min
///
/// References:
/// - [Revisiting Spacetrack Report #3: Rev 3](https://celestrak.org/publications/AIAA/2006-6753/AIAA-2006-6753-Rev3.pdf)
const XPDOTP: f64 = 229.1831180523293;

/// The rotational velocity of the earth in rad/min
///
/// References:
/// - [Revisiting Spacetrack Report #3: Rev 3](https://celestrak.org/publications/AIAA/2006-6753/AIAA-2006-6753-Rev3.pdf)
const RPTIM: f64 =  0.00437526908802;

/// Fundamental and derived constants for WGS-72
///
/// mu: 398600.8
///
/// r_earth_eq: 6378.135
///
/// j2: 0.001082616
///
/// j3: -0.00000253881
///
/// j4: -0.00000165597
///
/// xke: 0.07436691613317
///
/// tumin: 13.44683969695931
///
/// References:
/// - [Revisiting Spacetrack Report #3: Rev 3](https://celestrak.org/publications/AIAA/2006-6753/AIAA-2006-6753-Rev3.pdf)
const WGS72: Wgs = Wgs {
    mu: 398600.8,
    r_earth_eq: 6378.135,
    j2: 0.001082616,
    j3: -0.00000253881,
    j4: -0.00000165597,
    xke: 0.07436691613317,
    tumin: 13.44683969695931
};

/// Fundamental and derived constants for WGS-84
///
/// mu: 398600.4418
///
/// r_earth_eq: 6378.137
///
/// j2: 0.00108262998905
///
/// j3: -0.00000253215306
///
/// j4: -0.00000161098761
///
/// xke: 0.07436685316871
///
/// tumin: 13.44685108204498
/// 
/// References:
/// - [Revisiting Spacetrack Report #3: Rev 3](https://celestrak.org/publications/AIAA/2006-6753/AIAA-2006-6753-Rev3.pdf)
const WGS84: Wgs = Wgs {
    mu: 398600.4418,
    r_earth_eq: 6378.137,
    j2: 0.00108262998905,
    j3: -0.00000253215306,
    j4: -0.00000161098761,
    xke: 0.07436685316871,
    tumin: 13.44685108204498
};

/// Lunar physical and orbital constants
///
/// References:
/// - [History of Analytical Orbit Modeling in the U. S. Space Surveillance System](https://arc.aiaa.org/doi/abs/10.2514/1.9161?journalCode=jgcd)
/// - [Fundamentals of Astrodynamics Github Repository](https://github.com/CelesTrak/fundamentals-of-astrodynamics?tab=readme-ov-file)
pub mod SolarLunarConst {
    /// Obliquity of the ecliptic plane \[degrees\]
    pub const EPS: f64 = 23.4441;

    /// Moon's inclination with respect to the ecliptic plane \[degrees\]
    pub const I_ME: f64 = 5.145396374;

    /// Solar inclination \[degrees\]
    pub const I_S: f64 = 23.4441;

    /// Lunar eccentricity
    pub const E_M: f64 = 0.05490;

    /// Solar eccentricity
    pub const E_S: f64 = 0.01675;

    /// Lunar mean motion \[rad/min\]
    pub const N_M: f64 = 1.583521770e-4;

    /// Solar mean motion \[rad/min\]
    pub const N_S: f64 = 1.19459e-5;

    /// Solar right ascension of the ascending node (RAAN) \[degrees\]
    pub const RAAN_S: f64 = 0.0;

    /// Solar argument of periapsis \[degrees\]
    pub const OMEGA_S: f64 = 281.2208;

    /// Lunar perturbation coefficient \[rad/min\]
    pub const C_M: f64 = 4.796806521e-7;

    /// Solar perturbation coefficient \[rad/min\]
    pub const C_S: f64 = 2.98647972e-6;

    /// Lunar/Solar element epochs (12/31/1899 12:00:00 UTC) \[Julian date\]
    pub const EPOCH: f64 = 2415020.0;

    /// Lunar right ascension of the ascending node (RAAN) with respect to the ecliptic plane at epoch \[rad\]
    pub const RAAN_ME0: f64 = 4.5236020;

    /// Lunar right ascension of the ascending node (RAAN) with respect to the ecliptic plane time rate of change \[rad/day\]
    pub const RAAN_ME0_DOT: f64 = -9.2422029e-4;

    /// Lunar longitude of perigee with respect to the ecliptic plane at epoch \[rad\]
    pub const U_ME0: f64 = 5.8351514;

    /// Lunar longitude of perigee with respect to the ecliptic plane at epoch \[rad/day\]
    pub const U_ME0_DOT: f64 = 0.0019443680;
}

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

/// Convert a datetime in UTC to Julian date format
///
/// # Arguments
/// * `` - 
///
/// # Returns
/// * `` - 
///
/// # Examples
/// ```rust
/// ```
///
/// References:
/// - [Fundamentals of Astrodynamics and Applications](https://celestrak.org/software/vallado-sw.php)
pub fn utc2jday() -> f64 {
    // void    jday_SGP4
	// 	(
	// 	int year, int mon, int day, int hr, int minute, double sec,
	// 	double& jd, double& jdFrac
	// 	)
	// {
	// 	jd = 367.0 * year -
	// 		floor((7 * (year + floor((mon + 9) / 12.0))) * 0.25) +
	// 		floor(275 * mon / 9.0) +
	// 		day + 1721013.5;  // use - 678987.0 to go to mjd directly
	// 	jdFrac = (sec + minute * 60.0 + hr * 3600.0) / 86400.0;

	// 	// check that the day and fractional day are correct
	// 	if (fabs(jdFrac) > 1.0)
	// 	{
	// 		double dtt = floor(jdFrac);
	// 		jd = jd + dtt;
	// 		jdFrac = jdFrac - dtt;
	// 	}
}

/// Build an [`SGP4`] struct for state propagation from a [`Tle`] struct
///
/// Given a [`Tle`] struct, calculate the time-independent parameters necessary 
/// to propagate a satellite's states in time. 
///
/// # Arguments
/// * `tle` - The Two-Line Element parameters
/// * `wgs` - Optional, specify World Geodetic System (WGS) parameters (defaults to WGS-84)
///
/// # Returns
/// * [`SGP4`] - The time-independent parameters for the SGP4 propagator
///
/// # Examples
/// ```rust
/// ```
pub fn init_sgp4(tle: Tle, wgs: Option<Wgs>) -> Sgp4 {
    // Use WGS84 or custom WGS constant if provided
    let mut wgs_sgp4 = WGS84;
    if let Some(wgs_passed) = wgs {
        wgs_sgp4 = wgs_passed;
    }

    // Extract TLE contents in proper units
    let i0 = deg2rad(tle.inclination);
    let no = tle.mean_motion;
    let e0 = tle.eccentricity;
    let omega0 = deg2rad(tle.argument_of_perigee);
    let raan0 = deg2rad(tle.right_ascension_of_ascending_node);
    let m0 = deg2rad(tle.mean_anomaly);
    let bstar = tle.bstar;

    // Recover the Brouer mean motion from the Kozai mean motion (mean motion in TLE)
    let ke = wgs_sgp4.mu.sqrt() / wgs_sgp4.r_earth_eq.powf(1.5) * 60.0; // [(Earth radii)^1.5 / min]
    let k2 = 0.5 * wgs_sgp4.j2; // [(Earth radii)^2]
    let a1 = (ke / n0).powf(2/3);
    let delta1 = (3/2) * (k2 / a1.powf(2)) * (3 * i0.cos().powf(2) - 1) / (1 - e0.powf(2)).powf(3/2);
    let a2 = a1 * (1 - (1/3) * delta1 - delta1.powf(2) - (134/81) * delta1.powf(3));
    let delta0 = (3/2) * (k2 / a2.powf(2)) * (3 * i0.cos().powf(2) - 1) / (1 - e0.powf(2)).powf(3/2);
    let n0_brouwer = n0 / (1 + delta0);
    let a0 = (ke / n0_brouwer).powf(2/3);

    // Atmospheric drag
    let a30 = -wgs_sgp4.j3 * wgs_sgp4.r_earth_eq.powf(3);
    let rp = a0 * (1.0 - e0);
    let hp = rp - wgs_sgp4.r_earth_eq;
    let q0 = (120.0 + wgs_sgp4.r_earth_eq) / wgs_sgp4.r_earth_eq; // [Earth radii]
    let mut s = (78.0 + wgs_sgp4.r_earth_eq) / wgs_sgp4.r_earth_eq; // [Earth radii]
    if hp >= 98.0 && hp <= 156.0 {
        s = (rp - 78.0) / wgs_sgp4.r_earth_eq; // [Earth radii]
    } else if hp < 98.0 {
        s = (20.0 + wgs_sgp4.r_earth_eq) / wgs_sgp4.r_earth_eq; // [Earth radii]
    }
    let theta = i0.cos();
    let zeta = 1 / (a0 - s);
    let beta0 = (1 - e0.powf(2)).powf(0.5);
    let eta = a0 * e0 * zeta;
    let c2 = (q0 - s).powf(4) * zeta.powf(4) * n0_brouwer * (1 - eta.powf(2)).powf(-7/2) * (a0 * (1 + (3/2) * eta.powf(2) + 4 * e0 * eta + e0 * eta.powf(3)) + (3/2) * (k2 * zeta / (1 - eta.powf(2))) * (-(1/2) + (3/2) * theta.powf(2)) * (8 + 24 * eta.powf(2) + 3 * eta.powf(4)));
    let c1 = bstar * c2;
    let c3 = ((q0 - s).powf(4) * zeta.powf(5) * a30 * n0_brouwer * wgs_sgp4.r_earth_eq * i0.sin()) / (k2 * e0);
    let c4 = 2 * n0_brouwer * (q0 - s).powf(4) * zeta.powf(4) * a0 * beta0.powf(2) * (1 - eta.powf(2)).powf(-7/2) * ((2 * eta * (1 + e0*eta) + 0.5 * e0 + 0.5 * eta.powf(3)) - (2 * k2 * zeta / (a0 * (1 - eta.powf(2)))) * (3 * (1 - 3 * theta.powf(2)) * (1 + 3/2 * eta.powf(2) - 2 * e0 * eta - 0.5 * e0 * eta.powf(3))) + 3/4 * (1 - theta.powf(2)) * (2 * eta.powf(2) - e0 * eta - e0 * eta.powf(3)) * (2 * omega0).cos());
    let c5 = 2 * (q0 - s).powf(4) * zeta.powf(4) * a0 * beta0.powf(2) * (1 - eta.powf(2)).powf(-7/2) * (1 + 11/4 * eta * (eta + e0) + e0 * eta.powf(3));
    let d2 = 4 * a0 * zeta * c1.powf(2);
    let d3 = 4/3 * a0 * zeta.powf(2) * (17 * a0 + s) * c1.powf(3);
    let d4 = 2/3 * a0.powf(2) * zeta.powf(3) * (221 * a0 + 31 * s) * c1.powf(4);

    // Earth zonal harmonics
    let k4 = -3/8 * wgs_sgp4.j4 * wgs_sgp4.r_earth_eq.powf(4);
    let mdot = ((3 * k2 * (-1 + 3 * theta.powf(2)) / (2 * a0.powf(2) * beta0.powf(3))) + (3 * k2.powf(2) * (13 - 78 * theta.powf(2) + 137 * theta.powf(4)) / (16 * a0.powf(4) * beta0.powf(7)))) * n0_brouwer;
    let omegadot = (-3 * k2 * (1 - 5 * theta.powf(2)) / (2 * a0.powf(2) * beta0.powf(4)) + 3 * k2.powf(2) * (7 - 114 * theta.powf(2) + 395 * theta.powf(4)) / (16 * a0.powf(4) * beta0.powf(8)) + 5 * k4 * (3 - 36 * theta.powf(2) + 49 * theta.powf(4)) / (4 * a0.powf(4) * beta0.powf(8))) * n0_brouwer;
    let raandot = (-3 * k2 * theta / (a0.powf(2) * beta0.powf(4)) + 3 * k2.powf(2) * (4 * theta - 19 * theta.powf(3)) / (2 * a0.powf(4) * beta0.powf(8)) + 5 * k4 * theta * (3 - 7 * theta.powf(2)) / (2 * a0.powf(4) * beta0.powf(8))) * n0_brouwer;

    // Lunar and solar gravity effects

    // Earth gravity resonance effects

    // SGP4
    sgp4 = Sgp4;
    return sgp4;
}

/// Build an [`SGP4`] struct for state propagation from a [`Tle`] struct
///
/// Given a [`Tle`] struct, calculate the time-independent parameters necessary 
/// to propagate a satellite's states in time. 
///
/// # Arguments
/// * `tle` - The Two-Line Element parameters
/// * `wgs` - Optional, specify World Geodetic System (WGS) parameters (defaults to WGS-84)
///
/// # Returns
/// * [`SGP4`] - The time-independent parameters for the SGP4 propagator
///
/// # Examples
/// ```rust
/// ```
pub fn init_lunar_solar_effects(tle: Tle) -> i16 {
    // Get the epoch of the TLE

    // Convert the TLE epoch to Julian date

    // Find the difference in time between the Solar / Lunar epoch and the TLE epoch

    // Calculate the Lunar RAAN wrt to the ecliptic plane at TLE epoch

    // Calculate the Lunar inclination at TLE epoch

    // Calculate the Lunar longitude of perigee referred to the ecliptic

    // 
    return 0;
}

// ----------
// Unit Tests
// ----------
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_() {
    }
}