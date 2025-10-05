mod tle;

fn main() {
    let tle_string = "ISS (ZARYA)\n1 25544U 98067A   08264.51782528 -.00002182 -00100-2 -11606-4 0  2927\n2 25544  51.6416 247.4627 0006703 130.5360 325.0288 15.72125391563537";
    
    let tle_data = tle::from_string(tle_string);
    // println!("Satellite catalog number: {}", tle.satellite_catalog_number);
    // println!("Classification: {}", tle.classification);
    // println!("International designator: {}", tle.international_designator);
    // println!("Epoch year: {}", tle.epoch_year);
    // println!("Epoch day: {}", tle.epoch_day);
    // println!("First derivative of mean motion: {}", tle.first_derivative_of_mean_motion);
    // println!("Second derivative of mean motion: {}", tle.second_derivative_of_mean_motion);
    // println!("Bstar: {}", tle.bstar);
    // println!("Ephemeris type: {}", tle.ephemeris_type);
    // println!("Element set number: {}", tle.element_set_number);
    // println!("Inclination: {}", tle.inclination);
    // println!("Right ascension of ascending node: {}", tle.right_ascension_of_ascending_node);
    // println!("Eccentricity: {}", tle.eccentricity);
    // println!("Argument of perigee: {}", tle.argument_of_perigee);
    // println!("Mean anomaly: {}", tle.mean_anomaly);
    // println!("Mean motion: {}", tle.mean_motion);
    // println!("Revolution number at epoch: {}", tle.revolution_number_at_epoch);
}
