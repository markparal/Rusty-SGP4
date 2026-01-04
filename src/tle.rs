// Module for parsing TLEs

// ------------------
// External Libraries
// ------------------
use std::fs;

// ------------------
// Internal Libraries
// ------------------

// -------
// Structs
// -------

/// Two-Line Element (TLE) orbital parameters for an Earth-orbiting satellite.
///
/// This struct represents the parsed contents of a standard NORAD TLE.
///
/// References:
/// - [Celestrak TLE Format](https://celestrak.org/columns/v04n03/#FAQ01)
pub struct Tle {
    /// Common name of the satellite (e.g., "ISS (ZARYA)")
    pub common_name: String,

    /// NORAD satellite catalog number
    pub satellite_catalog_number: i32,

    /// Classification (`U` = Unclassified, `C` = Classified, `S` = Secret)
    pub classification: char,

    /// International designator (launch year, launch number, piece)
    pub international_designator: String,

    /// Epoch year (two-digit TLE year, e.g. 24 → 2024)
    pub epoch_year: i32,

    /// Epoch day of year, including fractional portion
    pub epoch_day: f64,

    /// First time derivative of mean motion \[revs/day^2\]
    pub first_derivative_of_mean_motion: f64,

    /// Second time derivative of mean motion \[revs/day^3\]
    pub second_derivative_of_mean_motion: f64,

    /// B* drag term \[1/Earth radii\]
    pub bstar: f64,

    /// Ephemeris type (always zero)
    pub ephemeris_type: i32,

    /// Element set number
    pub element_set_number: i32,

    /// Orbital inclination \[degrees\]
    pub inclination: f64,

    /// Right ascension of the ascending node (RAAN) \[degrees\]
    pub right_ascension_of_ascending_node: f64,

    /// Orbital eccentricity \[\]
    pub eccentricity: f64,

    /// Argument of perigee \[degrees\]
    pub argument_of_perigee: f64,

    /// Mean anomaly \[degrees\]
    pub mean_anomaly: f64,

    /// Mean motion \[revs/day\]
    pub mean_motion: f64,

    /// Revolution number at epoch \[revs\]
    pub revolution_number_at_epoch: i64,
}

// ---------
// Enums
// ---------

// ---------
// Constants
// ---------

// ---------
// Functions
// ---------

/// Builds a [`Tle`] struct from the lines of a Two-Line Element set.
///
/// Given the two required TLE lines (line 1 and line 2), and an optional
/// name line (line 0), this function parses the input into a [`TLE`] struct.
///
/// # Arguments
/// * `line1` - The first TLE data line (NORAD line 1)
/// * `line2` - The second TLE data line (NORAD line 2)
/// * `line0` - Optional name line (line 0)
///
/// # Panics
/// * If the TLE lines (1 and 2) are of invalid lengths (must be 69 characters)
/// * If the TLE lines (1 and 2) are invalid (checksum fails)
///
/// # Returns
/// * [`Tle`] - Struct containing the parsed TLE data.
///
/// # Examples
/// ```rust
/// // Define the TLE lines
/// let tle_line0 = "ISS (ZARYA)";
/// let tle_line1 = "1 25544U 98067A   08264.51782528 -.00002182 -00100-2 -11606-4 0  2921";
/// let tle_line2 = "2 25544  51.6416 247.4627 0006703 130.5360 325.0288 15.72125391563537";
/// 
/// // Parse the TLE lines into a TLE struct
/// let tle = from_lines(tle_line1, tle_line2, Some(tle_line0));
/// 
/// // Assert the TLE struct is correct
/// assert_eq!(tle.satellite_catalog_number, 25544);
/// ```
///
/// # References
/// - [Celestrak TLE Format](https://celestrak.org/columns/v04n03/#FAQ01)
pub fn from_lines(line1: &str, line2: &str, line0: Option<&str>) -> Tle {
    // Create mutable TLE struct
    let mut tle = Tle {
        common_name: String::new(),
        satellite_catalog_number: 0,
        classification: '0',
        international_designator: String::new(),
        epoch_year: 0,
        epoch_day: 0.0,
        first_derivative_of_mean_motion: 0.0,
        second_derivative_of_mean_motion: 0.0,
        bstar: 0.0,
        ephemeris_type: 0,
        element_set_number: 0,
        inclination: 0.0,
        right_ascension_of_ascending_node: 0.0,
        eccentricity: 0.0,
        argument_of_perigee: 0.0,
        mean_anomaly: 0.0,
        mean_motion: 0.0,
        revolution_number_at_epoch: 0,
    };

    // Validate the TLE checksum
    if !tle_checksum(line1) || !tle_checksum(line2) {
        println!("TLE lines are invalid");
        return tle;
    }

    // Extract the common name of the satellite from line 0
    if let Some(name_line) = line0 {
        if name_line.len() < 1 || name_line.len() > 24 {
            println!("TLE string for Line 0 is invalid");
        } else {
            tle.common_name = name_line.to_string();
        }
    }
    
    // Parse through line 1 and populate TLE struct
    if line1.len() < 69 || line1.len() > 69 {
        println!("TLE string is invalid");
    } else {
        // Line 1
        // Satellite catalog number
        tle.satellite_catalog_number = line1[2..7].trim().parse::<i32>().unwrap();

        // Classification
        tle.classification = line1[7..8].trim().parse::<char>().unwrap();

        // International designator
        tle.international_designator = line1[9..17].trim().to_string();

        // Epoch year (last two numbers)
        tle.epoch_year = line1[18..20].trim().parse::<i32>().unwrap();

        // Epoch day of year
        tle.epoch_day = line1[20..32].trim().parse::<f64>().unwrap();

        // 1st derivative of mean motion [revs/day^2]
        tle.first_derivative_of_mean_motion = line1[33..43].trim().parse::<f64>().unwrap() * 2.0;

        // 2nd derivative of mean motion [revs/days^3]
        // Account for - in 2nd derivative of mean motion
        if line1[44..45].parse::<char>().unwrap() == '-' {
            tle.second_derivative_of_mean_motion = format!("-0.{}", line1[45..50].trim()).parse::<f64>().unwrap() * 10.0_f64.powi(line1[50..52].parse::<i32>().unwrap()) * 6.0_f64;
        } else {
            tle.second_derivative_of_mean_motion = format!("0.{}", line1[45..50].trim()).parse::<f64>().unwrap() * 10.0_f64.powi(line1[50..52].parse::<i32>().unwrap()) * 6.0_f64;
        }

        // B* [1/Earth Radii]
        // Account for - in B* term
        if line1[53..54].parse::<char>().unwrap() == '-' {
            tle.bstar = format!("-0.{}", line1[54..59].trim()).parse::<f64>().unwrap() * 10.0_f64.powi(line1[59..61].parse::<i32>().unwrap());
        } else {
            tle.bstar = format!("0.{}", line1[54..59].trim()).parse::<f64>().unwrap() * 10.0_f64.powi(line1[59..61].parse::<i32>().unwrap());
        }

        // Ephemeris type
        tle.ephemeris_type = line1[62..63].parse::<i32>().unwrap();

        // Element set number
        tle.element_set_number = line1[64..68].trim().parse::<i32>().unwrap();
    }

    // Parse through line 2 and populate TLE struct
    if line2.len() < 69 || line2.len() > 69 {
        println!("TLE string is invalid");
    } else {
        // Line 2
        // Inclination [degs]
        tle.inclination = line2[8..16].trim().parse::<f64>().unwrap();

        // Right ascension of ascending node [degs]
        tle.right_ascension_of_ascending_node = line2[17..25].trim().parse::<f64>().unwrap();

        // Eccentricity
        tle.eccentricity = format!("0.{}", line2[26..33].trim()).parse::<f64>().unwrap();

        // Argument of perigee [degs]
        tle.argument_of_perigee = line2[34..42].trim().parse::<f64>().unwrap();

        // Mean anomaly [degs]
        tle.mean_anomaly = line2[43..51].trim().parse::<f64>().unwrap();

        // Mean motion [revs/day]
        tle.mean_motion = line2[52..63].trim().parse::<f64>().unwrap();

        // Revolution number at epoch
        tle.revolution_number_at_epoch = line2[63..68].trim().parse::<i64>().unwrap();
    }

    return tle;
}

/// Builds a vector of [`Tle`] structs from a string containing Two-Line Element sets.
///
/// This function parses a string containing one or more TLEs in either
/// 2-line or 3-line (name + 2 lines) format and returns all successfully
/// parsed entries.
///
/// # Arguments
/// * `tle_string` - A string containing one or more Two-Line Element sets
///
/// # Returns
/// * `Vec<Tle>` - A vector containing all successfully parsed TLEs
///
/// # Examples
/// ```rust
/// // Define the TLE string
/// let tle_string = "ISS (ZARYA)\n1 25544U 98067A   08264.51782528 -.00002182 -00100-2 -11606-4 0  2921\n2 25544  51.6416 247.4627 0006703 130.5360 325.0288 15.72125391563537";
/// 
/// // Parse the TLE string into a TLE struct
/// let tles = from_string(tle_string);
/// let tle = &tles[0];
/// 
/// // Assert the TLE struct is correct
/// assert_eq!(tle.satellite_catalog_number, 25544);
/// ```
///
/// # References
/// - [Celestrak TLE Format](https://celestrak.org/columns/v04n03/#FAQ01)
pub fn from_string(tle_string: &str) -> Vec<Tle> {
    // Parse the string into lines, removing spaces
    let lines: Vec<&str> = tle_string
        .lines()
        .map(str::trim)
        .filter(|l| !l.is_empty())
        .collect();

    // Create the tles vector
    let mut tles = Vec::new();
    let mut i = 0;

    // Iterate through the lines building TLE structs when possible
    while i < lines.len() {
        println!("{}", lines[i]);
        // Find TLEs within the string, either 2 or 3 line entries
        if lines[i].starts_with('1') {
            // This is likely a 2 line entry, check length
            if i + 1 >= lines.len() {
                break;
            }
            // Check that next line starts with '2'
            if lines[i + 1].starts_with('2') {
                let tle = from_lines(lines[i], lines[i + 1], None);
                tles.push(tle);
                i += 2;
            } else {
                i += 1;
            }
        } else {
            // This is likely a 3 line entry, check length
            if i + 2 >= lines.len() {
                break;
            }
            // Check that next line 2 lines starts with '1' and '2'
            if lines[i + 1].starts_with('1') && lines[i + 2].starts_with('2') {
                println!("{}\n{}\n{}", lines[i], lines[i+1], lines[i+2]);
                let tle = from_lines(lines[i + 1], lines[i + 2], Some(lines[i]));
                tles.push(tle);
                i += 3;
            } else {
                i += 1;
            }
        }
    }
    // Return vector of TLEs
    return tles;
}

/// Builds a vector of [`Tle`] structs from a file containing Two-Line Element sets.
///
/// This function parses a file containing one or more TLEs in either
/// 2-line or 3-line (name + 2 lines) format and returns all successfully
/// parsed entries.
///
/// # Arguments
/// * `file_path` - A path to a file containing one or more Two-Line Element sets
///
/// # Returns
/// * `Vec<Tle>` - A vector containing all successfully parsed TLEs.
///
/// # Examples
/// ```rust
/// // Define the TLE file path
/// let tle_file_path = "assets/test.tle";
/// 
/// // Parse the TLE file into a TLE struct
/// let tles = from_file(tle_file_path);
/// let tle = &tles[12];
/// 
/// // Assert the TLE structs are correct
/// assert_eq!(iss_tle.satellite_catalog_number, 25544);
/// ```
///
/// # References
/// - [Celestrak TLE Format](https://celestrak.org/columns/v04n03/#FAQ01)
pub fn from_file(file_path: &str) -> Vec<Tle> {
    // Open the TLE file
    let tle_string = fs::read_to_string(file_path)
        .expect("Cannot read TLE file");
    
    // Parse tle string into a vector of TLEs
    let tles = from_string(&tle_string);

    // Return the vector of TLEs
    return tles;
}

/// Calculate the checksum of the TLE line.
///
/// Given a TLE line, calculate the checksum of that line. Follow the following rules: 
/// - Ignore alpha characters
/// - Sum digits 0-9 as integer values
/// - '-' is treated as 1
/// - Return checksum % 10
///
/// # Arguments
/// * `line` - The TLE line to calculate the checksum of
///
/// # Panics
/// * If the TLE line is invalid (must be 69 characters)
///
/// # Returns
/// * `checksum` - The checksum of the TLE line (integer 0-9)
///
/// # Examples
/// ```rust
/// // Define the TLE line
/// let tle_line1 = "1 25544U 98067A   08264.51782528 -.00002182 -00100-2 -11606-4 0  2921";
/// 
/// // Calculate the checksum of the TLE line
/// let checksum = calc_checksum(tle_line1);
/// 
/// // Assert the checksum is correct
/// assert_eq!(checksum, 1);
/// ```
pub fn calc_checksum(line: &str) -> i32 {
    // Initialize checksum to 0
    let mut checksum = 0;

    // Loop through the line and calculate the checksum
    for c in line.chars().take(68) {
        match c {
            '0'..='9' => checksum += (c as u8 - b'0') as i32,
            '-' => checksum += 1,
            _ => {}
        }
    }

    // Calculate the checksum
    checksum = checksum % 10;

    // Return the checksum
    return checksum;
}

/// Check if the TLE line has been corrupted by running a checksum test.
///
/// Given a TLE line, check if the checksum of that line is valid.
///
/// # Arguments
/// * `line` - The TLE line to check the checksum of
///
/// # Returns
/// * `bool` - True if the checksum of the line is valid, false if otherwise
///
/// # Examples
/// ```rust
/// // Define the TLE line
/// let tle_line1 = "1 25544U 98067A   08264.51782528 -.00002182 -00100-2 -11606-4 0  2921";
/// 
/// // Calculate the checksum of the TLE line
/// let checksum = tle_checksum(tle_line1);
/// 
/// // Assert the checksum is correct
/// assert_eq!(checksum, true);
/// ```
pub fn tle_checksum(line: &str) -> bool {
    // Calculate the checksum of the line
    let checksum = calc_checksum(line);

    // Compare the checksum to the last character of the line
    if checksum == line[68..69].parse::<i32>().unwrap() {
        return true;
    } else {
        return false;
    }
}

// ----------
// Unit Tests
// ----------

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_checksum_calculation() {
        // Define the TLE line
        let tle_line1 = "1 25544U 98067A   08264.51782528 -.00002182 -00100-2 -11606-4 0  2921";

        // Calculate the checksum of the TLE line
        let checksum = calc_checksum(tle_line1);

        // Assert the checksum is correct
        assert_eq!(checksum, 1);
    }

    #[test]
    fn test_checksum_comparison() {
        // Define the TLE line
        let tle_line1 = "1 25544U 98067A   08264.51782528 -.00002182 -00100-2 -11606-4 0  2921";
        let tle_line2 = "1 25544U 98067A   08264.51782528 -.00002182 -00100-2 -11606-4 0  2922";

        // Calculate the checksum of the TLE line
        let checksum = tle_checksum(tle_line1);
        let checksum2 = tle_checksum(tle_line2);

        // Assert the checksum is correct
        assert_eq!(checksum, true);
        assert_eq!(checksum2, false);
    }

    #[test]
    fn test_tle_parsing_from_lines() {
        // Define the TLE lines
        let tle_line0 = "ISS (ZARYA)";
        let tle_line1 = "1 25544U 98067A   08264.51782528 -.00002182 -00100-2 -11606-4 0  2921";
        let tle_line2 = "2 25544  51.6416 247.4627 0006703 130.5360 325.0288 15.72125391563537";

        // Parse the TLE lines into a TLE struct
        let tle = from_lines(tle_line1, tle_line2, Some(tle_line0));

        // Assert the TLE struct is correct
        assert_eq!(tle.common_name, "ISS (ZARYA)");
        assert_eq!(tle.satellite_catalog_number, 25544);
        assert_eq!(tle.classification, 'U');
        assert_eq!(tle.international_designator, "98067A");
        assert_eq!(tle.epoch_year, 8);
        assert_eq!(tle.epoch_day, 264.51782528);
        assert_eq!(tle.first_derivative_of_mean_motion, -0.00004364);
        assert!((tle.second_derivative_of_mean_motion + 6.0e-5).abs() < 1e-12);
        assert_eq!(tle.bstar, -0.000011606);
        assert_eq!(tle.ephemeris_type, 0);
        assert_eq!(tle.element_set_number, 292);
        assert_eq!(tle.inclination, 51.6416);
        assert_eq!(tle.right_ascension_of_ascending_node, 247.4627);
        assert_eq!(tle.eccentricity, 0.0006703);
        assert_eq!(tle.argument_of_perigee, 130.536);
        assert_eq!(tle.mean_anomaly, 325.0288);
        assert_eq!(tle.mean_motion, 15.72125391);
        assert_eq!(tle.revolution_number_at_epoch, 56353);
    }

    #[test]
    fn test_tle_parsing_from_string() {
        // Define the TLE string
        let tle_string = "ISS (ZARYA)\n1 25544U 98067A   08264.51782528 -.00002182 -00100-2 -11606-4 0  2921\n2 25544  51.6416 247.4627 0006703 130.5360 325.0288 15.72125391563537";

        // Parse the TLE string into a TLE struct
        let tles = from_string(tle_string);
        let tle = &tles[0];

        // Assert the TLE struct is correct
        assert_eq!(tle.common_name, "ISS (ZARYA)");
        assert_eq!(tle.satellite_catalog_number, 25544);
        assert_eq!(tle.classification, 'U');
        assert_eq!(tle.international_designator, "98067A");
        assert_eq!(tle.epoch_year, 8);
        assert_eq!(tle.epoch_day, 264.51782528);
        assert_eq!(tle.first_derivative_of_mean_motion, -0.00004364);
        assert!((tle.second_derivative_of_mean_motion + 6.0e-5).abs() < 1e-12);
        assert_eq!(tle.bstar, -0.000011606);
        assert_eq!(tle.ephemeris_type, 0);
        assert_eq!(tle.element_set_number, 292);
        assert_eq!(tle.inclination, 51.6416);
        assert_eq!(tle.right_ascension_of_ascending_node, 247.4627);
        assert_eq!(tle.eccentricity, 0.0006703);
        assert_eq!(tle.argument_of_perigee, 130.536);
        assert_eq!(tle.mean_anomaly, 325.0288);
        assert_eq!(tle.mean_motion, 15.72125391);
        assert_eq!(tle.revolution_number_at_epoch, 56353);
    }

    #[test]
    fn test_tle_parsing_from_file() {
        // Define the TLE file path
        let tle_file_path = "assets/test.tle";

        // Parse the TLE file into a TLE struct
        let tles = from_file(tle_file_path);
        let tle_count = tles.len();
        let iss_tle = &tles[12];
        let hulianwang_tle = &tles[17];

        // Assert the TLE structs are correct
        assert_eq!(tle_count, 19);

        assert_eq!(iss_tle.common_name, "ISS (ZARYA)");
        assert_eq!(iss_tle.satellite_catalog_number, 25544);
        assert_eq!(iss_tle.classification, 'U');
        assert_eq!(iss_tle.international_designator, "98067A");
        assert_eq!(iss_tle.epoch_year, 8);
        assert_eq!(iss_tle.epoch_day, 264.51782528);
        assert_eq!(iss_tle.first_derivative_of_mean_motion, -0.00004364);
        assert!((iss_tle.second_derivative_of_mean_motion + 6.0e-5).abs() < 1e-12);
        assert_eq!(iss_tle.bstar, -0.000011606);
        assert_eq!(iss_tle.ephemeris_type, 0);
        assert_eq!(iss_tle.element_set_number, 292);
        assert_eq!(iss_tle.inclination, 51.6416);
        assert_eq!(iss_tle.right_ascension_of_ascending_node, 247.4627);
        assert_eq!(iss_tle.eccentricity, 0.0006703);
        assert_eq!(iss_tle.argument_of_perigee, 130.536);
        assert_eq!(iss_tle.mean_anomaly, 325.0288);
        assert_eq!(iss_tle.mean_motion, 15.72125391);
        assert_eq!(iss_tle.revolution_number_at_epoch, 56353);

        assert_eq!(hulianwang_tle.common_name, "HULIANWANG DIGUI-118");
        assert_eq!(hulianwang_tle.satellite_catalog_number, 66957);
        assert_eq!(hulianwang_tle.classification, 'U');
        assert_eq!(hulianwang_tle.international_designator, "25287E");
        assert_eq!(hulianwang_tle.epoch_year, 25);
        assert_eq!(hulianwang_tle.epoch_day, 346.69967332);
        assert_eq!(hulianwang_tle.first_derivative_of_mean_motion, -0.00000302);
        assert_eq!(hulianwang_tle.second_derivative_of_mean_motion, 0.0);
        assert!((hulianwang_tle.bstar + 1.9373e-4).abs() < 1e-12);
        assert_eq!(hulianwang_tle.ephemeris_type, 0);
        assert_eq!(hulianwang_tle.element_set_number, 999);
        assert_eq!(hulianwang_tle.inclination, 86.4945);
        assert_eq!(hulianwang_tle.right_ascension_of_ascending_node, 346.1700);
        assert_eq!(hulianwang_tle.eccentricity, 0.0007219);
        assert_eq!(hulianwang_tle.argument_of_perigee, 190.5502);
        assert_eq!(hulianwang_tle.mean_anomaly, 169.5507);
        assert_eq!(hulianwang_tle.mean_motion, 13.69137019);
        assert_eq!(hulianwang_tle.revolution_number_at_epoch, 52);
    }
}
