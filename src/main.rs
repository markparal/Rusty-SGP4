mod tle;

fn main() {
    let tle_string = "ISS (ZARYA)\n1 25544U 98067A   08264.51782528 -.00002182 -00100-2 -11606-4 0  2927\n2 25544  51.6416 247.4627 0006703 130.5360 325.0288 15.72125391563537";
    
    let tle_data = tle::from_string(tle_string);
    println!("Satellite catalog number: {}", tle_data.satellite_catalog_number);
    println!("Classification: {}", tle_data.classification);
    println!("International designator: {}", tle_data.international_designator);
    println!("Epoch year: {}", tle_data.epoch_year);
    println!("Epoch day: {}", tle_data.epoch_day);
    println!("First derivative of mean motion: {}", tle_data.first_derivative_of_mean_motion);
    println!("Second derivative of mean motion: {}", tle_data.second_derivative_of_mean_motion);
    println!("Bstar: {}", tle_data.bstar);
    println!("Ephemeris type: {}", tle_data.ephemeris_type);
    println!("Element set number: {}", tle_data.element_set_number);
    println!("Inclination: {}", tle_data.inclination);
    println!("Right ascension of ascending node: {}", tle_data.right_ascension_of_ascending_node);
    println!("Eccentricity: {}", tle_data.eccentricity);
    println!("Argument of perigee: {}", tle_data.argument_of_perigee);
    println!("Mean anomaly: {}", tle_data.mean_anomaly);
    println!("Mean motion: {}", tle_data.mean_motion);
    println!("Revolution number at epoch: {}", tle_data.revolution_number_at_epoch);
}
