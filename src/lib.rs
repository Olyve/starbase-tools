//! # Starbase Ship Building Tools
//!
//! This library contains a set of tools that ship builders in the game Starbase will
//! find useful while creating or updating their ships. The functions that are provided here
//! are sourced from the community and from Collective.
//!
//! Starbase Tools is a work in progress. New functionality will likely be added as the community
//! discovers more useful calculations. Starbase is in early access and so it is also likely that
//! some of the values could change as things are balanced (ex: thruster values could change).
//!
//! This crate will not reach a stable v1 until Starbase has left early access and reaches a stable
//! version. The crate should still be perfectly usable in the current state though.

const BOX_THRUSTER_THRUST: f64 = 500_000.0;
const TRIANGLE_THRUSTER_THRUST: f64 = 300_000.0;
const DRAG_COEFFICIENT: f64 = -3.5503;
const SPEED_COEFFICIENT: f64 = 46.1538;

/// # Calculate a Potential Ship Speed
///
/// The `calculate_potential_ship_speed` function is used to calculate a potential speed for a ship
/// based on the number of box thrusters, triangle thrusters, and the weight of the ship.
///
/// It is important to note that this is a possible speed because the actual speed may differ depending
/// on how well balanced the ship is and whether or not thrusters are placed at an angle. If your
/// ship is symmetrical and all thrusters are aligned forwards or backwards, then the speed should be
/// relatively accurate.
///
/// There are no guarantees that your ship will match this speed, but it is a
/// good tool to try and get an estimate of how many thrusters you may need based on ship weight.
pub fn calculate_potential_ship_speed(num_of_box: &u32, num_of_tri: &u32, weight: &f64) -> f64 {
    let thrust = calculate_thrust(num_of_box, num_of_tri);
    let thrust_divided_by_weight = thrust / weight;

    let raw_speed = DRAG_COEFFICIENT * thrust_divided_by_weight.powi(2)
        + SPEED_COEFFICIENT * thrust_divided_by_weight;

    let rounded_speed = (raw_speed * 100.0).round() / 100.0;

    return rounded_speed;
}

fn calculate_thrust(num_of_box: &u32, num_of_tri: &u32) -> f64 {
    let num_of_box = *num_of_box as f64;
    let num_of_tri = *num_of_tri as f64;

    (num_of_box * &BOX_THRUSTER_THRUST) + (num_of_tri * &TRIANGLE_THRUSTER_THRUST)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn function_name_test() {
        let num_of_box = 1;
        let num_of_tri = 1;
        let expected_thrust = &BOX_THRUSTER_THRUST + &TRIANGLE_THRUSTER_THRUST;

        let thrust = calculate_thrust(&num_of_box, &num_of_tri);

        assert_eq!(thrust, expected_thrust)
    }
}
