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
use crate::common;

// -------
// Structs
// -------

/// Simplified General Perturbations 4 (SGP4) parameters
///
/// This struct contains the epoch parameters which are necessary to propagate the state vectors of a satellite with SGP4
///
/// References
/// - [Revisiting Spacetrack Report #3: Rev 3 by Vallado et al](https://celestrak.org/publications/AIAA/2006-6753/AIAA-2006-6753-Rev3.pdf)
/// - [Fundamentals of Astrodynamics and Applications by Vallado et al](https://celestrak.org/software/vallado-sw.php)
/// - [History of Analytical Orbit Modeling in the U.S. Space Surveillance System by Hoots et al](https://arc.aiaa.org/doi/abs/10.2514/1.9161?casa_token=pVowNFT6MOkAAAAA%3A_DFsBbZwGC2QcMWxPhJN2k3suNrcP5YzV7NVBYSvwMxGy19RzX-AvUnyO9JT5Cku0cDYPfpIQm4&journalCode=jgcd)
pub struct Sgp4 {
    /// WGS model
    pub wgs: Wgs,

    /// TLE
    pub tle: Tle,

    /// Julian date at epoch \[days\]
    pub jd0: f64,

    /// Fractional Julian date at epoch \[days\]
    pub jdfrac0: f64,

    /// Deep space satellite 
    pub deep_space: bool,

    /// Brouwer mean elements at epoch
    pub brouwer0: BrouwerMeanElements,

    /// Atmospheric Drag Parameters
    pub atm_params: AtmDragParams,

    /// Earth Zonal Harmonics Parameters
    pub zonal_params: EarthZonalParams,
}

/// Brouwer Mean Orbital Elements
///
/// This struct contains the mean orbital elements of a TLE converted to Brouwer convention. TLEs report mean orbital elements
/// in Kozai convention.
///
/// References
/// - [Revisiting Spacetrack Report #3: Rev 3 by Vallado et al](https://celestrak.org/publications/AIAA/2006-6753/AIAA-2006-6753-Rev3.pdf)
/// - [Fundamentals of Astrodynamics and Applications by Vallado et al](https://celestrak.org/software/vallado-sw.php)
/// - [History of Analytical Orbit Modeling in the U.S. Space Surveillance System by Hoots et al](https://arc.aiaa.org/doi/abs/10.2514/1.9161?casa_token=pVowNFT6MOkAAAAA%3A_DFsBbZwGC2QcMWxPhJN2k3suNrcP5YzV7NVBYSvwMxGy19RzX-AvUnyO9JT5Cku0cDYPfpIQm4&journalCode=jgcd)
pub struct BrouwerMeanElements {
    /// Orbital inclination \[rad\]
    pub i: f64,

    /// The cosine of the orbital inclination
    pub theta: f64,

    /// Right ascension of the ascending node (RAAN) \[rad\]
    pub raan: f64,

    /// Orbital eccentricity \[\]
    pub e: f64,

    /// The square root of 1 minus the orbital eccentricity squared \[\]
    pub beta: f64,

    /// Argument of perigee \[rad\]
    pub omega: f64,

    /// Mean anomaly \[rad\]
    pub m: f64,

    /// Mean motion \[revs/min\]
    pub n: f64,

    /// Semi-major axis \[Earth Radii\]
    pub a: f64,

    /// The orbital period \[mins\]
    pub period: f64,
}

/// Atmospheric Drag Effects
///
/// This struct contains the parameters necessary to account for the impacts of atmospheric drag on an orbit.
///
/// References
/// - [Revisiting Spacetrack Report #3: Rev 3 by Vallado et al](https://celestrak.org/publications/AIAA/2006-6753/AIAA-2006-6753-Rev3.pdf)
/// - [Fundamentals of Astrodynamics and Applications by Vallado et al](https://celestrak.org/software/vallado-sw.php)
/// - [History of Analytical Orbit Modeling in the U.S. Space Surveillance System by Hoots et al](https://arc.aiaa.org/doi/abs/10.2514/1.9161?casa_token=pVowNFT6MOkAAAAA%3A_DFsBbZwGC2QcMWxPhJN2k3suNrcP5YzV7NVBYSvwMxGy19RzX-AvUnyO9JT5Cku0cDYPfpIQm4&journalCode=jgcd)
pub struct AtmDragParams {
    /// Perigee height \[km\]
    pub hp: f64,

    /// q0 parameter of power-law density function \[Earth Radii\]
    pub q0: f64,

    /// s parameter of power-law density function \[Earth Radii\]
    pub s: f64,

    /// Zeta constant \[1 / Earth Radii\]
    pub zeta: f64,

    /// Eta constant \[\]
    pub eta: f64,

    /// C1 constant \[\]
    pub c1: f64,

    /// C2 constant \[\]
    pub c2: f64,

    /// C3 constant \[\]
    pub c3: f64,

    /// C4 constant \[\]
    pub c4: f64,

    /// C5 constant \[\]
    pub c5: f64,

    /// D1 constant \[\]
    pub d1: f64,

    /// D2 constant \[\]
    pub d2: f64,

    /// D3 constant \[\]
    pub d3: f64,

    /// D4 constant \[\]
    pub d4: f64,
}

/// Earth Zonal Harmonics
///
/// This struct contains the parameters necessary to account for the impacts of Earth's zonal harmonics on an orbit.
///
/// References
/// - [Revisiting Spacetrack Report #3: Rev 3 by Vallado et al](https://celestrak.org/publications/AIAA/2006-6753/AIAA-2006-6753-Rev3.pdf)
/// - [Fundamentals of Astrodynamics and Applications by Vallado et al](https://celestrak.org/software/vallado-sw.php)
/// - [History of Analytical Orbit Modeling in the U.S. Space Surveillance System by Hoots et al](https://arc.aiaa.org/doi/abs/10.2514/1.9161?casa_token=pVowNFT6MOkAAAAA%3A_DFsBbZwGC2QcMWxPhJN2k3suNrcP5YzV7NVBYSvwMxGy19RzX-AvUnyO9JT5Cku0cDYPfpIQm4&journalCode=jgcd)
pub struct EarthZonalParams {
    /// Rate of change of mean anomaly \[rad / min\]
    pub m_dot: f64,

    /// Rate of change of the argument of perigee \[rad / min\]
    pub omega_dot: f64,

    /// Rate of change of the right ascension of the ascending node \[rad / min\]
    pub raan_dot: f64,
}

/// Solar and Lunar 3rd Body Effects
///
/// This struct contains the parameters necessary to account for the impacts of the Sun and Moon on an orbit.
///
/// References
/// - [Revisiting Spacetrack Report #3: Rev 3 by Vallado et al](https://celestrak.org/publications/AIAA/2006-6753/AIAA-2006-6753-Rev3.pdf)
/// - [Fundamentals of Astrodynamics and Applications by Vallado et al](https://celestrak.org/software/vallado-sw.php)
/// - [History of Analytical Orbit Modeling in the U.S. Space Surveillance System by Hoots et al](https://arc.aiaa.org/doi/abs/10.2514/1.9161?casa_token=pVowNFT6MOkAAAAA%3A_DFsBbZwGC2QcMWxPhJN2k3suNrcP5YzV7NVBYSvwMxGy19RzX-AvUnyO9JT5Cku0cDYPfpIQm4&journalCode=jgcd)
pub struct ThirdBodyParams {
    /// Rate of change of mean anomaly \[rad / min\]
    pub m_dot: f64,

    /// Rate of change of the argument of perigee \[rad / min\]
    pub omega_dot: f64,

    /// Rate of change of the right ascension of the ascending node \[rad / min\]
    pub raan_dot: f64,
}

// -----
// Enums
// -----

// ---------
// Constants
// ---------

/// A conversion from rev/day to rad/min
///
/// References
/// - [Revisiting Spacetrack Report #3: Rev 3 by Vallado et al](https://celestrak.org/publications/AIAA/2006-6753/AIAA-2006-6753-Rev3.pdf)
const XPDOTP: f64 = 229.1831180523293;

/// The rotational velocity of the earth in rad/min
///
/// References:
/// - [Revisiting Spacetrack Report #3: Rev 3 by Vallado et al](https://celestrak.org/publications/AIAA/2006-6753/AIAA-2006-6753-Rev3.pdf)
const RPTIM: f64 =  0.00437526908802;

// ---------
// Functions
// ---------

/// Build an [`SGP4`] struct for state propagation from a [`Tle`] struct
///
/// Given a [`Tle`] struct, calculate the time-independent parameters necessary 
/// to propagate a satellite's states in time. These parameters include
/// - Brouwer mean orbital elements
/// - Atmospheric drag parameters
/// - Earth zonal harmonics parameters
/// - Solar and Lunar 3rd body effects
/// - Resonance effects of Earth's gravity
///
/// # Arguments
/// * `tle` - The Two-Line Element parameters
/// * `wgs` - Optional, specify World Geodetic System (WGS) parameters (defaults to WGS-72, the standard for TLEs)
///
/// # Returns
/// * [`SGP4`] - The time-independent parameters for the SGP4 propagator
///
/// # Examples
/// ```rust
/// ```
/// References
/// - [Revisiting Spacetrack Report #3: Rev 3 by Vallado et al](https://celestrak.org/publications/AIAA/2006-6753/AIAA-2006-6753-Rev3.pdf)
/// - [Fundamentals of Astrodynamics and Applications by Vallado et al](https://celestrak.org/software/vallado-sw.php)
/// - [History of Analytical Orbit Modeling in the U.S. Space Surveillance System by Hoots et al](https://arc.aiaa.org/doi/abs/10.2514/1.9161?casa_token=pVowNFT6MOkAAAAA%3A_DFsBbZwGC2QcMWxPhJN2k3suNrcP5YzV7NVBYSvwMxGy19RzX-AvUnyO9JT5Cku0cDYPfpIQm4&journalCode=jgcd)
pub fn init_sgp4(tle: Tle, wgs: Option<common::Wgs>) -> Sgp4 {
    // Use WGS72 or custom WGS models if provided
    let mut wgs_sgp4 = common::WGS72;
    if let Some(wgs_passed) = wgs {
        wgs_sgp4 = wgs_passed;
    }

    // Extract TLE contents in proper units
    let i0 = deg2rad(tle.inclination); // [rad]
    let n0_kozai = tle.mean_motion * XPDOTP; // [revs/min]
    let e0 = tle.eccentricity; // []
    let omega0 = deg2rad(tle.argument_of_perigee); // [rad]
    let raan0 = deg2rad(tle.right_ascension_of_ascending_node); // [rad]
    let m0 = deg2rad(tle.mean_anomaly); // [rad]

    // Extract TLE epoch in Julian day format
    let datetime0 = dayofyr2utc(tle.epoch_year, tle.epoch_day).unwrap();
    let (jd0, jdfrac0) = utc2jday(datetime0).unwrap();

    // Recover Brouwer mean motion from Kozai mean motion (mean motion in TLE)
    let theta0 = i0.cos();
    let beta0 = (1. - e0.powi(2))sqrt();
    let a1 = (wgs_sgp4.ke / n0_kozai).powf(2./3.);
    let delta1 = (3./2.) * (wgs_sgp4.k2 / a1.powf(2.)) * (3. * i0.cos().powf(2.) - 1.) / (1. - e0.powf(2.)).powf(3./2.);
    let a2 = a1 * (1. - (1./3.) * delta1 - delta1.powf(2.) - (134./81.) * delta1.powf(3.));
    let delta0 = (3./2.) * (wgs_sgp4.k2 / a2.powf(2.)) * (3. * i0.cos().powf(2.) - 1.) / (1. - e0.powf(2.)).powf(3./2.);
    let n0 = n0_kozai / (1. + delta0);
    let a0 = (wgs_sgp4.ke / n0).powf(2./3.);
    let period0 = calc_period(a0, wgs_sgp4.mu); // [min]

    // Store Brouwer mean elements
    let brouwer0 = BrouwerMeanElements {
        i: i0,
        theta: theta0,
        raan: raan0,
        e: e0,
        beta: beta0,
        omega: omega0,
        m: m0,
        n: n0,
        a: a0,
        period: period0,
    }

    // Initialize atmospheric drag parameters
    let atm_params = init_atm_effects(wgs_sgp4, tle, brouwer0);

    // Initialize Earth zonal harmonics parameters
    let zonal_params = init_zonal_effects(wgs_sgp4, brouwer0);

    // Check for deep space satellite
    let mut deep_space = false;
    if period0 >= 225. {
        deep_space = true;
    }

    // Lunar and solar gravity effects (TODO logic)
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

    // Construct SGP4 propagator
    let sgp4 = Sgp4 {
        wgs: wgs_sgp4,
        tle: tle,
        jd0: jd0,
        jdfrac0: jdfrac0,
        deep_space: deep_space,
        brouwer0: brouwer0,
        atm_params: atm_params,
        zonal_params: zonal_params,
    };
    return sgp4;
}

///
/// References
/// - [Revisiting Spacetrack Report #3: Rev 3 by Vallado et al](https://celestrak.org/publications/AIAA/2006-6753/AIAA-2006-6753-Rev3.pdf)
/// - [Fundamentals of Astrodynamics and Applications by Vallado et al](https://celestrak.org/software/vallado-sw.php)
/// - [History of Analytical Orbit Modeling in the U.S. Space Surveillance System by Hoots et al](https://arc.aiaa.org/doi/abs/10.2514/1.9161?casa_token=pVowNFT6MOkAAAAA%3A_DFsBbZwGC2QcMWxPhJN2k3suNrcP5YzV7NVBYSvwMxGy19RzX-AvUnyO9JT5Cku0cDYPfpIQm4&journalCode=jgcd)
pub fn init_atm_effects(wgs: Wgs, tle: Tle, brouwer0: BrouwerMeanElements) -> AtmDragParams {
    // Define initial constants
    let a30 = -wgs.j3; // [Earth Radii^3]
    let q0 = (120. + wgs.r_earth_eq) / wgs.r_earth_eq; // [Earth radii]

    // Determine parameter s based on perigee height
    let rp = brouwer0.a0 * (1. - brouwer0.e0); // Radius of perigee [Earth Radii]
    let hp = (rp - 1.) * wgs.r_earth_eq; // Perigee height [km]
    
    let mut s = 0. // [Earth radii]
    if hp >= 156. {
        s = (78. + wgs.r_earth_eq) / wgs.r_earth_eq;
    } else if hp >= 98.{
        s = (hp - 78. + wgs.r_earth_eq) / wgs.r_earth_eq; // [Earth radii]
    } else {
        s = (20. + wgs.r_earth_eq) / wgs.r_earth_eq; // [Earth radii]
    }

    // Calculate atmospheric drag parameters
    let zeta = 1. / (brouwer0.a0 - s);
    let eta = brouwer0.a0 * brouwer0.e0 * zeta;
    
    let c2_1 = (q0 - s).powi(4) * zeta.powi(4) * brouwer0.n0 * (1. - eta.powi(2)).powf(-7./2.);
    let c2_2 = brouwer0.a0 * (1. + (3./2.) * eta.powi(2) + 4. * brouwer0.e0 * eta + brouwer0.e0 * eta.powi(3));
    let c2_3 = (3./2.) * (wgs.k2 * zeta / (1. - eta.powi(2))) * (-(1./2.) + (3./2.) * brouwer0.theta.powi(2)) * (8. + 24. * eta.powi(2) + 3. * eta.powi(4));
    let c2 = c2_1 * (c2_2 + c2_3);
    
    let c1 = tle.bstar * c2;
    let c3 = ((q0 - s).powf(4.) * zeta.powf(5.) * a30 * brouwer0.n0 * i0.sin()) / (wgs.k2 * brouwer0.e0);
    
    let c4_1 = 2. * brouwer0.n0 * (q0 - s).powi(4) * zeta.powi(4) * brouwer0.a0 * brouwer0.beta0.powi(2) * (1. - eta.powi(2)).powf(-7./2.);
    let c4_2 = 2. * eta * (1. + brouwer0.e0*eta) + 0.5 * brouwer0.e0 + 0.5 * eta.powi(3);
    let c4_3 = 2. * wgs.k2 * zeta / (brouwer0.a0 * (1. - eta.powi(2)));
    let c4_4 = 3. * (1. - 3. * brouwer0.theta.powi(2)) * (1. + 3./2. * eta.powi(2) - 2. * brouwer0.e0 * eta - 0.5 * brouwer0.e0 * eta.powi(3));
    let c4_5 = 3./4. * (1. - brouwer0.theta.powi(2)) * (2. * eta.powi(2) - brouwer0.e0 * eta - brouwer0.e0 * eta.powi(3)) * (2. * brouwer0.omega0).cos();
    let c4 = c4_1 * (c4_2 - c4_3 * (c4_4 + c4_5));
    
    let c5_1 = 2. * (q0 - s).powi(4) * zeta.powi(4) * brouwer0.a0 * brouwer0.beta0.powi(2) * (1. - eta.powi(2)).powf(-7./2.);
    let c5_2 = 1. + 11./4. * eta * (eta + brouwer0.e0) + brouwer0.e0 * eta.powi(3);
    let c5 = c5_1 * c5_2;
    
    let d2 = 4. * brouwer0.a0 * zeta * c1.powi(2);
    let d3 = 4./3. * brouwer0.a0 * zeta.powi(2) * (17. * brouwer0.a0 + s) * c1.powi(3);
    let d4 = 2./3. * brouwer0.a0.powi(2) * zeta.powi(3) * (221. * brouwer0.a0 + 31. * s) * c1.powi(4);

    // Store atmospheric drag parameters
    let atm_params = AtmDragParams {
        hp: hp,
        q0: q0,
        s: s,
        zeta: zeta,
        eta: eta,
        c1: c1,
        c2: c2,
        c3: c3,
        c4: c4,
        c5: c5,
        d1: d1,
        d2: d2,
        d3: d3,
        d4: d4,
    };

    return atm_params;
}

///
/// References
/// - [Revisiting Spacetrack Report #3: Rev 3 by Vallado et al](https://celestrak.org/publications/AIAA/2006-6753/AIAA-2006-6753-Rev3.pdf)
/// - [Fundamentals of Astrodynamics and Applications by Vallado et al](https://celestrak.org/software/vallado-sw.php)
/// - [History of Analytical Orbit Modeling in the U.S. Space Surveillance System by Hoots et al](https://arc.aiaa.org/doi/abs/10.2514/1.9161?casa_token=pVowNFT6MOkAAAAA%3A_DFsBbZwGC2QcMWxPhJN2k3suNrcP5YzV7NVBYSvwMxGy19RzX-AvUnyO9JT5Cku0cDYPfpIQm4&journalCode=jgcd)
pub fn init_zonal_effects(wgs: Wgs, brouwer0: BrouwerMeanElements) -> EarthZonalParams {
    // Calculate orbital element rates of change due to zonal harmonics
    let m_dot_1 = 3. * wgs.k2 * (-1. + 3. * brouwer0.theta.powi(2)) / (2. * brouwer0.a0.powi(2) * brouwer0.beta0.powi(3));
    let m_dot_2 = 3. * wgs.k2.powi(2) * (13. - 78. * brouwer0.theta.powi(2) + 137. * brouwer0.theta.powi(4)) / (16. * brouwer0.a0.powi(4) * brouwer0.beta0.powi(7));
    let m_dot = (m_dot_1 + m_dot_2) * brouwer0.n0;

    let omega_dot_1 = -3. * wgs.k2 * (1. - 5. * brouwer0.theta.powi(2)) / (2. * brouwer0.a0.powi(2) * brouwer0.beta0.powi(4));
    let omega_dot_2 = 3. * wgs.k2.powi(2) * (7. - 114. * brouwer0.theta.powi(2) + 395. * brouwer0.theta.powi(4)) / (16. * brouwer0.a0.powi(4) * brouwer0.beta0.powi(8));
    let omega_dot_3 = 5. * wgs.k4 * (3. - 36. * brouwer0.theta.powi(2) + 49. * brouwer0.theta.powi(4)) / (4. * brouwer0.a0.powi(4) * brouwer0.beta0.powi(8));
    let omega_dot = (omega_dot_1 + omega_dot_2 + omega_dot_3) * brouwer0.n0;

    let raan_dot_1 = -3. * wgs.k2 * brouwer0.theta / (brouwer0.a0.powi(2) * brouwer0.beta0.powi(4));
    let raan_dot_2 = 3. * wgs.k2.powi(2) * (4. * brouwer0.theta - 19. * brouwer0.theta.powi(3)) / (2. * brouwer0.a0.powi(4) * brouwer0.beta0.powi(8));
    let raan_dot_3 = 5. * wgs.k4 * brouwer0.theta * (3. - 7. * brouwer0.theta.powi(2)) / (2. * brouwer0.a0.powi(4) * brouwer0.beta0.powi(8));
    let raan_dot = (raan_dot_1 + raan_dot_2 + raan_dot_3) * brouwer0.n0;

    // Store Earth zonal parameters
    let zonal_params = EarthZonalParams {
        m_dot: m_dot,
        omega_dot: omega_dot,
        raan_dot: raan_dot
    };

    return zonal_params;
}

///
/// References
/// - [Revisiting Spacetrack Report #3: Rev 3 by Vallado et al](https://celestrak.org/publications/AIAA/2006-6753/AIAA-2006-6753-Rev3.pdf)
/// - [Fundamentals of Astrodynamics and Applications by Vallado et al](https://celestrak.org/software/vallado-sw.php)
/// - [History of Analytical Orbit Modeling in the U.S. Space Surveillance System by Hoots et al](https://arc.aiaa.org/doi/abs/10.2514/1.9161?casa_token=pVowNFT6MOkAAAAA%3A_DFsBbZwGC2QcMWxPhJN2k3suNrcP5YzV7NVBYSvwMxGy19RzX-AvUnyO9JT5Cku0cDYPfpIQm4&journalCode=jgcd)
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

///
/// References
/// - [Revisiting Spacetrack Report #3: Rev 3 by Vallado et al](https://celestrak.org/publications/AIAA/2006-6753/AIAA-2006-6753-Rev3.pdf)
/// - [Fundamentals of Astrodynamics and Applications by Vallado et al](https://celestrak.org/software/vallado-sw.php)
/// - [History of Analytical Orbit Modeling in the U.S. Space Surveillance System by Hoots et al](https://arc.aiaa.org/doi/abs/10.2514/1.9161?casa_token=pVowNFT6MOkAAAAA%3A_DFsBbZwGC2QcMWxPhJN2k3suNrcP5YzV7NVBYSvwMxGy19RzX-AvUnyO9JT5Cku0cDYPfpIQm4&journalCode=jgcd)
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

///
/// References
/// - [Revisiting Spacetrack Report #3: Rev 3 by Vallado et al](https://celestrak.org/publications/AIAA/2006-6753/AIAA-2006-6753-Rev3.pdf)
/// - [Fundamentals of Astrodynamics and Applications by Vallado et al](https://celestrak.org/software/vallado-sw.php)
/// - [History of Analytical Orbit Modeling in the U.S. Space Surveillance System by Hoots et al](https://arc.aiaa.org/doi/abs/10.2514/1.9161?casa_token=pVowNFT6MOkAAAAA%3A_DFsBbZwGC2QcMWxPhJN2k3suNrcP5YzV7NVBYSvwMxGy19RzX-AvUnyO9JT5Cku0cDYPfpIQm4&journalCode=jgcd)
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

///
/// References
/// - [Revisiting Spacetrack Report #3: Rev 3 by Vallado et al](https://celestrak.org/publications/AIAA/2006-6753/AIAA-2006-6753-Rev3.pdf)
/// - [Fundamentals of Astrodynamics and Applications by Vallado et al](https://celestrak.org/software/vallado-sw.php)
/// - [History of Analytical Orbit Modeling in the U.S. Space Surveillance System by Hoots et al](https://arc.aiaa.org/doi/abs/10.2514/1.9161?casa_token=pVowNFT6MOkAAAAA%3A_DFsBbZwGC2QcMWxPhJN2k3suNrcP5YzV7NVBYSvwMxGy19RzX-AvUnyO9JT5Cku0cDYPfpIQm4&journalCode=jgcd)
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

///
/// References
/// - [Revisiting Spacetrack Report #3: Rev 3 by Vallado et al](https://celestrak.org/publications/AIAA/2006-6753/AIAA-2006-6753-Rev3.pdf)
/// - [Fundamentals of Astrodynamics and Applications by Vallado et al](https://celestrak.org/software/vallado-sw.php)
/// - [History of Analytical Orbit Modeling in the U.S. Space Surveillance System by Hoots et al](https://arc.aiaa.org/doi/abs/10.2514/1.9161?casa_token=pVowNFT6MOkAAAAA%3A_DFsBbZwGC2QcMWxPhJN2k3suNrcP5YzV7NVBYSvwMxGy19RzX-AvUnyO9JT5Cku0cDYPfpIQm4&journalCode=jgcd)
pub fn sgp4_prop(sgp4: Sgp4, datetime: DateTime) -> {
    // Convert datetime to Julian day format
    let (jd_prop, jdfrac_prop) = utc2jday(datetime).unwrap();

    // Get minutes since epoch
    let delta_t = (jd_prop + jdfrac_prop - (sgp4.jd0 + sgp4.jdfrac0)) * 1440.;

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