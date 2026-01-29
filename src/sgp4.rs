// Module for propagating TLEs with SGP4

// ------------------
// External Libraries
// ------------------
use std::f64::consts::PI;

// ------------------
// Internal Libraries
// ------------------
use crate::tle::Tle;
use crate::time::{dayofyr2utc, utc2jday};
use crate::common::{Wgs, WGS72, deg2rad, calc_period};

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

    /// Solar 3rd Body Parameters
    pub solar_params: ThirdBodyParams,

    /// Lunar 3rd Body Parameters
    pub lunar_params: ThirdBodyParams,

    /// Account for whole day resonance effects of Earth's gravity
    pub whole_day_resonance: bool,

    /// Whole day resonance parameters of Earth's gravity
    pub whole_day_resonance_params: WholeDayResonanceParams,

    /// Account for half day resonance effects of Earth's gravity
    pub half_day_resonance: bool,

    /// Half day resonance parameters of Earth's gravity
    pub half_day_resonance_params: HalfDayResonanceParams,

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
#[derive(Default, Clone, Copy)]
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
#[derive(Default, Clone, Copy)]
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
#[derive(Default, Clone, Copy)]
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
#[derive(Default, Clone, Copy)]
pub struct ThirdBodyParams {
    /// Third body orbital inclination \[rad\]
    pub i: f64,

    /// Third body mean motion \[revs/min\]
    pub n: f64,

    /// Third body argument of perigee \[rad\]
    pub omega: f64,

    /// Third body right ascension of the ascending node (RAAN) \[rad\]
    pub raan: f64,

    /// Third body perturbation coefficient \[rad/min\]
    pub c: f64,

    /// x1 constant
    pub x1: f64,

    /// x2 constant
    pub x2: f64,

    /// x3 constant
    pub x3: f64,

    /// x4 constant
    pub x4: f64,

    /// x5 constant
    pub x5: f64,

    /// x6 constant 
    pub x6: f64,

    /// x7 constant
    pub x7: f64,

    /// x8 constant
    pub x8: f64,

    /// z1 constant
    pub z1: f64,

    /// z2 constant
    pub z2: f64,

    /// z3 constant
    pub z3: f64,

    /// z11 constant
    pub z11: f64,

    /// z13 constant
    pub z13: f64,

    /// z21 constant
    pub z21: f64,

    /// z23 constant
    pub z23: f64,

    /// z22 constant
    pub z22: f64,

    /// z12 constant
    pub z12: f64,

    /// z31 constant
    pub z31: f64,

    /// z32 constant
    pub z32: f64,

    /// z33 constant
    pub z33: f64,

    /// Rate of change of the orbital eccentricity \[1 / min\]
    pub e_dot: f64,

    /// Rate of change of the orbital inclination \[rad / min\]
    pub i_dot: f64,

    /// Rate of change of the mean anomaly \[rad / min\]
    pub m_dot: f64,

    /// Rate of change of the argument of perigee \[rad / min\]
    pub omega_dot: f64,

    /// Rate of change of the right ascension of the ascending node \[rad / min\]
    pub raan_dot: f64,
}

/// Half day resonance effects of Earth's gravity
///
/// This struct contains the parameters necessary to account for the impacts of half day resonance effects on an orbit.
///
/// References
/// - [Revisiting Spacetrack Report #3: Rev 3 by Vallado et al](https://celestrak.org/publications/AIAA/2006-6753/AIAA-2006-6753-Rev3.pdf)
/// - [Fundamentals of Astrodynamics and Applications by Vallado et al](https://celestrak.org/software/vallado-sw.php)
/// - [History of Analytical Orbit Modeling in the U.S. Space Surveillance System by Hoots et al](https://arc.aiaa.org/doi/abs/10.2514/1.9161?casa_token=pVowNFT6MOkAAAAA%3A_DFsBbZwGC2QcMWxPhJN2k3suNrcP5YzV7NVBYSvwMxGy19RzX-AvUnyO9JT5Cku0cDYPfpIQm4&journalCode=jgcd)
#[derive(Default, Clone, Copy)]
pub struct HalfDayResonanceParams {
    /// d2201 constant
    pub d2201: f64,

    /// d2211 constant
    pub d2211: f64,

    /// d3210 constant
    pub d3210: f64,

    /// d3222 constant
    pub d3222: f64,

    /// d5220 constant
    pub d5220: f64,

    /// d5232 constant
    pub d5232: f64,

    /// d4422 constant
    pub d4422: f64,

    /// d5421 constant
    pub d5421: f64,

    /// d5433 constant
    pub d5433: f64,

    /// d4410 constant
    pub d4410: f64,
}

/// Whole day resonance effects of Earth's gravity
///
/// This struct contains the parameters necessary to account for the impacts of whole day resonance effects on an orbit.
///
/// References
/// - [Revisiting Spacetrack Report #3: Rev 3 by Vallado et al](https://celestrak.org/publications/AIAA/2006-6753/AIAA-2006-6753-Rev3.pdf)
/// - [Fundamentals of Astrodynamics and Applications by Vallado et al](https://celestrak.org/software/vallado-sw.php)
/// - [History of Analytical Orbit Modeling in the U.S. Space Surveillance System by Hoots et al](https://arc.aiaa.org/doi/abs/10.2514/1.9161?casa_token=pVowNFT6MOkAAAAA%3A_DFsBbZwGC2QcMWxPhJN2k3suNrcP5YzV7NVBYSvwMxGy19RzX-AvUnyO9JT5Cku0cDYPfpIQm4&journalCode=jgcd)
#[derive(Default, Clone, Copy)]
pub struct WholeDayResonanceParams {
    /// lam31 constant
    pub lam31: f64,

    /// lam22 constant
    pub lam22: f64,

    /// lam33 constant
    pub lam33: f64,

    /// delta1 constant
    pub delta1: f64,

    /// delta2 constant
    pub delta2: f64,

    /// delta3 constant
    pub delta3: f64,
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

/// Build an [`Sgp4`] struct for state propagation from a [`Tle`] struct
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
/// * [`Sgp4`] - The time-independent parameters for the SGP4 propagator
///
/// # Examples
/// ```rust
/// // Define TLE
/// let tle = Tle::default();
/// 
/// // Define WGS model
/// let wgs = WGS72;
///
/// // Initialize the SGP4 propagator
/// let sgp4 = init_sgp4(&tle, Some(&wgs));
/// ```
/// References
/// - [Revisiting Spacetrack Report #3: Rev 3 by Vallado et al](https://celestrak.org/publications/AIAA/2006-6753/AIAA-2006-6753-Rev3.pdf)
/// - [Fundamentals of Astrodynamics and Applications by Vallado et al](https://celestrak.org/software/vallado-sw.php)
/// - [History of Analytical Orbit Modeling in the U.S. Space Surveillance System by Hoots et al](https://arc.aiaa.org/doi/abs/10.2514/1.9161?casa_token=pVowNFT6MOkAAAAA%3A_DFsBbZwGC2QcMWxPhJN2k3suNrcP5YzV7NVBYSvwMxGy19RzX-AvUnyO9JT5Cku0cDYPfpIQm4&journalCode=jgcd)
pub fn init_sgp4(tle: &Tle, wgs: Option<&Wgs>) -> Sgp4 {
    // Use WGS72 or custom WGS models if provided
    let wgs_sgp4 = if let Some(wgs_passed) = wgs { *wgs_passed } else { WGS72 };

    // Extract TLE contents in proper units
    let i0 = deg2rad(tle.inclination); // [rad]
    let n0_kozai = tle.mean_motion * XPDOTP; // [rad/min]
    let e0 = tle.eccentricity; // []
    let omega0 = deg2rad(tle.argument_of_perigee); // [rad]
    let raan0 = deg2rad(tle.right_ascension_of_ascending_node); // [rad]
    let m0 = deg2rad(tle.mean_anomaly); // [rad]

    // Extract TLE epoch in Julian day format
    let datetime0 = dayofyr2utc(tle.epoch_year, tle.epoch_day).unwrap();
    let (jd0, jdfrac0) = utc2jday(&datetime0).unwrap();

    // Recover Brouwer mean motion from Kozai mean motion (mean motion in TLE)
    let theta0 = i0.cos();
    let beta0 = (1. - e0.powi(2)).sqrt();
    let a1 = (wgs_sgp4.ke / n0_kozai).powf(2./3.);
    let delta1 = (3./2.) * (wgs_sgp4.k2 / a1.powf(2.)) * (3. * i0.cos().powf(2.) - 1.) / (1. - e0.powf(2.)).powf(3./2.);
    let a2 = a1 * (1. - (1./3.) * delta1 - delta1.powf(2.) - (134./81.) * delta1.powf(3.));
    let delta0 = (3./2.) * (wgs_sgp4.k2 / a2.powf(2.)) * (3. * i0.cos().powf(2.) - 1.) / (1. - e0.powf(2.)).powf(3./2.);
    let n0 = n0_kozai / (1. + delta0); // [rad/min]
    let a0 = (wgs_sgp4.ke / n0).powf(2./3.); // [Earth radii]
    let a0_km = a0 * wgs_sgp4.r_earth_eq; // [km]
    let period0 = calc_period(a0_km, wgs_sgp4.mu); // [min]

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
    };

    // Initialize atmospheric drag parameters
    let atm_params = init_atm_effects(&wgs_sgp4, tle, &brouwer0);

    // Initialize Earth zonal harmonics parameters
    let zonal_params = init_zonal_effects(&wgs_sgp4, &brouwer0);

    // Check for deep space satellite
    let mut deep_space = false;
    if period0 >= 225. {
        deep_space = true;
    }

    // Lunar and solar gravity effects
    let (lunar_params, solar_params) = init_lunar_solar_effects(deep_space, jd0, jdfrac0, &brouwer0);

    // Earth gravity resonance effects
    let mut whole_day_resonance = false;
    let mut half_day_resonance = false;
    let mut whole_day_resonance_params = WholeDayResonanceParams::default();
    let mut half_day_resonance_params = HalfDayResonanceParams::default();
    if period0 >= 1200. && period0 <= 1800. {
        whole_day_resonance = true;
        whole_day_resonance_params = init_earth_gravity_resonance_wholeday(&brouwer0);
    } else if period0 >= 680. && period0 <= 760. {
        half_day_resonance = true;
        half_day_resonance_params = init_earth_gravity_resonance_halfday(&brouwer0);
    }

    // Construct SGP4 propagator
    let sgp4 = Sgp4 {
        wgs: wgs_sgp4,
        tle: tle.clone(),
        jd0: jd0,
        jdfrac0: jdfrac0,
        deep_space: deep_space,
        brouwer0: brouwer0,
        atm_params: atm_params,
        zonal_params: zonal_params,
        lunar_params: lunar_params,
        solar_params: solar_params,
        whole_day_resonance: whole_day_resonance,
        whole_day_resonance_params: whole_day_resonance_params,
        half_day_resonance: half_day_resonance,
        half_day_resonance_params: half_day_resonance_params,
    };

    return sgp4;
}

/// Initialize the atmospheric drag effects
///
/// # Arguments
/// * `wgs` - The WGS model
/// * `tle` - The TLE
/// * `brouwer0` - The Brouwer mean elements at epoch
///
/// # Returns
/// * `AtmDragParams` - The atmospheric drag parameters
///
/// # Examples
/// ```rust
/// // Define WGS model
/// let wgs = WGS72;
///
/// // Initialize the atmospheric drag effects
/// let atm_params = init_atm_effects(&wgs, &tle, &brouwer0);
/// ```
///
/// References
/// - [Revisiting Spacetrack Report #3: Rev 3 by Vallado et al](https://celestrak.org/publications/AIAA/2006-6753/AIAA-2006-6753-Rev3.pdf)
/// - [Fundamentals of Astrodynamics and Applications by Vallado et al](https://celestrak.org/software/vallado-sw.php)
/// - [History of Analytical Orbit Modeling in the U.S. Space Surveillance System by Hoots et al](https://arc.aiaa.org/doi/abs/10.2514/1.9161?casa_token=pVowNFT6MOkAAAAA%3A_DFsBbZwGC2QcMWxPhJN2k3suNrcP5YzV7NVBYSvwMxGy19RzX-AvUnyO9JT5Cku0cDYPfpIQm4&journalCode=jgcd)
pub fn init_atm_effects(wgs: &Wgs, tle: &Tle, brouwer0: &BrouwerMeanElements) -> AtmDragParams {
    // Define initial constants
    let a30 = -wgs.j3; // [Earth Radii^3]
    let q0 = (120. + wgs.r_earth_eq) / wgs.r_earth_eq; // [Earth radii]

    // Determine parameter s based on perigee height
    let rp = brouwer0.a * (1. - brouwer0.e); // Radius of perigee [Earth Radii]
    let hp = (rp - 1.) * wgs.r_earth_eq; // Perigee height [km]
    
    let mut s = 0.; // [Earth radii]
    if hp >= 156. {
        s = (78. + wgs.r_earth_eq) / wgs.r_earth_eq;
    } else if hp >= 98.{
        s = (hp - 78. + wgs.r_earth_eq) / wgs.r_earth_eq; // [Earth radii]
    } else {
        s = (20. + wgs.r_earth_eq) / wgs.r_earth_eq; // [Earth radii]
    }

    // Calculate atmospheric drag parameters
    let zeta = 1. / (brouwer0.a - s);
    let eta = brouwer0.a * brouwer0.e * zeta;
    
    let c2_1 = (q0 - s).powi(4) * zeta.powi(4) * brouwer0.n * (1. - eta.powi(2)).powf(-7./2.);
    let c2_2 = brouwer0.a * (1. + (3./2.) * eta.powi(2) + 4. * brouwer0.e * eta + brouwer0.e * eta.powi(3));
    let c2_3 = (3./2.) * (wgs.k2 * zeta / (1. - eta.powi(2))) * (-(1./2.) + (3./2.) * brouwer0.theta.powi(2)) * (8. + 24. * eta.powi(2) + 3. * eta.powi(4));
    let c2 = c2_1 * (c2_2 + c2_3);
    
    let c1 = tle.bstar * c2;
    let c3 = ((q0 - s).powf(4.) * zeta.powf(5.) * a30 * brouwer0.n * brouwer0.i.sin()) / (wgs.k2 * brouwer0.e);
    
    let c4_1 = 2. * brouwer0.n * (q0 - s).powi(4) * zeta.powi(4) * brouwer0.a * brouwer0.beta.powi(2) * (1. - eta.powi(2)).powf(-7./2.);
    let c4_2 = 2. * eta * (1. + brouwer0.e*eta) + 0.5 * brouwer0.e + 0.5 * eta.powi(3);
    let c4_3 = 2. * wgs.k2 * zeta / (brouwer0.a * (1. - eta.powi(2)));
    let c4_4 = 3. * (1. - 3. * brouwer0.theta.powi(2)) * (1. + 3./2. * eta.powi(2) - 2. * brouwer0.e * eta - 0.5 * brouwer0.e * eta.powi(3));
    let c4_5 = 3./4. * (1. - brouwer0.theta.powi(2)) * (2. * eta.powi(2) - brouwer0.e * eta - brouwer0.e * eta.powi(3)) * (2. * brouwer0.omega).cos();
    let c4 = c4_1 * (c4_2 - c4_3 * (c4_4 + c4_5));
    
    let c5_1 = 2. * (q0 - s).powi(4) * zeta.powi(4) * brouwer0.a * brouwer0.beta.powi(2) * (1. - eta.powi(2)).powf(-7./2.);
    let c5_2 = 1. + 11./4. * eta * (eta + brouwer0.e) + brouwer0.e * eta.powi(3);
    let c5 = c5_1 * c5_2;
    
    let d2 = 4. * brouwer0.a * zeta * c1.powi(2);
    let d3 = 4./3. * brouwer0.a * zeta.powi(2) * (17. * brouwer0.a + s) * c1.powi(3);
    let d4 = 2./3. * brouwer0.a.powi(2) * zeta.powi(3) * (221. * brouwer0.a + 31. * s) * c1.powi(4);

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
        d2: d2,
        d3: d3,
        d4: d4,
    };

    return atm_params;
}

/// Initialize the Earth zonal harmonics effects
///
/// # Arguments
/// * `wgs` - The WGS model
/// * `brouwer0` - The Brouwer mean elements at epoch
///
/// # Returns
/// * `EarthZonalParams` - The Earth zonal harmonics parameters
///
/// # Examples
/// ```rust
/// // Define Brouwer mean elements at epoch
/// let brouwer0 = BrouwerMeanElements::default();
///
/// // Initialize the Earth zonal harmonics effects
/// let zonal_params = init_zonal_effects(&WGS72, &brouwer0);
/// ```
///
/// References
/// - [Revisiting Spacetrack Report #3: Rev 3 by Vallado et al](https://celestrak.org/publications/AIAA/2006-6753/AIAA-2006-6753-Rev3.pdf)
/// - [Fundamentals of Astrodynamics and Applications by Vallado et al](https://celestrak.org/software/vallado-sw.php)
/// - [History of Analytical Orbit Modeling in the U.S. Space Surveillance System by Hoots et al](https://arc.aiaa.org/doi/abs/10.2514/1.9161?casa_token=pVowNFT6MOkAAAAA%3A_DFsBbZwGC2QcMWxPhJN2k3suNrcP5YzV7NVBYSvwMxGy19RzX-AvUnyO9JT5Cku0cDYPfpIQm4&journalCode=jgcd)
pub fn init_zonal_effects(wgs: &Wgs, brouwer0: &BrouwerMeanElements) -> EarthZonalParams {
    // Calculate orbital element rates of change due to zonal harmonics
    let m_dot_1 = 3. * wgs.k2 * (-1. + 3. * brouwer0.theta.powi(2)) / (2. * brouwer0.a.powi(2) * brouwer0.beta.powi(3));
    let m_dot_2 = 3. * wgs.k2.powi(2) * (13. - 78. * brouwer0.theta.powi(2) + 137. * brouwer0.theta.powi(4)) / (16. * brouwer0.a.powi(4) * brouwer0.beta.powi(7));
    let m_dot = (m_dot_1 + m_dot_2) * brouwer0.n;

    let omega_dot_1 = -3. * wgs.k2 * (1. - 5. * brouwer0.theta.powi(2)) / (2. * brouwer0.a.powi(2) * brouwer0.beta.powi(4));
    let omega_dot_2 = 3. * wgs.k2.powi(2) * (7. - 114. * brouwer0.theta.powi(2) + 395. * brouwer0.theta.powi(4)) / (16. * brouwer0.a.powi(4) * brouwer0.beta.powi(8));
    let omega_dot_3 = 5. * wgs.k4 * (3. - 36. * brouwer0.theta.powi(2) + 49. * brouwer0.theta.powi(4)) / (4. * brouwer0.a.powi(4) * brouwer0.beta.powi(8));
    let omega_dot = (omega_dot_1 + omega_dot_2 + omega_dot_3) * brouwer0.n;

    let raan_dot_1 = -3. * wgs.k2 * brouwer0.theta / (brouwer0.a.powi(2) * brouwer0.beta.powi(4));
    let raan_dot_2 = 3. * wgs.k2.powi(2) * (4. * brouwer0.theta - 19. * brouwer0.theta.powi(3)) / (2. * brouwer0.a.powi(4) * brouwer0.beta.powi(8));
    let raan_dot_3 = 5. * wgs.k4 * brouwer0.theta * (3. - 7. * brouwer0.theta.powi(2)) / (2. * brouwer0.a.powi(4) * brouwer0.beta.powi(8));
    let raan_dot = (raan_dot_1 + raan_dot_2 + raan_dot_3) * brouwer0.n;

    // Store Earth zonal parameters
    let zonal_params = EarthZonalParams {
        m_dot: m_dot,
        omega_dot: omega_dot,
        raan_dot: raan_dot
    };

    return zonal_params;
}

/// Initialize the Lunar and Solar third body effects
///
/// # Arguments
/// * `deep_space` - Is this a deep space satellite
/// * `jd0` - The Julian date at epoch \[days\]
/// * `jdfrac0` - The fractional Julian date at epoch \[days\]
/// * `brouwer0` - The Brouwer mean elements at epoch
///
/// # Returns
/// * `(ThirdBodyParams, ThirdBodyParams)` - The Lunar and Solar third body parameters
///
/// # Examples
/// ```rust
/// // Define Brouwer mean elements at epoch
/// let brouwer0 = BrouwerMeanElements::default();
///
/// // Initialize the Lunar and Solar third body effects
/// let (lunar_params, solar_params) = init_lunar_solar_effects(true, jd0, jdfrac0, &brouwer0);
/// ```
///
/// References
/// - [Revisiting Spacetrack Report #3: Rev 3 by Vallado et al](https://celestrak.org/publications/AIAA/2006-6753/AIAA-2006-6753-Rev3.pdf)
/// - [Fundamentals of Astrodynamics and Applications by Vallado et al](https://celestrak.org/software/vallado-sw.php)
/// - [History of Analytical Orbit Modeling in the U.S. Space Surveillance System by Hoots et al](https://arc.aiaa.org/doi/abs/10.2514/1.9161?casa_token=pVowNFT6MOkAAAAA%3A_DFsBbZwGC2QcMWxPhJN2k3suNrcP5YzV7NVBYSvwMxGy19RzX-AvUnyO9JT5Cku0cDYPfpIQm4&journalCode=jgcd)
pub fn init_lunar_solar_effects(deep_space: bool, jd0: f64, jdfrac0: f64, brouwer0: &BrouwerMeanElements) -> (ThirdBodyParams, ThirdBodyParams) {
    // Check if the satellite is not in deep space
    if !deep_space {
        return (ThirdBodyParams::default(), ThirdBodyParams::default());
    }

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
    let delta_t = jd0 + jdfrac0 - epoch_sm;

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
    let lunar_params = calc_lunar_solar_secular_rates(i_m, n_m, omega_m, raan_m, c_m, brouwer0);

    // Calculate the Solar secular rates
    let solar_params = calc_lunar_solar_secular_rates(i_s, n_s, omega_s, raan_s, c_s, brouwer0);

    return (lunar_params, solar_params);
}

/// Calculate the secular rates of a third body's orbital elements
///
/// # Arguments
/// * `i_x` - The third body orbital inclination \[rad\]
/// * `n_x` - The third body mean motion \[rad/min\]
/// * `omega_x` - The third body argument of perigee \[rad\]
/// * `raan_x` - The third body right ascension of the ascending node (RAAN) \[rad\]
/// * `c_x` - The third body perturbation coefficient \[rad/min\]
/// * `brouwer0` - The Brouwer mean elements at epoch
///
/// # Returns
/// * `ThirdBodyParams` - The third body parameters
///
/// # Examples
/// ```rust
/// // Define Brouwer mean elements at epoch
/// let brouwer0 = BrouwerMeanElements::default();
///
/// // Calculate the Lunar secular rates
/// let lunar_params = calc_lunar_solar_secular_rates(i_m, n_m, omega_m, raan_m, c_m, &brouwer0);
/// ```
///
/// References
/// - [Revisiting Spacetrack Report #3: Rev 3 by Vallado et al](https://celestrak.org/publications/AIAA/2006-6753/AIAA-2006-6753-Rev3.pdf)
/// - [Fundamentals of Astrodynamics and Applications by Vallado et al](https://celestrak.org/software/vallado-sw.php)
/// - [History of Analytical Orbit Modeling in the U.S. Space Surveillance System by Hoots et al](https://arc.aiaa.org/doi/abs/10.2514/1.9161?casa_token=pVowNFT6MOkAAAAA%3A_DFsBbZwGC2QcMWxPhJN2k3suNrcP5YzV7NVBYSvwMxGy19RzX-AvUnyO9JT5Cku0cDYPfpIQm4&journalCode=jgcd)
pub fn calc_lunar_solar_secular_rates(i_x: f64, n_x: f64, omega_x: f64, raan_x: f64, c_x: f64, brouwer0: &BrouwerMeanElements) -> ThirdBodyParams {
    // Precompute common quantities
    let cos_raan_diff = (brouwer0.raan - raan_x).cos();
    let sin_raan_diff = (brouwer0.raan - raan_x).sin();
    let cos_omega_x = omega_x.cos();
    let sin_omega_x = omega_x.sin();
    let cos_omega0 = brouwer0.omega.cos();
    let sin_omega0 = brouwer0.omega.sin();
    let cos_i_x = i_x.cos();
    let sin_i_x = i_x.sin();
    let cos_i0 = brouwer0.i.cos();
    let sin_i0 = brouwer0.i.sin();

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
    let z1 = 6. * (a1.powi(2) + a2.powi(2)) + (1. + brouwer0.e.powi(2)) * z31;
    let z2 = 12. * (a1 * a3 + a2 * a4) + (1. + brouwer0.e.powi(2)) * z32;
    let z3 = 6. * (a3.powi(2) + a4.powi(2)) + (1. + brouwer0.e.powi(2)) * z33;
    let z11 = -6. * a1 * a5 + brouwer0.e.powi(2) * (-24. * x1 * x7 - 6. * x3 * x5);
    let z13 = -6. * a3 * a6 + brouwer0.e.powi(2) * (-24. * x2 * x8 - 6. * x4 * x6);
    let z21 = 6. * a2 * a5 + brouwer0.e.powi(2) * (24. * x1 * x5 - 6. * x3 * x7);
    let z23 = 6. * a4 * a6 + brouwer0.e.powi(2) * (24. * x2 * x6 - 6. * x4 * x8);
    let z22 = 6. * a4 * a5 + 6. * a2 * a6 + brouwer0.e.powi(2) * (24. * x2 * x5 + 24. * x1 * x6 - 6. * x4 * x7 - 6. * x3 * x8);
    let z12 = -6. * a1 * a6 - 6. * a3 * a5 - brouwer0.e.powi(2) * (24. * x2 * x7 + 24. * x1 * x8 + 6. * x3 * x6 + 6. * x4 * x5);

    // Calculate secular rates
    let e_x_dot = -15. * c_x * n_x * (brouwer0.e * brouwer0.beta / brouwer0.n) * (x1 * x3 + x2 * x4);
    
    let i_x_dot = (-c_x * n_x / (2. * brouwer0.n * brouwer0.beta)) * (z11 + z13);
    
    let m_x_dot = (-c_x * n_x / brouwer0.n) * (z1 + z3 - 14. - 6. * brouwer0.e.powi(2));
    
    let mut raan_x_dot = 0.;
    if brouwer0.i >= deg2rad(3.) {
        raan_x_dot = c_x * n_x / (2. * brouwer0.n * brouwer0.beta * sin_i0) * (z21 + z23);
    }

    let mut omega_x_dot = c_x * n_x * brouwer0.beta / brouwer0.n * (z31 + z33 - 6.);
    if brouwer0.i >= deg2rad(3.) {
        omega_x_dot = omega_x_dot - raan_x_dot * cos_i0;
    }

    // Store the 3rd body parameters
    let third_body_params = ThirdBodyParams {
        i: i_x,
        n: n_x,
        omega: omega_x,
        raan: raan_x,
        c: c_x,
        x1: x1,
        x2: x2,
        x3: x3,
        x4: x4,
        x5: x5,
        x6: x6,
        x7: x7,
        x8: x8,
        z1: z1,
        z2: z2,
        z3: z3,
        z11: z11,
        z13: z13,
        z21: z21,
        z23: z23,
        z22: z22,
        z12: z12,
        z31: z31,
        z32: z32,
        z33: z33,
        e_dot: e_x_dot,
        i_dot: i_x_dot,
        m_dot: m_x_dot,
        raan_dot: raan_x_dot,
        omega_dot: omega_x_dot,
    };

    // Return 3rd body parameters
    return third_body_params;
}

/// Initialize the half day resonance effects of Earth's gravity
///
/// # Arguments
/// * `brouwer0` - The Brouwer mean elements at epoch
///
/// # Returns
/// * `HalfDayResonanceParams` - The half day resonance parameters
///
/// # Examples
/// ```rust
/// // Define Brouwer mean elements at epoch
/// let brouwer0 = BrouwerMeanElements::default();
///
/// // Initialize half day resonance effects
/// let half_day_resonance_params = init_earth_gravity_resonance_halfday(&brouwer0);
/// ```
///
/// References
/// - [Revisiting Spacetrack Report #3: Rev 3 by Vallado et al](https://celestrak.org/publications/AIAA/2006-6753/AIAA-2006-6753-Rev3.pdf)
/// - [Fundamentals of Astrodynamics and Applications by Vallado et al](https://celestrak.org/software/vallado-sw.php)
/// - [History of Analytical Orbit Modeling in the U.S. Space Surveillance System by Hoots et al](https://arc.aiaa.org/doi/abs/10.2514/1.9161?casa_token=pVowNFT6MOkAAAAA%3A_DFsBbZwGC2QcMWxPhJN2k3suNrcP5YzV7NVBYSvwMxGy19RzX-AvUnyO9JT5Cku0cDYPfpIQm4&journalCode=jgcd)
pub fn init_earth_gravity_resonance_halfday(brouwer0: &BrouwerMeanElements) -> HalfDayResonanceParams {
    // Precompute common quantities
    let cos_i0 = brouwer0.i.cos();
    let sin_i0 = brouwer0.i.sin();

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
    let f543 = (945./32.) * sin_i0 * (cos_i0.powi(2) * (12. + 8. * cos_i0 - 10. * cos_i0.powi(2)) - 2. - 8. * cos_i0);

    // Calculate functions of eccentricity
    let g211: f64;
    let g201 = -0.306 - 0.44 * (brouwer0.e - 0.64);
    let g310: f64;
    let g322: f64;
    let g410: f64;
    let g422: f64;
    let g520: f64;
    let g521: f64;
    let g532: f64;
    let g533: f64;
    if brouwer0.e <= 0.65 {
        g211 = 3.616 - 13.247 * brouwer0.e + 16.29 * brouwer0.e.powi(2);
        g310 = -19.302 + 117.39 * brouwer0.e - 228.419 * brouwer0.e.powi(2) + 156.591 * brouwer0.e.powi(3);
        g322 = -18.9068 + 109.7927 * brouwer0.e - 214.6334 * brouwer0.e.powi(2) + 146.5816 * brouwer0.e.powi(2);
        g410 = -41.122 + 242.694 * brouwer0.e - 471.094 * brouwer0.e.powi(2) + 313.953 * brouwer0.e.powi(3);
        g422 = -146.407 + 841.88 * brouwer0.e - 1629.014 * brouwer0.e.powi(2) + 1083.435 * brouwer0.e.powi(3);
        g520 = -532.114 + 3017.977 * brouwer0.e - 5740.032 * brouwer0.e.powi(2) + 3708.276 * brouwer0.e.powi(3);
    } else {
        g211 = -72.099 + 331.819 * brouwer0.e - 508.738 * brouwer0.e.powi(2) + 266.724 * brouwer0.e.powi(3);
        g310 = -346.844 + 1582.851 * brouwer0.e - 2415.925 * brouwer0.e.powi(2) + 1246.113 * brouwer0.e.powi(3);
        g322 = -342.585 + 1554.908 * brouwer0.e - 2366.899 * brouwer0.e.powi(2) + 1215.972 * brouwer0.e.powi(3);
        g410 = -1052.797 + 4758.686 * brouwer0.e - 7193.992 * brouwer0.e.powi(2) + 3651.957 * brouwer0.e.powi(3);
        g422 = -3581.69 + 16178.11 * brouwer0.e - 24462.77 * brouwer0.e.powi(2) + 12422.52 * brouwer0.e.powi(3);
        if brouwer0.e < 0.715 {
            g520 = 1464.74 - 4664.75 * brouwer0.e + 3763.64 * brouwer0.e.powi(2);
        } else {
            g520 = -5149.66 + 29936.92 * brouwer0.e - 54087.36 * brouwer0.e.powi(2) + 31324.56 * brouwer0.e.powi(3);
        }
    }
    if brouwer0.e < 0.7 {
        g521 = -822.71072 + 4568.6173 * brouwer0.e - 8491.4146 * brouwer0.e.powi(2) + 5337.524 * brouwer0.e.powi(3);
        g532 = -853.666 + 4690.25 * brouwer0.e - 8624.77 * brouwer0.e.powi(2) + 5341.4 * brouwer0.e.powi(3);
        g533 = -919.2277 + 4988.61 * brouwer0.e - 9064.77 * brouwer0.e.powi(2) + 5542.21 * brouwer0.e.powi(3);
    } else {
        g521 = -51752.104 + 218913.95 * brouwer0.e - 309468.16 * brouwer0.e.powi(2) + 146349.42 * brouwer0.e.powi(3);
        g532 = -40023.88 + 170470.89 * brouwer0.e - 242699.48 * brouwer0.e.powi(2) + 115605.82 * brouwer0.e.powi(3);
        g533 = -37995.78 + 161616.52 * brouwer0.e - 229838.2 * brouwer0.e.powi(2) + 109377.94 * brouwer0.e.powi(3);
    }

    // Calculate the quadruples
    let d2201 = 3. * brouwer0.n.powi(2) / brouwer0.a.powi(2) * (c22s22 * f220 * g201);
    let d2211 = 3. * brouwer0.n.powi(2) / brouwer0.a.powi(2) * (c22s22 * f221 * g211);
    let d3210 = 3. * brouwer0.n.powi(2) / brouwer0.a.powi(3) * (c32s32 * f321 * g310);
    let d3222 = 3. * brouwer0.n.powi(2) / brouwer0.a.powi(3) * (c32s32 * f322 * g322);
    let d5220 = 3. * brouwer0.n.powi(2) / brouwer0.a.powi(5) * (c52s52 * f522 * g520);
    let d5232 = 3. * brouwer0.n.powi(2) / brouwer0.a.powi(5) * (c52s52 * f523 * g532);
    let d4422 = 3. * brouwer0.n.powi(2) / brouwer0.a.powi(4) * (c44s44 * f442 * g422);
    let d5421 = 3. * brouwer0.n.powi(2) / brouwer0.a.powi(5) * (c54s54 * f542 * g521);
    let d5433 = 3. * brouwer0.n.powi(2) / brouwer0.a.powi(5) * (c54s54 * f543 * g533); // Typo in Hoots et al 2004
    let d4410 = 3. * brouwer0.n.powi(2) / brouwer0.a.powi(4) * (c44s44 * f441 * g410);

    // Store resonance parameters
    let half_day_resonance_params = HalfDayResonanceParams {
        d2201: d2201,
        d2211: d2211,
        d3210: d3210,
        d3222: d3222,
        d5220: d5220,
        d5232: d5232,
        d4422: d4422,
        d5421: d5421,
        d5433: d5433,
        d4410: d4410,
    };

    return half_day_resonance_params;
}

/// Initialize the whole day resonance effects of Earth's gravity
///
/// # Arguments
/// * `brouwer0` - The Brouwer mean elements at epoch
///
/// # Returns
/// * `WholeDayResonanceParams` - The whole day resonance parameters
///
/// # Examples
/// ```rust
/// // Define Brouwer mean elements at epoch
/// let brouwer0 = BrouwerMeanElements::default();
///
/// // Initialize whole day resonance effects
/// let whole_day_resonance_params = init_earth_gravity_resonance_wholeday(&brouwer0);
/// ```
///
/// References
/// - [Revisiting Spacetrack Report #3: Rev 3 by Vallado et al](https://celestrak.org/publications/AIAA/2006-6753/AIAA-2006-6753-Rev3.pdf)
/// - [Fundamentals of Astrodynamics and Applications by Vallado et al](https://celestrak.org/software/vallado-sw.php)
/// - [History of Analytical Orbit Modeling in the U.S. Space Surveillance System by Hoots et al](https://arc.aiaa.org/doi/abs/10.2514/1.9161?casa_token=pVowNFT6MOkAAAAA%3A_DFsBbZwGC2QcMWxPhJN2k3suNrcP5YzV7NVBYSvwMxGy19RzX-AvUnyO9JT5Cku0cDYPfpIQm4&journalCode=jgcd)
pub fn init_earth_gravity_resonance_wholeday(brouwer0: &BrouwerMeanElements) -> WholeDayResonanceParams {
    // Precompute common quantities
    let cos_i0 = brouwer0.i.cos();
    let sin_i0 = brouwer0.i.sin();

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
    let g200 = 1. - (5./2.) * brouwer0.e.powi(2) + (13. / 16.) * brouwer0.e.powi(4);
    let g310 = 1. + 2. * brouwer0.e.powi(2);
    let g300 = 1. - 6. * brouwer0.e.powi(2) + (423. / 64.) * brouwer0.e.powi(4);

    // Calculate coefficients of the resonance terms
    let delta1 = (3. * brouwer0.n.powi(2) / brouwer0.a.powi(3)) * f311 * g310 * q31;
    let delta2 = (6. * brouwer0.n.powi(2) / brouwer0.a.powi(2)) * f220 * g200 * q22;
    let delta3 = (9. * brouwer0.n.powi(2) / brouwer0.a.powi(3)) * f330 * g300 * q33;

    // Store resonance parameters
    let whole_day_resonance_params = WholeDayResonanceParams {
        lam31: lam31,
        lam22: lam22,
        lam33: lam33,
        delta1: delta1,
        delta2: delta2,
        delta3: delta3,
    };

    return whole_day_resonance_params;
}

///
/// References
/// - [Revisiting Spacetrack Report #3: Rev 3 by Vallado et al](https://celestrak.org/publications/AIAA/2006-6753/AIAA-2006-6753-Rev3.pdf)
/// - [Fundamentals of Astrodynamics and Applications by Vallado et al](https://celestrak.org/software/vallado-sw.php)
/// - [History of Analytical Orbit Modeling in the U.S. Space Surveillance System by Hoots et al](https://arc.aiaa.org/doi/abs/10.2514/1.9161?casa_token=pVowNFT6MOkAAAAA%3A_DFsBbZwGC2QcMWxPhJN2k3suNrcP5YzV7NVBYSvwMxGy19RzX-AvUnyO9JT5Cku0cDYPfpIQm4&journalCode=jgcd)
// pub fn sgp4_prop(sgp4: Sgp4, datetime: DateTime) -> {
//     // Convert datetime to Julian day format
//     let (jd_prop, jdfrac_prop) = utc2jday(datetime).unwrap();

//     // Get minutes since epoch
//     let delta_t = (jd_prop + jdfrac_prop - (sgp4.jd0 + sgp4.jdfrac0)) * 1440.;

//     // Account for Earth zonal gravity and partial atmospheric drag effects
//     let m_df = m0 + n0 * delta_t + m_dot * delta_t;
//     let omega_df = omega0 + omega_dot * delta_t;
//     let raan_df = raan0 + raan_dot * delta_t;
//     let delta_omega = bstar * c3 * omega0.cos() * delta_t
// }

// ----------
// Unit Tests
// ----------
#[cfg(test)]
mod tests {
    use super::*;
}