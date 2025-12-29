// Module for propagating TLEs with SGP4

// ------------------
// External Libraries
// ------------------

// ------------------
// Internal Libraries
// ------------------
use crate::tle::TLE;

// -------
// Structs
// -------

/// World Geodetic System (WGS) parameters
///
/// This struct contains the important Earth parameters defined by different WGS standards (ex: WGS-72, WGS-84)
///
/// References:
/// - [Revisiting Spacetrack Report #3: Rev 3](https://celestrak.org/publications/AIAA/2006-6753/AIAA-2006-6753-Rev3.pdf)
pub struct WGS {
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

/// Simplified General Perturbations 4 (SGP4) parameters
///
/// This struct contains the epoch parameters which are necessary to propagate the state vectors of a satellite with SGP4
///
/// References:
/// - [Revisiting Spacetrack Report #3: Rev 3](https://celestrak.org/publications/AIAA/2006-6753/AIAA-2006-6753-Rev3.pdf)
pub struct SGP4 {
    ///
    pub a0: f64,

    /// 
    pub a1: f64,

    ///
    pub a2: f64,

    ///
    pub delta1: f64,

    ///
    pub delta0: f64,

    ///
    pub n0: f64,

    ///
    pub rho: f64,

    ///
    pub theta: f64,

    ///
    pub zeta: f64,

    ///
    pub beta0: f64,

    ///
    pub eta: f64,

    ///
    pub c1: f64,

    ///
    pub c2: f64,

    ///
    pub c3: f64,

    ///
    pub c4: f64,

    ///
    pub c5: f64,

    ///
    pub d2: f64,

    ///
    pub d3: f64,

    ///
    pub d4: f64,

    ///
    pub m_dot: f64,

    ///
    pub omega_dot: f64,

    ///
    pub big_omega_dot: f64,

    ///

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
const WGS72: WGS = WGS {
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
const WGS84: WGS = WGS {
    mu: 398600.4418,
    r_earth_eq: 6378.137,
    j2: 0.00108262998905,
    j3: -0.00000253215306,
    j4: -0.00000161098761,
    xke: 0.07436685316871,
    tumin: 13.44685108204498
};

// ---------
// Functions
// ---------

///
pub fn init_sgp4(tle: TLE, wgs: Option<WGS>) -> SGP4 {
    // Use WGS84 or custom WGS constant if provided
    let mut wgs_const = WGS84;
    if let Some(wgs_passed) = wgs {
        wgs_const = wgs_passed;
    }

    // SGP4
    return SGP4;
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