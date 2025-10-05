pub struct Tle {
    common_name: String,
    satellite_catalog_number: i32,
    classification: char,
    international_designator: String,
    epoch_year: i32,
    epoch_day: f64,
    first_derivative_of_mean_motion: f64,
    second_derivative_of_mean_motion: f64,
    bstar: f64,
    ephemeris_type: i32,
    element_set_number: i32,
    inclination: f64,
    right_ascension_of_ascending_node: f64,
    eccentricity: f64,
    argument_of_perigee: f64,
    mean_anomaly: f64,
    mean_motion: f64,
    revolution_number_at_epoch: i64,
}

pub fn from_string(tle_string: &str) {
    /// Function to parse the TLE string and return a Tle struct

    // Create a mutable Tle struct
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
    
    // Split the TLE string into lines
    let lines = tle_string.split("\n").collect::<Vec<&str>>();

    // TLEs can be 2 or 3 lines, account for both cases
    if lines.len() == 3 {
        // First line is the name of the satellite
        if lines[0].len() < 1 || lines[0].len() > 24 {
            println!("TLE string is invalid");
        } else {
            tle.common_name = lines[0].to_string();
        }

        // Second line is the satellite catalog number, classification, international designator, epoch year, epoch day, first derivative of mean motion, second derivative of mean motion, bstar, ephemeris type, element set number
        if lines[1].len() < 69 || lines[1].len() > 69 {
            println!("TLE string is invalid");
        } else {
            // Line 1
            // Satellite catalog number
            tle.satellite_catalog_number = lines[1][2..7].parse::<i32>().unwrap();

            // Classification
            tle.classification = lines[1][7..8].parse::<char>().unwrap();

            // International designator
            tle.international_designator = lines[1][9..17].to_string();

            // Epoch year (last two numbers)
            tle.epoch_year = lines[1][18..20].parse::<i32>().unwrap();

            // Epoch day of year
            tle.epoch_day = lines[1][20..32].parse::<f64>().unwrap();

            // 1st derivative of mean motion [revs/day^2]
            tle.first_derivative_of_mean_motion = lines[1][33..43].parse::<f64>().unwrap() * 2.0;

            // 2nd derivative of mean motion [revs/days^3]
            // Account for - in 2nd derivative of mean motion
            if lines[1][44..45].parse::<char>().unwrap() == '-' {
                tle.second_derivative_of_mean_motion = format!("-0.{}", lines[1][45..50].trim()).parse::<f64>().unwrap() * 10.0_f64.powi(lines[1][50..52].parse::<i32>().unwrap()) * 6.0_f64; // TODO: Fix floating point precision error
            } else {
                tle.second_derivative_of_mean_motion = format!("0.{}", lines[1][45..50].trim()).parse::<f64>().unwrap() * 10.0_f64.powi(lines[1][50..52].parse::<i32>().unwrap()) * 6.0_f64; // TODO: Fix floating point precision error
            }

            // B* [1/Earth Radii]
            // Account for - in B* term
            if lines[1][53..54].parse::<char>().unwrap() == '-' {
                tle.bstar = format!("-0.{}", lines[1][54..59].trim()).parse::<f64>().unwrap() * 10.0_f64.powi(lines[1][59..61].parse::<i32>().unwrap()); // TODO: Fix floating point precision error
            } else {
                tle.bstar = format!("0.{}", lines[1][54..59].trim()).parse::<f64>().unwrap() * 10.0_f64.powi(lines[1][59..61].parse::<i32>().unwrap()); // TODO: Fix floating point precision error
            }

            // Ephemeris type
            tle.ephemeris_type = lines[1][62..63].parse::<i32>().unwrap();

            // Element set number
            tle.element_set_number = lines[1][64..68].trim().parse::<i32>().unwrap();
        }

        if lines[2].len() < 69 || lines[2].len() > 69 {
            println!("TLE string is invalid");
        } else {
            // Line 2
            // Inclination [degs]
            tle.inclination = lines[2][8..16].trim().parse::<f64>().unwrap();

            // Right ascension of ascending node [degs]
            tle.right_ascension_of_ascending_node = lines[2][17..25].trim().parse::<f64>().unwrap();

            // Eccentricity
            tle.eccentricity = format!("0.{}", lines[2][26..33].trim()).parse::<f64>().unwrap();

            // Argument of perigee [degs]
            tle.argument_of_perigee = lines[2][34..42].trim().parse::<f64>().unwrap();

            // Mean anomaly [degs]
            tle.mean_anomaly = lines[2][43..51].trim().parse::<f64>().unwrap();

            // Mean motion [revs/day]
            tle.mean_motion = lines[2][52..63].trim().parse::<f64>().unwrap();

            // Revolution number at epoch
            tle.revolution_number_at_epoch = lines[2][63..68].trim().parse::<i64>().unwrap();
            // TODO: Continue...
            println!("{}", tle.revolution_number_at_epoch);
        }
    } else if lines.len() == 2 {
        println!("TLE string is invalid");
    } else {
        println!("TLE string is invalid");
    }
    
}