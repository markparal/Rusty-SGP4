// Module for propagating TLEs with SGP4

// ------------------
// External Libraries
// ------------------
use std::f64::consts::PI;

// ------------------
// Internal Libraries
// ------------------
use crate::tle::Tle;
use crate::time;

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
    pub ke: f64
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
    /// TLE
    pub tle: Tle,

    ///
    pub jd0: f64,

    pub jdfrac0: f64,

    pub i0: f64,

    pub e0: f64,

    pub omega0: f64,

    pub raan0: f64,

    pub m0: f64,

    pub eta0: f64,

    pub bstar: f64,

    pub n0: f64,

    pub a0: f64,

    pub period0: f64,
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
/// mu: 398600.8 - Standard gravitational parameter, a product of the gravitational constant and the body's mass \[km^3 / s^2\]
///
/// r_earth_eq: 6378.135 - The Earth's equatorial radius \[km\]
///
/// j2: 0.001082616 - Second zonal harmonic (Earth's oblateness). Represents the equatorial bulge
///
/// j3: -0.00000253881 - Third zonal harmonic (Pear-shaped component)
///
/// j4: -0.00000165597 - Fourth zonal harmonic (Symmetric "squatness")
///
/// ke: 0.07436691613317 - The square root of the standard gravitational parameter \[Earth radii^1.5 / min\]
///
/// tumin: 13.44683969695931 - The inverse of ke \[min / Earth radii^1.5\]
///
/// References:
/// - [Revisiting Spacetrack Report #3: Rev 3](https://celestrak.org/publications/AIAA/2006-6753/AIAA-2006-6753-Rev3.pdf)
const WGS72: Wgs = Wgs {
    mu: 398600.8,
    r_earth_eq: 6378.135,
    j2: 0.001082616,
    j3: -0.00000253881,
    j4: -0.00000165597,
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
/// j3: -0.00000253215306 - Third zonal harmonic (Pear-shaped component)
///
/// j4: -0.00000161098761 - Fourth zonal harmonic (Symmetric "squatness")
///
/// ke: 0.07436685316871 - The square root of the standard gravitational parameter \[Earth radii^1.5 / min\]
///
/// tumin: 13.44685108204498 - The inverse of ke \[min / Earth radii^1.5\]
/// 
/// References:
/// - [Revisiting Spacetrack Report #3: Rev 3](https://celestrak.org/publications/AIAA/2006-6753/AIAA-2006-6753-Rev3.pdf)
const WGS84: Wgs = Wgs {
    mu: 398600.5,
    r_earth_eq: 6378.137,
    j2: 0.00108262998905,
    j3: -0.00000253215306,
    j4: -0.00000161098761,
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

pub fn calc_period(a: f64, mu: f64) -> (f64) {
    // Calculate time period in minutes
    period = 2 * PI * (a.powi(3) / mu).sqrt() / 60.;

    return period;
}

/// Build an [`SGP4`] struct for state propagation from a [`Tle`] struct
///
/// Given a [`Tle`] struct, calculate the time-independent parameters necessary 
/// to propagate a satellite's states in time. 
///
/// # Arguments
/// * `tle` - The Two-Line Element parameters
/// * `wgs` - Optional, specify World Geodetic System (WGS) parameters (defaults to WGS-72)
///
/// # Returns
/// * [`SGP4`] - The time-independent parameters for the SGP4 propagator
///
/// # Examples
/// ```rust
/// ```
pub fn init_sgp4(tle: Tle, wgs: Option<Wgs>) -> Sgp4 {
    // Use WGS72 or custom WGS constant if provided
    let mut wgs_sgp4 = WGS72;
    if let Some(wgs_passed) = wgs {
        wgs_sgp4 = wgs_passed;
    }

    // Extract TLE contents in proper units
    let i0 = deg2rad(tle.inclination);
    let n0_kozai = tle.mean_motion;
    let e0 = tle.eccentricity;
    let omega0 = deg2rad(tle.argument_of_perigee);
    let raan0 = deg2rad(tle.right_ascension_of_ascending_node);
    let m0 = deg2rad(tle.mean_anomaly);
    let eta0 = (1. - e0.powi(2)).sqrt();
    let bstar = tle.bstar;

    // Extract TLE epoch in Julian day format
    let datetime0 = dayofyr2utc(tle.epoch_year, tle.epoch_day).unwrap();
    let (jd0, jdfrac0) = utc2jday(datetime0).unwrap();

    // Recover the Brouer mean motion from the Kozai mean motion (mean motion in TLE)
    let ke = wgs_sgp4.mu.sqrt() / wgs_sgp4.r_earth_eq.powf(1.5) * 60.; // [(Earth radii)^1.5 / min]
    let k2 = 0.5 * wgs_sgp4.j2; // [(Earth radii)^2]
    let a1 = (ke / n0_kozai).powf(2./3.);
    let delta1 = (3./2.) * (k2 / a1.powf(2.)) * (3. * i0.cos().powf(2.) - 1.) / (1. - e0.powf(2.)).powf(3./2.);
    let a2 = a1 * (1. - (1./3.) * delta1 - delta1.powf(2.) - (134./81.) * delta1.powf(3.));
    let delta0 = (3./2.) * (k2 / a2.powf(2.)) * (3. * i0.cos().powf(2.) - 1.) / (1. - e0.powf(2.)).powf(3./2.);
    let n0 = n0_kozai / (1. + delta0);
    let a0 = (ke / n0).powf(2./3.);
    let period0 = calc_period(a0, wgs_sgp4.mu); // [min]

    // Atmospheric drag
    let a30 = -wgs_sgp4.j3 * wgs_sgp4.r_earth_eq.powf(3.);
    let rp = a0 * (1. - e0);
    let hp = rp - wgs_sgp4.r_earth_eq;
    let q0 = (120. + wgs_sgp4.r_earth_eq) / wgs_sgp4.r_earth_eq; // [Earth radii]
    let mut s = (78. + wgs_sgp4.r_earth_eq) / wgs_sgp4.r_earth_eq; // [Earth radii]
    if hp >= 98. && hp <= 156. {
        s = (rp - 78.) / wgs_sgp4.r_earth_eq; // [Earth radii]
    } else if hp < 98. {
        s = (20. + wgs_sgp4.r_earth_eq) / wgs_sgp4.r_earth_eq; // [Earth radii]
    }
    let theta = i0.cos();
    let zeta = 1. / (a0 - s);
    let beta0 = (1. - e0.powf(2.)).powf(0.5);
    let eta = a0 * e0 * zeta;
    let c2 = (q0 - s).powf(4.) * zeta.powf(4.) * n0 * (1. - eta.powf(2.)).powf(-7./2.) * (a0 * (1. + (3./2.) * eta.powf(2.) + 4. * e0 * eta + e0 * eta.powf(3.)) + (3./2.) * (k2 * zeta / (1. - eta.powf(2.))) * (-(1./2.) + (3./2.) * theta.powf(2.)) * (8. + 24. * eta.powf(2.) + 3. * eta.powf(4.)));
    let c1 = bstar * c2;
    let c3 = ((q0 - s).powf(4.) * zeta.powf(5.) * a30 * n0 * wgs_sgp4.r_earth_eq * i0.sin()) / (k2 * e0);
    let c4 = 2. * n0 * (q0 - s).powf(4.) * zeta.powf(4.) * a0 * beta0.powf(2.) * (1. - eta.powf(2.)).powf(-7./2.) * ((2. * eta * (1. + e0*eta) + 0.5 * e0 + 0.5 * eta.powf(3.)) - (2. * k2 * zeta / (a0 * (1. - eta.powf(2.)))) * (3. * (1. - 3. * theta.powf(2.)) * (1. + 3./2. * eta.powf(2.) - 2. * e0 * eta - 0.5 * e0 * eta.powf(3.))) + 3./4. * (1. - theta.powf(2.)) * (2. * eta.powf(2.) - e0 * eta - e0 * eta.powf(3.)) * (2. * omega0).cos());
    let c5 = 2. * (q0 - s).powf(4.) * zeta.powf(4.) * a0 * beta0.powf(2.) * (1. - eta.powf(2.)).powf(-7./2.) * (1. + 11./4. * eta * (eta + e0) + e0 * eta.powf(3.));
    let d2 = 4. * a0 * zeta * c1.powf(2.);
    let d3 = 4./3. * a0 * zeta.powf(2.) * (17. * a0 + s) * c1.powf(3.);
    let d4 = 2./3. * a0.powf(2.) * zeta.powf(3.) * (221. * a0 + 31. * s) * c1.powf(4.);

    // Earth zonal harmonics
    let k4 = -3./8. * wgs_sgp4.j4 * wgs_sgp4.r_earth_eq.powf(4.);
    let m_dot = ((3. * k2 * (-1. + 3. * theta.powf(2.)) / (2. * a0.powf(2.) * beta0.powf(3.))) + (3. * k2.powf(2.) * (13. - 78. * theta.powf(2.) + 137. * theta.powf(4.)) / (16. * a0.powf(4.) * beta0.powf(7.)))) * n0;
    let omega_dot = (-3. * k2 * (1. - 5. * theta.powf(2.)) / (2. * a0.powf(2.) * beta0.powf(4.)) + 3. * k2.powf(2.) * (7. - 114. * theta.powf(2.) + 395. * theta.powf(4.)) / (16. * a0.powf(4.) * beta0.powf(8.)) + 5. * k4 * (3. - 36. * theta.powf(2.) + 49. * theta.powf(4.)) / (4. * a0.powf(4.) * beta0.powf(8.))) * n0;
    let raan_dot = (-3. * k2 * theta / (a0.powf(2.) * beta0.powf(4.)) + 3. * k2.powf(2.) * (4. * theta - 19. * theta.powf(3.)) / (2. * a0.powf(4.) * beta0.powf(8.)) + 5. * k4 * theta * (3. - 7. * theta.powf(2.)) / (2. * a0.powf(4.) * beta0.powf(8.))) * n0;

    // Lunar and solar gravity effects
    let (e_ls_dot, i_ls_dot, m_ls_dot, raan_ls_dot, omega_ls_dot) = init_lunar_solar_effects(jd0, jdfrac0, i0, n0, e0, omega0, raan0, eta0).unwrap();

    // Earth gravity resonance effects
    let mut resonance
    let mut res1 = 0.;
    let mut res2 = 0.;
    let mut res3 = 0.;
    let mut res4 = 0.;
    let mut res5 = 0.;
    let mut res6 = 0.;
    let mut res7 = 0.;
    let mut res8 = 0.;
    let mut res9 = 0.;
    let mut res10 = 0.;
    if period0 >= 1200. && period0 <= 1800. {
        (res1, res2, res3, res4, res5, res6) = init_earth_gravity_resonance_wholeday(i0, n0, e0, a0).unwrap();
    } else if period0 >= 680. && period0 <= 760. {
        (res1, res2, res3, res4, res5, res6, res7, res8, res9, res10) = init_earth_gravity_resonance_halfday(i0, n0, e0, a0).unwrap();
    }

    // SGP4
    let sgp4 = Sgp4{};
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
pub fn init_lunar_solar_effects(jd0: f64, jdfrac0: f64, i0: f64, n0: f64, e0: f64, omega0: f64, raan0: f64, eta0: f64) -> (f64, f64, f64, f64, f64) {
    // Obliquity of the ecliptic plane \[rad\]
    let eps = deg2rad(23.4441);
    let sin_eps = eps.sin();
    let cos_eps = eps.cos();

    // Lunar/Solar element epochs (12/31/1899 12:00:00 UTC) \[Julian date\]
    let epoch_sm = 2415020.0;
    
    // Lunar constants
    // Moon's inclination with respect to the ecliptic plane \[rad\]
    let i_me = deg2rad(5.145396374);
    let sin_i_me = i_me.sin();
    let cos_i_me = i_me.cos();

    // Lunar eccentricity
    let e_m = 0.05490;

    // Lunar mean motion \[rad/min\]
    let n_m = 1.583521770e-4;

    // Lunar perturbation coefficient \[rad/min\]
    let c_m = 4.796806521e-7;

    // Lunar right ascension of the ascending node (RAAN) with respect to the ecliptic plane at epoch \[rad\]
    let raan_me0 = 4.5236020;

    // Lunar right ascension of the ascending node (RAAN) with respect to the ecliptic plane time rate of change at epoch \[rad/day\]
    let raan_me0_dot = -9.2422029e-4;

    // Lunar longitude of perigee with respect to the ecliptic plane at epoch \[rad\]
    let u_me0 = 5.8351514;

    // Lunar longitude of perigee with respect to the ecliptic plane time rate of change at epoch \[rad/day\]
    let u_me0_dot = 0.0019443680;

    // Solar constants
    // Solar inclination \[rad\]
    let i_s = deg2rad(23.4441);
    let sin_i_s = i_s.sin();
    let cos_i_s = i_s.cos();

    // Solar eccentricity
    let e_s = 0.01675;

    // Solar mean motion \[rad/min\]
    let n_s = 1.19459e-5;

    // Solar right ascension of the ascending node (RAAN) \[rad\]
    let raan_s = 0.0;

    // Solar argument of periapsis \[rad\]
    let omega_s = deg2rad(281.2208);

    // Solar perturbation coefficient \[rad/min\]
    let c_s = 2.98647972e-6;

    // Solar mean anomaly at epoch \[rad\]
    let m_s0 = 6.2565837;

    // Solar mean anomaly time rate of change at epoch \[rad/day\]
    let m_s0_dot = 0.017201977;

    // Find the difference in time between the Solar / Lunar epoch and the TLE epoch
    let delta_t = tle_jd + tle_jdfrac - epoch_sm;

    // Calculate the Lunar RAAN wrt to the ecliptic plane at TLE epoch
    let raan_me = (raan_me0 + raan_me0_dot * delta_t).rem_euclid(2.0 * PI);
    let sin_raan_me = raan_me.sin();
    let cos_raan_me = raan_me.cos();

    // Calculate the Lunar inclination at TLE epoch (this formula is fine because lunar inclination never is negative)
    let i_m = (cos_eps * cos_i_me - sin_eps * sin_i_me * cos_raan_me).clamp(-1.0, 1.0).acos();
    let sin_i_m = i_m.sin();
    let cos_i_m = i_m.cos();

    // Calculate the Lunar longitude of perigee referred to the ecliptic
    let gamma_m = u_me0 + u_me0_dot * delta_t;

    // Calculate the Lunar RAAN \[rad\]
    let sin_raan_m = (sin_i_me * sin_raan_me) / sin_i_m;
    let cos_raan_m = (cos_i_me * sin_eps + cos_eps * sin_i_me * cos_raan_me) / sin_i_m;
    let raan_m = sin_raan_m.atan2(cos_raan_m);

    // Calculate the Lunar phase shift \[rad\]
    let sin_delta = (sin_eps * sin_raan_me) / sin_i_m;
    let cos_delta = cos_raan_m * cos_raan_me + sin_raan_m * sin_raan_me * cos_eps;
    let delta = sin_delta.atan2(cos_delta);

    // Calculate the Lunar argument of periapsis \[rad\]
    let omega_m = gamma_m - raan_me + delta;

    // Calculate the Solar mean anomaly \[rad\]
    let m_s = (m_s0 + m_s0_dot * delta_t).rem_euclid(2.0 * PI);

    // Calculate the Lunar secular rates
    let (a_m_dot, e_m_dot, i_m_dot, m_m_dot, raan_m_dot, omega_m_dot) = calc_lunar_solar_secular_rates(i_m, n_m, omega_m, raan_m, c_m, i0, n0, e0, omega0, raan0, eta0).unwrap();

    // Calculate the Solar secular rates
    let (a_s_dot, e_s_dot, i_s_dot, m_s_dot, raan_s_dot, omega_s_dot) = calc_lunar_solar_secular_rates(i_s, n_s, omega_s, raan_s, c_s, i0, n0, e0, omega0, raan0, eta0).unwrap();

    // Calculate the combined 3rd body secular rates
    let e_ls_dot = e_m_dot + e_s_dot;
    let i_ls_dot = i_m_dot + i_s_dot;
    let m_ls_dot = m_m_dot + m_s_dot;
    let raan_ls_dot = raan_m_dot + raan_s_dot;
    let omega_ls_dot = omega_m_dot + omega_s_dot;

    return Ok((e_ls_dot, i_ls_dot, m_ls_dot, raan_ls_dot, omega_ls_dot));
}

pub fn calc_lunar_solar_secular_rates(i_x: f64, n_x: f64, omega_x: f64, raan_x: f64, c_x: f64, i0: f64, n0: f64, e0: f64, omega0: f64, raan0: f64, eta0: f64) -> (f64, f64, f64, f64, f64, f64) {
    // Precompute common quantities
    let cos_raan_diff = (raan0 - raan_x).cos();
    let sin_raan_diff = (raan0 - raan_x).sin();
    let cos_omega_x = omega_x.cos();
    let sin_omega_x = omega_x.sin();
    let cos_omega0 = omega0.cos();
    let sin_omega0 = omega0.sin();
    let cos_i_x = i_x.cos();
    let sin_i_x = i_x.sin();
    let cos_i0 = i0.cos();
    let sin_i0 = i0.sin();

    // Calculate 3rd body constants
    let a1 = cos_omega_x * cos_raan_diff + sin_omega_x * cos_i_x * sin_raan_diff;
    let a3 = -sin_omega_x * cos_raan_diff + cos_omega_x * cos_i_x * sin_raan_diff;
    let a7 = -cos_omega_x * sin_raan_diff + sin_omega_x * cos_i_x * cos_raan_diff;
    let a8 = sin_omega_x * sin_i_x;
    let a9 = sin_omega_x * sin_raan_diff + cos_omega_x * cos_i_x * cos_raan_diff;
    let a10 = cos_omega_x * sin_i_x;
    let a2 = a7 * cos_i0 + a8 * sin_i0;
    let a4 = a9 * cos_i0 + a10 * sin_i0;
    let a5 = -a7 * sin_i0 + a8 * cos_i0;
    let a6 = -a9 * sin_i0 + a10 * cos_i0;

    let x1 = a1 * cos_omega0 + a2 * sin_omega0;
    let x2 = a3 * cos_omega0 + a4 * sin_omega0;
    let x3 = -a1 * sin_omega0 + a2 * cos_omega0;
    let x4 = -a3 * sin_omega0 + a4 * cos_omega0;
    let x5 = a5 * sin_omega0;
    let x6 = a6 * sin_omega0;
    let x7 = a5 * cos_omega0;
    let x8 = a6 * cos_omega0;
    
    let z31 = 12. * x1.powi(2) - 3. * x3.powi(2);
    let z32 = 24. * x1 * x2 - 6. * x3 * x4;
    let z33 = 12. * x2.powi(2) - 3. * x4.powi(2);
    let z1 = 6. * (a1.powi(2) + a2.powi(2)) + (1. + e0.powi(2)) * z31;
    let z2 = 12. * (a1 * a3 + a2 * a4) + (1 + e0.powi(2)) * z32;
    let z3 = 6. * (a3.powi(2) + a4.powi(2)) + (1 + e0.powi(2)) * z33;
    let z11 = -6. * a1 * a5 + e0.powi(2) * (-24. * x1 * x7 - 6. * x3 * x5);
    let z13 = -6. * a3 * a6 + e0.powi(2) * (-24. * x2 * x8 - 6. * x4 * x6);
    let z21 = 6. * a2 * a5 + e0.powi(2) * (24. * x1 * x5 - 6. * x3 * x7);
    let z23 = 6. * a4 * a6 + e0.powi(2) * (24. * x2 * x6 - 6. * x4 * x8);
    let z22 = 6. * a4 * a5 + 6. * a2 * a6 + e0.powi(2) * (24. * x2 * x5 + 24. * x1 * x6 - 6. * x4 * x7 - 6. * x3 * x8);
    let z12 = -6. * a1 * a6 - 6. * a3 * a5 - e0.powi(2) * (24. * x2 * x7 + 24. * x1 * x8 + 6. * x3 * x6 + 6. * x4 * x5);

    // Calculate secular rates
    let a_x_dot = 0.;
    
    let e_x_dot = -15. * c_x * n_x * (e0 * eta0 / n0) * (x1 * x3 + x2 * x4);
    
    let i_x_dot = (-c_x * n_x / (2. * n0 * eta0)) * (z11 + z13);
    
    let m_x_dot = (-c_x * n_x / n0) * (z1 + z3 - 14. - 6. * e0.powi(2));
    
    let mut raan_x_dot = 0.;
    if i0 >= deg2rad(3.) {
        raan_x_dot = c_x * n_x / (2. * n0 * eta0 * sin_i0) * (z21 + z23);
    }

    let mut omega_x_dot = c_x * n_x * eta0 / n0 * (z31 + z33 - 6.);
    if i0 >= deg2rad(3.) {
        omega_x_dot = omega_x_dot - raan_x_dot * cos_i0;
    }

    // Return secular rates
    return Ok((a_x_dot, e_x_dot, i_x_dot, m_x_dot, raan_x_dot, omega_x_dot))
}

pub fn init_earth_gravity_resonance_halfday(i0: f64, n0: f64, e0: f64, a0: f64) -> {
    // Precompute common quantities
    let cos_i0 = i0.cos();
    let sin_i0 = i0.sin();

    // Define constants
    let c22s22 = 1.7891679e-6;
    let c32s32 = 3.7393792e-7;
    let c44s44 = 7.3636953e-9;
    let c52s52 = 1.1428639e-7;
    let c54s54 = 2.1765803e-9;

    // Calculate functions of inclination
    let f220 = (3./4.) * (1. + cos_i0).powi(2);
    let f221 = (3./2.) * sin_i0.powi(2);
    let f321 = (15./8.) * sin_i0 * (1. - 2. * cos_i0 - 3. * cos_i0.powi(2));
    let f322 = (-15./8.) * sin_i0 * (1. + 2. * cos_i0 - 3. * cos_i0.powi(2));
    let f441 = (105./4.) * sin_i0.powi(2) * (1. + cos_i0).powi(2);
    let f442 = (315./8.) * sin_i0.powi(4);
    let f522 = (315./32.) * (sin_i0.powi(3) - 2. * sin_i0.powi(3) * cos_i0 - 5. * sin_i0.powi(3) * cos_i0.powi(2) + sin_i0 * ((-2./3.) + (4./3.) * cos_i0 + 2. * cos_i0.powi(2)));
    let f523 = (105./16.) * sin_i0 * (1. + 2. * cos_i0 - 3. * cos_i0.powi(2) - (3./2.) * sin_i0.powi(2) * (1. + 2. * cos_i0 - 5. * cos_i0.powi(2)));
    let f542 = (945./32.) * sin_i0 * (2. - 8. * cos_i0 + cos_i0.powi(2) * (-12. + 8. * cos_i0 + 10. * cos_i0.powi(2)));
    let f543 = (945./32.) * sin_i0 * (cos_i0.powi(2) * (12. + 8. * cos_i0 - 10. cos_i0.powi(2)) - 2. - 8. * cos_i0);

    // Calculate functions of eccentricity
    let mut g211 = 0.;
    let g201 = -0.306 - 0.44 * (e0 - 0.64);
    let mut g310 = 0.;
    let mut g322 = 0.;
    let mut g410 = 0.;
    let mut g422 = 0.;
    let mut g520 = 0.;
    let mut g521 = 0.;
    let mut g532 = 0.;
    let mut g533 = 0.;
    if e0 <= 0.65 {
        g211 = 3.616 - 13.247 * e0 + 16.29 * e0.powi(2);
        g310 = -19.302 + 117.39 * e0 - 228.419 * e0.powi(2) + 156.591 * e0.powi(3);
        g322 = -18.9068 + 109.7927 * e0 - 214.6334 * e0.powi(2) + 146.5816 * e0.powi(2);
        g410 = -41.122 + 242.694 * e0 - 471.094 * e0.powi(2) + 313.953 * e0.powi(3);
        g422 = -146.407 + 841.88 * e0 - 1629.014 * e0.powi(2) + 1083.435 * e0.powi(3);
        g520 = -532.114 + 3017.977 * e0 - 5740.032 * e0.powi(2) + 3708.276 * e0.powi(3);
    } else {
        g211 = -72.099 + 331.819 * e0 - 508.738 * e0.powi(2) + 266.724 * e0.powi(3);
        g310 = -346.844 + 1582.851 * e0 - 2415.925 * e0.powi(2) + 1246.113 * e0.powi(3);
        g322 = -342.585 + 1554.908 * e0 - 2366.899 * e0.powi(2) + 1215.972 * e0.powi(3);
        g410 = -1052.797 + 4758.686 * e0 - 7193.992 * e0.powi(2) + 3651.957 * e0.powi(3);
        g422 = -3581.69 + 16178.11 * e0 - 24462.77 * e0.powi(2) + 12422.52 * e0.powi(3);
        if e0 < 0.715 {
            g520 = 1464.74 - 4664.75 * e0 + 3763.64 * e0.powi(2);
        } else {
            g520 = -5149.66 + 29936.92 * e0 - 54087.36 * e0.powi(2) + 31324.56 * e0.powi(3);
        }
    }
    if e0 < 0.7 {
        g521 = -822.71072 + 4568.6173 * e0 - 8491.4146 * e0.powi(2) + 5337.524 * e0.powi(3);
        g532 = -853.666 + 4690.25 * e0 - 8624.77 * e0.powi(2) + 5341.4 * e0.powi(3);
        g533 = -919.2277 + 4988.61 * e0 - 9064.77 * e0.powi(2) + 5542.21 * e0.powi(3);
    } else {
        g521 = -51752.104 + 218913.95 * e0 - 309468.16 * e0.powi(2) + 146349.42 * e0.powi(3);
        g532 = -40023.88 + 170470.89 * e0 - 242699.48 * e0.powi(2) + 115605.82 * e0.powi(3);
        g533 = -37995.78 + 161616.52 * e0 - 229838.2 * e0.powi(2) + 109377.94 * e0.powi(3);
    }

    // Calculate the quadruples
    let d2201 = 3 * n0.powi(2) / a0.powi(2) * (c22s22 * f220 * g201);
    let d2211 = 3 * n0.powi(2) / a0.powi(2) * (c22s22 * f221 * g211);
    let d3210 = 3 * n0.powi(2) / a0.powi(3) * (c32s32 * f321 * g310);
    let d3222 = 3 * n0.powi(2) / a0.powi(3) * (c32s32 * f322 * g322);
    let d5220 = 3 * n0.powi(2) / a0.powi(5) * (c52s52 * f522 * g520);
    let d5232 = 3 * n0.powi(2) / a0.powi(5) * (c52s52 * f523 * g532);
    let d4422 = 3 * n0.powi(2) / a0.powi(4) * (c44s44 * f442 * g422);
    let d5421 = 3 * n0.powi(2) / a0.powi(5) * (c54s54 * f542 * g521);
    let d5433 = 3 * n0.powi(2) / a0.powi(5) * (c54s54 * f543 * g533); // Typo in Hoots et al 2004
    let d4410 = 3 * n0.powi(2) / a0.powi(4) * (c44s44 * f441 * g410);

    // Return quadruples
    return Ok((d2201, d2211, d3210, d3222, d5220, d5232, d4422, d5421, d5433, d4410))
}

pub fn init_earth_gravity_resonance_wholeday(i0: f64, n0: f64, e0: f64, a0: f64) -> (f64, f64, f64, f64, f64, f64){
    // Precompute common quantities
    let cos_i0 = i0.cos();
    let sin_i0 = i0.sin();

    // Define constants
    let q31 = 2.1460748e-6;
    let q22 = 1.7891679e-6;
    let q33 = 2.2123015e-7;
    let lam31 = 0.13130908;
    let lam22 = 2.88431980;
    let lam33 = 0.37448087;

    // Calculate functions of inclination
    let f220 = (3./4.) * (1. + cos_i0).powi(2);
    let f311 = (15./16.) * sin_i0.powi(2) * (1. + 3. * cos_i0) - (3./4.) * (1. + cos_i0);
    let f330 = (15. / 8.) * (1. + cos_i0).powi(3);
    
    // Calculate functions of eccentricity
    let g200 = 1. - (5./2.) * e0.powi(2) + (13. / 16.) * e0.powi(4);
    let g310 = 1. + 2. * e0.powi(2);
    let g300 = 1. - 6. * e0.powi(2) + (423. / 64.) * e0.powi(4);

    // Calculate coefficients of the resonance terms
    let delta1 = (3. * n0.powi(2) / a0.powi(3)) * f311 * g310 * q31;
    let delta2 = (6. * n0.powi(2) / a0.powi(2)) * f220 * g200 * q22;
    let delta3 = (9. * n0.powi(2) / a0.powi(3)) * f330 * g300 * q33;

    // Return resonance terms
    return Ok((lam31, lam22, lam33, delta1, delta2, delta3));
}

pub fn sgp4_prop(sgp4: Sgp4, datetime: DateTime) -> {
    // Convert datetime to Julian day format
    let (jd_prop, jdfrac_prop) = utc2jday(datetime).unwrap();

    // Get days since epoch
    let delta_t = jd_prop + jdfrac_prop - (sgp4.jd0 + sgp4.jdfrac0);

    // Account for Earth zonal gravity and partial atmospheric drag effects
    let m_df = m0 + n0 * delta_t + m_dot * delta_t;
    let omega_df = omega0 + omega_dot * delta_t;
    let raan_df = raan0 + raan_dot * delta_t;
    let delta_omega = bstar * c3 * omega0.cos() * delta_t
}

// ----------
// Unit Tests
// ----------
#[cfg(test)]
mod tests {
    use super::*;
}