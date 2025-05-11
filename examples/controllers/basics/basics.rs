use webots::{DistanceSensor, Robot};

fn main() {
    const INFINITY: f64 = 1.0 / 0.0;
    const MAX_SPEED: f64 = 6.28;
    const TIME_STEP: i32 = 64;

    println!("Rust controller has started");
    let _robot = Robot::default();

    let distance_sensor_names = vec!["ps0", "ps1", "ps2", "ps3", "ps4", "ps5", "ps6", "ps7"];
    let distance_sensors: Vec<DistanceSensor> = distance_sensor_names
        .iter()
        .map(|name| {
            let sensor = Robot::distance_sensor(name);
            sensor.enable(TIME_STEP);
            sensor
        })
        .collect();

    let left_motor = Robot::motor("left wheel motor");
    let right_motor = Robot::motor("right wheel motor");
    left_motor.set_position(INFINITY);
    right_motor.set_position(INFINITY);

    left_motor.set_velocity(0.1 * MAX_SPEED);
    right_motor.set_velocity(0.1 * MAX_SPEED);

    loop {
        if Robot::step(TIME_STEP) == -1 {
            break;
        }

        let distance_values: Vec<f64> = distance_sensors
            .iter()
            .map(|sensor| sensor.value())
            .collect();

        // detect obsctacles
        let left_obstacle =
            distance_values[5] > 80.0 || distance_values[6] > 80.0 || distance_values[7] > 80.0;
        let right_obstacle =
            distance_values[0] > 80.0 || distance_values[1] > 80.0 || distance_values[2] > 80.0;

        // initialize motor speeds at 50% of MAX_SPEED.
        let mut left_speed = 0.5 * MAX_SPEED;
        let mut right_speed = 0.5 * MAX_SPEED;

        // modify speeds according to obstacles
        if left_obstacle {
            // turn right
            left_speed += 0.5 * MAX_SPEED;
            right_speed -= 0.5 * MAX_SPEED;
        } else if right_obstacle {
            // turn left
            left_speed -= 0.5 * MAX_SPEED;
            right_speed += 0.5 * MAX_SPEED;
        }

        // write actuators inputs
        left_motor.set_velocity(left_speed);
        right_motor.set_velocity(right_speed);
    }
}
