pub struct TLE {
    pub common_name: String,
    pub satellite_catalog_number: i32,
    pub classification: char,
    pub international_designator: String,
    pub epoch_year: i32,
    pub epoch_day: f64,
    pub first_derivative_of_mean_motion: f64, // [revs/day^2]
    pub second_derivative_of_mean_motion: f64, // [revs/days^3]
    pub bstar: f64, // [1/Earth Radii]
    pub ephemeris_type: i32,
    pub element_set_number: i32,
    pub inclination: f64, // [degs]
    pub right_ascension_of_ascending_node: f64, // [degs]
    pub eccentricity: f64,
    pub argument_of_perigee: f64, // [degs]
    pub mean_anomaly: f64, // [degs]
    pub mean_motion: f64, // [revs/day]
    pub revolution_number_at_epoch: i64,
}

pub fn from_string(tle_string: &str) -> TLE {
    /// Function to parse the TLE string and return a Tle struct
    
    // Split the TLE string into lines
    let lines = tle_string.split("\n").collect::<Vec<&str>>();

    // TLEs can be 2 or 3 lines, account for both cases
    if lines.len() == 3 {
        // Create TLE struct
        let tle = from_lines(lines[1],lines[2],Some(lines[0]));

        // Return TLE struct
        return tle;

    } else if lines.len() == 2 {
        // Create TLE struct
        let tle = from_lines(lines[0],lines[1],None);

        // Return TLE struct
        return tle;
    
    } else {
        println!("TLE string is invalid");

        // Make an empty TLE struct
        let tle = TLE {
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

        // Return TLE struct
        return tle;
    }
    
}

pub fn from_lines(line1: &str, line2: &str, line0: Option<&str>) -> TLE {
    /// Function to parse the TLE lines into a TLE struct

    // Create mutable TLE struct
    let mut tle = TLE {
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
        tle.satellite_catalog_number = line1[2..7].parse::<i32>().unwrap();

        // Classification
        tle.classification = line1[7..8].parse::<char>().unwrap();

        // International designator
        tle.international_designator = line1[9..17].trim().to_string();

        // Epoch year (last two numbers)
        tle.epoch_year = line1[18..20].parse::<i32>().unwrap();

        // Epoch day of year
        tle.epoch_day = line1[20..32].parse::<f64>().unwrap();

        // 1st derivative of mean motion [revs/day^2]
        tle.first_derivative_of_mean_motion = line1[33..43].parse::<f64>().unwrap() * 2.0;

        // 2nd derivative of mean motion [revs/days^3]
        // Account for - in 2nd derivative of mean motion
        if line1[44..45].parse::<char>().unwrap() == '-' {
            tle.second_derivative_of_mean_motion = format!("-0.{}", line1[45..50].trim()).parse::<f64>().unwrap() * 10.0_f64.powi(line1[50..52].parse::<i32>().unwrap()) * 6.0_f64; // TODO: Fix floating point precision error
        } else {
            tle.second_derivative_of_mean_motion = format!("0.{}", line1[45..50].trim()).parse::<f64>().unwrap() * 10.0_f64.powi(line1[50..52].parse::<i32>().unwrap()) * 6.0_f64; // TODO: Fix floating point precision error
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

pub fn tle_checksum() -> bool {
    /// Validate the TLE hasn't been corrupted by running a checksum test

    // TODO: Write this function!!!
    return true;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_tle_parsing_from_string() {
        let tle_string = "ISS (ZARYA)\n1 25544U 98067A   08264.51782528 -.00002182 -00100-2 -11606-4 0  2927\n2 25544  51.6416 247.4627 0006703 130.5360 325.0288 15.72125391563537";

        let tle = from_string(tle_string);

        assert_eq!(tle.common_name, "ISS (ZARYA)");
        assert_eq!(tle.satellite_catalog_number, 25544);
        assert_eq!(tle.classification, 'U');
        assert_eq!(tle.international_designator, "98067A");
        assert_eq!(tle.epoch_year, 8);
        assert_eq!(tle.epoch_day, 264.51782528);
        assert_eq!(tle.first_derivative_of_mean_motion, -0.00004364);
        assert_eq!(tle.second_derivative_of_mean_motion, -0.00006000000000000001);
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
}
