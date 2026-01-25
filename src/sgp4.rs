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
}

// -----
// Enums
// -----

/// Errors that can occur during date conversion operations
#[derive(Debug, Clone, PartialEq)]
pub enum DateError {
    /// The provided date is before October 10th, 1582 (Gregorian calendar adoption)
    DateTooEarly,
    /// The day of year is invalid (less than 1 or greater than 365/366)
    InvalidDayOfYear,
}

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

/// Convert a datetime in UTC to Julian date (JD) format.
///
/// The Julian date is a continuous count of days since 4713-01-01 12:00:00 BCE (Julian calendar).
/// This function converts a UTC datetime to Julian date format, valid for any date after
/// October 10th, 1582 (Gregorian calendar adoption).
///
/// # Arguments
/// * `year` - The year (must be >= 1582, or >= 1582 with month >= 10, or >= 1582 with month == 10 and day >= 10)
/// * `mon` - The month (1-12)
/// * `day` - The day (1-31) 
/// * `hr` - The hour (0-23)
/// * `minute` - The minute (0-59)
/// * `sec` - The seconds with fractional component (0.0-59.999...)
///
/// # Returns
/// * `Result<(f64, f64), DateError>` - On success, returns a tuple containing:
///   - `jd` - The Julian day (integer part), a continuous count of days since 4713-01-01 12:00:00 BCE
///   - `jdfrac` - The Julian day fraction (fractional part), a continuous count of days since 4713-01-01 12:00:00 BCE
///   On error, returns `DateError::DateTooEarly` if the date is before October 10th, 1582
///
/// # Examples
/// ```rust
/// use Rusty_SGP4::sgp4::utc2jday;
///
/// let (jd, jdfrac) = utc2jday(2024, 1, 1, 12, 0, 0.0)?;
/// let jd_total = jd + jdfrac;
/// ```
///
/// # Errors
/// Returns `DateError::DateTooEarly` if the provided date is before October 10th, 1582.
///
/// # References
/// - [Fundamentals of Astrodynamics and Applications](https://celestrak.org/software/vallado-sw.php)
/// - [Satellite Orbits](https://link.springer.com/book/10.1007/978-3-642-58351-3)
pub fn utc2jday(year: i32, mon: i32, day: i32, hr: i32, minute: i32, sec: f64) -> Result<(f64, f64), DateError> {
    // Calculate the MJD
    let (mjd, mjdfrac) = utc2mjday(year, mon, day, hr, minute, sec)?;

    // Modify MJD to be JD
    let mut jd: f64 = mjd + 2400000.5;
    let mut jdfrac: f64 = mjdfrac;

    // Make JD whole
    if jdfrac >= 1.0 || jdfrac < 0.0 {
        jd = jd + jdfrac.floor();
        jdfrac = jdfrac - jdfrac.floor();
    }
    
    return Ok((jd, jdfrac));
}

/// Convert a datetime in UTC to Modified Julian date (MJD) format.
///
/// The Modified Julian date is a continuous count of days since 1858-11-17 00:00:00 CE.
/// MJD is related to Julian Date (JD) by: MJD = JD - 2400000.5
/// This function is valid for any date after October 10th, 1582 (Gregorian calendar adoption).
///
/// # Arguments
/// * `year` - The year (must be >= 1582, or >= 1582 with month >= 10, or >= 1582 with month == 10 and day >= 10)
/// * `mon` - The month (1-12)
/// * `day` - The day (1-31) 
/// * `hr` - The hour (0-23)
/// * `minute` - The minute (0-59)
/// * `sec` - The seconds with fractional component (0.0-59.999...)
///
/// # Returns
/// * `Result<(f64, f64), DateError>` - On success, returns a tuple containing:
///   - `mjd` - The Modified Julian day (integer part), a continuous count of days since 1858-11-17 00:00:00 CE
///   - `mjdfrac` - The Modified Julian day fraction (fractional part), a continuous count of days since 1858-11-17 00:00:00 CE
///   On error, returns `DateError::DateTooEarly` if the date is before October 10th, 1582
///
/// # Examples
/// ```rust
/// use Rusty_SGP4::sgp4::utc2mjday;
///
/// let (mjd, mjdfrac) = utc2mjday(2024, 1, 1, 12, 0, 0.0)?;
/// let mjd_total = mjd + mjdfrac;
/// ```
///
/// # Errors
/// Returns `DateError::DateTooEarly` if the provided date is before October 10th, 1582.
///
/// # References
/// - [Fundamentals of Astrodynamics and Applications](https://celestrak.org/software/vallado-sw.php)
/// - [Satellite Orbits](https://link.springer.com/book/10.1007/978-3-642-58351-3)
pub fn utc2mjday(year: i32, mon: i32, day: i32, hr: i32, minute: i32, sec: f64) -> Result<(f64, f64), DateError> {
    // Verify date is after Oct 10th, 1582
    if year < 1582 || (year == 1582 && mon < 10) || (year == 1582 && mon == 10 && day < 10) {
        return Err(DateError::DateTooEarly);
    }
    
    // Cast inputs as f64
    let year = year as f64;
    let mon = mon as f64;
    let day = day as f64;
    let hr = hr as f64;
    let minute = minute as f64;

    // Modify month and year to account for leap years, start year in March instead of January
    let mut year_leap: f64;
    let mut mon_leap: f64;
    if mon <= 2. {
        year_leap = year - 1.;
        mon_leap = mon + 12.;
    } else {
        year_leap = year;
        mon_leap = mon;
    }

    // Account for leap days with B auxilary quantity
    let b_leap: f64 = (year_leap / 400.).floor() - (year_leap / 100.).floor() + (year_leap / 4.).floor();

    // Calculate the modified Julian date
    let mut mjd = 365. * year_leap - 679004. + b_leap + (30.6001 * (mon_leap + 1.)).floor() + day;
    let mut mjdfrac = (sec + minute * 60. + hr * 3600.) / 86400.;

    // Validate mjdfrac
    if mjdfrac >= 1.0 || mjdfrac < 0.0 {
        mjd = mjd + mjdfrac.floor();
        mjdfrac = mjdfrac - mjdfrac.floor();
    }
    
    return Ok((mjd, mjdfrac));
}

/// Convert a year and day of year to a UTC datetime
///
/// Converts a year and day of year (with fractional day) into a full UTC datetime.
/// The day of year is 1-based (1 = January 1st, 365/366 = December 31st).
///
/// # Arguments
/// * `year` - The year
/// * `dayofyr` - The day of year with fractional component (e.g., 123.5 = day 123 at 12:00:00 UTC)
///
/// # Returns
/// * `Result<(i32, i32, i32, i32, i32, f64), DateError>` - On success, returns a tuple containing:
///   - Year
///   - Month (1-12)
///   - Day (1-31)
///   - Hour (0-23)
///   - Minute (0-59)
///   - Second with fractional component (0.0-59.999...)
///   On error, returns:
///   - `DateError::InvalidDayOfYear` if the day of year is less than 1 or exceeds the number of days in the year
///
/// # Examples
/// ```rust
/// use Rusty_SGP4::sgp4::dayofyr2utc;
///
/// // Day 123.5 of 2024 = May 2nd, 2024 at 12:00:00
/// let (year, month, day, hour, minute, second) = dayofyr2utc(2024, 123.5)?;
/// ```
pub fn dayofyr2utc(year: i32, dayofyr: f64) -> Result<(i32, i32, i32, i32, i32, f64), DateError> {
    // Validate day of year is positive
    if dayofyr < 1.0 {
        return Err(DateError::InvalidDayOfYear);
    }
    
    // Check for leap year
    let is_leap = (year % 4 == 0 && year % 100 != 0) || (year % 400 == 0);
    
    // Days per month (non-leap year)
    let days_per_month = [31, 28, 31, 30, 31, 30, 31, 31, 30, 31, 30, 31];
    
    // Adjust February for leap year
    let max_days = if is_leap { 366 } else { 365 };
    
    // Extract integer and fractional parts
    let day_int = dayofyr.floor() as i32;
    
    // Validate day of year doesn't exceed days in year (check integer part)
    if day_int > max_days {
        return Err(DateError::InvalidDayOfYear);
    }
    let day_frac = dayofyr - day_int as f64;
    
    // Find which month the day falls in and calculate day of month
    let mut day_count = 0;
    let mut month = 1;
    let mut day = 1;
    
    for (idx, &days_in_month) in days_per_month.iter().enumerate() {
        let days_this_month = if idx == 1 && is_leap {
            29  // February in leap year
        } else {
            days_in_month
        };
        
        if day_int <= day_count + days_this_month {
            month = (idx + 1) as i32;
            day = (day_int - day_count) as i32;
            break;
        }
        day_count += days_this_month;
    }
    
    // Convert fractional day to hours, minutes, seconds
    let total_seconds = day_frac * 86400.0;
    let hour = (total_seconds / 3600.0).floor() as i32;
    let remaining_seconds = total_seconds - (hour as f64 * 3600.0);
    let minute = (remaining_seconds / 60.0).floor() as i32;
    let second = remaining_seconds - (minute as f64 * 60.0);

    // Protect against rounding errors and handle overflow
    let mut final_second = second;
    let mut final_minute = minute;
    let mut final_hour = hour;
    let mut final_day = day;
    let mut final_month = month;
    let mut final_year = year;
    
    // Handle second overflow
    if final_second >= 60.0 {
        final_second -= 60.0;
        final_minute += 1;
    }
    
    // Handle minute overflow
    if final_minute >= 60 {
        final_minute -= 60;
        final_hour += 1;
    }
    
    // Handle hour overflow
    if final_hour >= 24 {
        final_hour -= 24;
        final_day += 1;
    }
    
    // Handle day overflow (check if day exceeds days in current month)
    let days_in_current_month = if final_month == 2 && is_leap {
        29  // February in leap year
    } else {
        days_per_month[(final_month - 1) as usize]
    };
    
    if final_day > days_in_current_month {
        final_day -= days_in_current_month;
        final_month += 1;
    }
    
    // Handle month overflow
    if final_month > 12 {
        final_month -= 12;
        final_year += 1;
    }
    
    Ok((final_year, final_month, final_day, final_hour, final_minute, final_second))
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
    let n0 = tle.mean_motion;
    let e0 = tle.eccentricity;
    let omega0 = deg2rad(tle.argument_of_perigee);
    let raan0 = deg2rad(tle.right_ascension_of_ascending_node);
    let m0 = deg2rad(tle.mean_anomaly);
    let bstar = tle.bstar;

    // Extract TLE epoch in Julian day format
    let (tle_year, tle_month, tle_day, tle_hour, tle_minute, tle_second) = dayofyr2utc(tle.epoch_year, tle.epoch_day).unwrap();
    let (tle_jd, tle_jdfrac) = utc2jday(tle_year, tle_month, tle_day, tle_hour, tle_minute, tle_second).unwrap();

    // Recover the Brouer mean motion from the Kozai mean motion (mean motion in TLE)
    let ke = wgs_sgp4.mu.sqrt() / wgs_sgp4.r_earth_eq.powf(1.5) * 60.; // [(Earth radii)^1.5 / min]
    let k2 = 0.5 * wgs_sgp4.j2; // [(Earth radii)^2]
    let a1 = (ke / n0).powf(2./3.);
    let delta1 = (3./2.) * (k2 / a1.powf(2.)) * (3. * i0.cos().powf(2.) - 1.) / (1. - e0.powf(2.)).powf(3./2.);
    let a2 = a1 * (1. - (1./3.) * delta1 - delta1.powf(2.) - (134./81.) * delta1.powf(3.));
    let delta0 = (3./2.) * (k2 / a2.powf(2.)) * (3. * i0.cos().powf(2.) - 1.) / (1. - e0.powf(2.)).powf(3./2.);
    let n0_brouwer = n0 / (1. + delta0);
    let a0 = (ke / n0_brouwer).powf(2./3.);

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
    let c2 = (q0 - s).powf(4.) * zeta.powf(4.) * n0_brouwer * (1. - eta.powf(2.)).powf(-7./2.) * (a0 * (1. + (3./2.) * eta.powf(2.) + 4. * e0 * eta + e0 * eta.powf(3.)) + (3./2.) * (k2 * zeta / (1. - eta.powf(2.))) * (-(1./2.) + (3./2.) * theta.powf(2.)) * (8. + 24. * eta.powf(2.) + 3. * eta.powf(4.)));
    let c1 = bstar * c2;
    let c3 = ((q0 - s).powf(4.) * zeta.powf(5.) * a30 * n0_brouwer * wgs_sgp4.r_earth_eq * i0.sin()) / (k2 * e0);
    let c4 = 2. * n0_brouwer * (q0 - s).powf(4.) * zeta.powf(4.) * a0 * beta0.powf(2.) * (1. - eta.powf(2.)).powf(-7./2.) * ((2. * eta * (1. + e0*eta) + 0.5 * e0 + 0.5 * eta.powf(3.)) - (2. * k2 * zeta / (a0 * (1. - eta.powf(2.)))) * (3. * (1. - 3. * theta.powf(2.)) * (1. + 3./2. * eta.powf(2.) - 2. * e0 * eta - 0.5 * e0 * eta.powf(3.))) + 3./4. * (1. - theta.powf(2.)) * (2. * eta.powf(2.) - e0 * eta - e0 * eta.powf(3.)) * (2. * omega0).cos());
    let c5 = 2. * (q0 - s).powf(4.) * zeta.powf(4.) * a0 * beta0.powf(2.) * (1. - eta.powf(2.)).powf(-7./2.) * (1. + 11./4. * eta * (eta + e0) + e0 * eta.powf(3.));
    let d2 = 4. * a0 * zeta * c1.powf(2.);
    let d3 = 4./3. * a0 * zeta.powf(2.) * (17. * a0 + s) * c1.powf(3.);
    let d4 = 2./3. * a0.powf(2.) * zeta.powf(3.) * (221. * a0 + 31. * s) * c1.powf(4.);

    // Earth zonal harmonics
    let k4 = -3./8. * wgs_sgp4.j4 * wgs_sgp4.r_earth_eq.powf(4.);
    let mdot = ((3. * k2 * (-1. + 3. * theta.powf(2.)) / (2. * a0.powf(2.) * beta0.powf(3.))) + (3. * k2.powf(2.) * (13. - 78. * theta.powf(2.) + 137. * theta.powf(4.)) / (16. * a0.powf(4.) * beta0.powf(7.)))) * n0_brouwer;
    let omegadot = (-3. * k2 * (1. - 5. * theta.powf(2.)) / (2. * a0.powf(2.) * beta0.powf(4.)) + 3. * k2.powf(2.) * (7. - 114. * theta.powf(2.) + 395. * theta.powf(4.)) / (16. * a0.powf(4.) * beta0.powf(8.)) + 5. * k4 * (3. - 36. * theta.powf(2.) + 49. * theta.powf(4.)) / (4. * a0.powf(4.) * beta0.powf(8.))) * n0_brouwer;
    let raandot = (-3. * k2 * theta / (a0.powf(2.) * beta0.powf(4.)) + 3. * k2.powf(2.) * (4. * theta - 19. * theta.powf(3.)) / (2. * a0.powf(4.) * beta0.powf(8.)) + 5. * k4 * theta * (3. - 7. * theta.powf(2.)) / (2. * a0.powf(4.) * beta0.powf(8.))) * n0_brouwer;

    // Lunar and solar gravity effects

    // Earth gravity resonance effects

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
pub fn init_lunar_solar_effects(tle_jd: f64, tle_jdfrac: f64) -> i32 {
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
    fn test_utc2jday() {
        // Make a test date in 20th century
        let year1 = 1959;
        let mon1 = 3;
        let day1 = 25;
        let hr1 = 12;
        let minute1 = 34;
        let sec1 = 49.123;
        
        let (jd1, jdfrac1) = utc2jday(year1, mon1, day1, hr1, minute1, sec1).unwrap();
        let jd1_total = jd1 + jdfrac1;
        let jd1_expect = 2436653.024179664440453;
        assert!((jd1_total - jd1_expect).abs() < 1e-8,"Julian Date Test failed: expected {}, got {}", jd1_expect, jd1_total);

        // Make a test date in 21st century
        let year2 = 2026;
        let mon2 = 1;
        let day2 = 23;
        let hr2 = 0;
        let minute2 = 10;
        let sec2 = 32.999;

        let (jd2, jdfrac2) = utc2jday(year2, mon2, day2, hr2, minute2, sec2).unwrap();
        let jd2_total = jd2 + jdfrac2;
        let jd2_expect = 2461063.507326377090067;
        assert!((jd2_total - jd2_expect).abs() < 1e-8,"Julian Date Test failed: expected {}, got {}", jd2_expect, jd2_total);

        // Make a test date around year 2000
        let year3 = 2000;
        let mon3 = 1;
        let day3 = 1;
        let hr3 = 0;
        let minute3 = 0;
        let sec3 = 0.0;

        let (jd3, jdfrac3) = utc2jday(year3, mon3, day3, hr3, minute3, sec3).unwrap();
        let jd3_total = jd3 + jdfrac3;
        let jd3_expect = 2451544.5;
        assert!((jd3_total - jd3_expect).abs() < 1e-8,"Julian Date Test failed: expected {}, got {}", jd3_expect, jd3_total);

        // Test date too early (before Oct 10, 1582)
        let result = utc2jday(1582, 10, 9, 12, 0, 0.0);
        assert!(result.is_err(), "Should return error for date before Oct 10, 1582");
        assert_eq!(result.unwrap_err(), DateError::DateTooEarly);
    }

    #[test]
    fn test_utc2mjday() {
        // Make a test date in 20th century
        let year1 = 1997;
        let mon1 = 4;
        let day1 = 2;
        let hr1 = 16;
        let minute1 = 12;
        let sec1 = 35.505;
        
        let (mjd1, mjdfrac1) = utc2mjday(year1, mon1, day1, hr1, minute1, sec1).unwrap();
        let mjd1_total = mjd1 + mjdfrac1;
        let mjd1_expect = 50540.675410937503329;
        assert!((mjd1_total - mjd1_expect).abs() < 1e-8,"Modified Julian Date Test failed: expected {}, got {}", mjd1_expect, mjd1_total);

        // Make a test date in 21st century
        let year2 = 2013;
        let mon2 = 8;
        let day2 = 12;
        let hr2 = 2;
        let minute2 = 49;
        let sec2 = 57.623;

        let (mjd2, mjdfrac2) = utc2mjday(year2, mon2, day2, hr2, minute2, sec2).unwrap();
        let mjd2_total = mjd2 + mjdfrac2;
        let mjd2_expect = 56516.118028043973027;
        assert!((mjd2_total - mjd2_expect).abs() < 1e-8,"Modified Julian Date Test failed: expected {}, got {}", mjd2_expect, mjd2_total);

        // Make a test date around year 2000
        let year3 = 2000;
        let mon3 = 1;
        let day3 = 1;
        let hr3 = 0;
        let minute3 = 0;
        let sec3 = 0.0;

        let (mjd3, mjdfrac3) = utc2mjday(year3, mon3, day3, hr3, minute3, sec3).unwrap();
        let mjd3_total = mjd3 + mjdfrac3;
        let mjd3_expect = 51544.0;
        assert!((mjd3_total - mjd3_expect).abs() < 1e-8,"Modified Julian Date Test failed: expected {}, got {}", mjd3_expect, mjd3_total);

        // Test date too early (before Oct 10, 1582)
        let result = utc2mjday(1582, 10, 9, 12, 0, 0.0);
        assert!(result.is_err(), "Should return error for date before Oct 10, 1582");
        assert_eq!(result.unwrap_err(), DateError::DateTooEarly);
    }

    #[test]
    fn test_dayofyr_rounding() {
        // Test date in 20th century - Day 100.5 of 1959 (April 10, 1959 at 12:00:00)
        let (year1, month1, day1, hour1, minute1, second1) = dayofyr2utc(1959, 100.5).unwrap();
        assert_eq!(year1, 1959, "Day 100.5 of 1959: year should be 1959, got {}", year1);
        assert_eq!(month1, 4, "Day 100.5 of 1959: month should be 4 (April), got {}", month1);
        assert_eq!(day1, 10, "Day 100.5 of 1959: day should be 10, got {}", day1);
        assert_eq!(hour1, 12, "Day 100.5 of 1959: hour should be 12, got {}", hour1);
        assert_eq!(minute1, 0, "Day 100.5 of 1959: minute should be 0, got {}", minute1);
        assert!((second1 - 0.0).abs() < 1e-6, "Day 100.5 of 1959: second should be 0.0, got {}", second1);

        // Test date in 21st century - Day 200.75 of 2024 (July 18, 2024 at 18:00:00)
        let (year2, month2, day2, hour2, minute2, second2) = dayofyr2utc(2024, 200.75).unwrap();
        assert_eq!(year2, 2024, "Day 200.75 of 2024: year should be 2024, got {}", year2);
        assert_eq!(month2, 7, "Day 200.75 of 2024: month should be 7 (July), got {}", month2);
        assert_eq!(day2, 18, "Day 200.75 of 2024: day should be 18, got {}", day2);
        assert_eq!(hour2, 18, "Day 200.75 of 2024: hour should be 18, got {}", hour2);
        assert_eq!(minute2, 0, "Day 200.75 of 2024: minute should be 0, got {}", minute2);
        assert!((second2 - 0.0).abs() < 1e-6, "Day 200.75 of 2024: second should be 0.0, got {}", second2);

        // Test year rollover - Day 365.9999999999 of 2023 (very close to midnight of 2024)
        let (year3, month3, day3, hour3, minute3, second3) = dayofyr2utc(2023, 365.9999999999).unwrap();
        assert_eq!(year3, 2023, "Day 365.9999999999 of 2023: year should be 2023, got {}", year3);
        assert_eq!(month3, 12, "Day 365.9999999999 of 2023: month should be 12 (December), got {}", month3);
        assert_eq!(day3, 31, "Day 365.9999999999 of 2023: day should be 31, got {}", day3);
        assert_eq!(hour3, 23, "Day 365.9999999999 of 2023: hour should be 23, got {}", hour3);
        assert_eq!(minute3, 59, "Day 365.9999999999 of 2023: minute should be 59, got {}", minute3);
        assert!(second3 >= 59.0 && second3 < 60.0, "Day 365.9999999999 of 2023: second should be between 59.0 and 60.0, got {}", second3);

        // Test leap year day of year - Day 366 of 2024 (December 31, 2024)
        let (year4, month4, day4, hour4, minute4, second4) = dayofyr2utc(2024, 366.0).unwrap();
        assert_eq!(year4, 2024, "Day 366 of 2024: year should be 2024, got {}", year4);
        assert_eq!(month4, 12, "Day 366 of 2024: month should be 12 (December), got {}", month4);
        assert_eq!(day4, 31, "Day 366 of 2024: day should be 31, got {}", day4);
        assert_eq!(hour4, 0, "Day 366 of 2024: hour should be 0, got {}", hour4);
        assert_eq!(minute4, 0, "Day 366 of 2024: minute should be 0, got {}", minute4);
        assert!((second4 - 0.0).abs() < 1e-6, "Day 366 of 2024: second should be 0.0, got {}", second4);

        // Test leap year with fractional day - Day 60.5 of 2024 (February 29, 2024 at 12:00:00)
        let (year5, month5, day5, hour5, minute5, second5) = dayofyr2utc(2024, 60.5).unwrap();
        assert_eq!(year5, 2024, "Day 60.5 of 2024: year should be 2024, got {}", year5);
        assert_eq!(month5, 2, "Day 60.5 of 2024: month should be 2 (February), got {}", month5);
        assert_eq!(day5, 29, "Day 60.5 of 2024: day should be 29 (leap day), got {}", day5);
        assert_eq!(hour5, 12, "Day 60.5 of 2024: hour should be 12, got {}", hour5);
        assert_eq!(minute5, 0, "Day 60.5 of 2024: minute should be 0, got {}", minute5);
        assert!((second5 - 0.0).abs() < 1e-6, "Day 60.5 of 2024: second should be 0.0, got {}", second5);

        // Test day of year too high - Day 366.1 of non-leap year 2023
        let result = dayofyr2utc(2023, 366.1);
        assert!(result.is_err(), "Day 366.1 of 2023 (non-leap year): should return error for day of year exceeding 365, got Ok({:?})", result);
        assert_eq!(result.unwrap_err(), DateError::InvalidDayOfYear, "Day 366.1 of 2023: error should be InvalidDayOfYear");

        // Test day of year too high - Day 367 of leap year 2024
        let result = dayofyr2utc(2024, 367.0);
        assert!(result.is_err(), "Day 367 of 2024 (leap year): should return error for day of year exceeding 366, got Ok({:?})", result);
        assert_eq!(result.unwrap_err(), DateError::InvalidDayOfYear, "Day 367 of 2024: error should be InvalidDayOfYear");

        // Test day of year too low - Day 0.5
        let result = dayofyr2utc(2024, 0.5);
        assert!(result.is_err(), "Day 0.5 of 2024: should return error for day of year less than 1, got Ok({:?})", result);
        assert_eq!(result.unwrap_err(), DateError::InvalidDayOfYear, "Day 0.5 of 2024: error should be InvalidDayOfYear");

        // Test day of year too low - Day 0.0
        let result = dayofyr2utc(2024, 0.0);
        assert!(result.is_err(), "Day 0.0 of 2024: should return error for day of year equal to 0, got Ok({:?})", result);
        assert_eq!(result.unwrap_err(), DateError::InvalidDayOfYear, "Day 0.0 of 2024: error should be InvalidDayOfYear");
    }
}