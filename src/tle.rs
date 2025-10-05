pub struct Tle {
    common_name: String,
    satellite_catalog_number: i32,
    classification: char,
    international_designator: String,
    epoch_year: i32,
    epoch_day: f64,
    first_derivative_of_mean_motion: f32,
    second_derivative_of_mean_motion: f32,
    bstar: f32,
    ephemeris_type: i32,
    element_set_number: i32,
    inclination: f32,
    right_ascension_of_ascending_node: f32,
    eccentricity: f32,
    argument_of_perigee: f32,
    mean_anomaly: f32,
    mean_motion: f32,
    revolution_number_at_epoch: i32,
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
            tle.satellite_catalog_number = lines[1][2..7].parse::<i32>().unwrap();
            tle.classification = lines[1][7..8].parse::<char>().unwrap();
            tle.international_designator = lines[1][9..17].to_string();
            tle.epoch_year = lines[1][18..20].parse::<i32>().unwrap();
            tle.epoch_day = lines[1][20..32].parse::<f64>().unwrap();
            // TODO: Continue...
            println!("{} {}", tle.epoch_year, tle.epoch_day);
        }

    } else if lines.len() == 2 {
        println!("TLE string is invalid");
    } else {
        println!("TLE string is invalid");
    }
    
}