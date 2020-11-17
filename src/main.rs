extern crate sysfs_gpio;

use sysfs_gpio::{Direction, Pin};
use std::thread::sleep;
use std::time::Duration;

fn main() {
    println!("Hello, world!");

    let r_led = Pin::new(17); // number depends on chip, etc.s

    r_led.with_exported(|| {
        // When running as non-root, need to wait a few ms (100 or so)
        // for exported pin to become available
        sleep(Duration::from_millis(100));
        r_led.set_direction(Direction::Out).unwrap();

        for _ in 0..10 {
            r_led.set_value(0)?;
            sleep(Duration::from_millis(200));
            r_led.set_value(1)?;
            sleep(Duration::from_millis(200));
        }

        r_led.set_value(0)?;
        Ok(())
    }).unwrap();

    println!("Done!");
}