extern crate sysfs_gpio;
use rand::prelude::*;
use std::thread::sleep;
use std::time::Duration;
use sysfs_gpio::{Direction, Pin, Edge};

fn main() {
    rgb_touch();
}

fn flash_led() {
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
}

fn rgb_touch() {
    let mut rng = rand::thread_rng();

    let b_led = Pin::new(23); // number depends on chip, etc.s
    let g_led = Pin::new(24); // number depends on chip, etc.s
    let r_led = Pin::new(25); // number depends on chip, etc.s

    let touch_sensor = Pin::new(5);

    touch_sensor.with_exported(|| {
        b_led.export()?;
        g_led.export()?;
        r_led.export()?;
        sleep(Duration::from_millis(100));

        r_led.set_direction(Direction::Out).unwrap();
        g_led.set_direction(Direction::Out).unwrap();
        b_led.set_direction(Direction::Out).unwrap();
        touch_sensor.set_direction(Direction::In).unwrap();
        touch_sensor.set_edge(Edge::FallingEdge)?;
        let mut touch_poller = touch_sensor.get_poller().unwrap();
    
        for i in 0..1000 {
            let touch_value = touch_poller.poll(1000).unwrap();
            println!("{}: {:?}", i, touch_value);
            let led = match rng.gen_range(0, 3) {
                0 => r_led,
                1 => g_led,
                _ => b_led 
            };
            match touch_value {
                None => {
                    r_led.set_value(1)?;
                    g_led.set_value(1)?;
                    b_led.set_value(1)?;
                },
                Some(val) => {
                    led.set_value(val)?;
                },
            }
            
        }
        Ok(())
    }).unwrap_or(());
    
    r_led.unexport().unwrap_or(());
    b_led.unexport().unwrap_or(());
    g_led.unexport().unwrap_or(());
}
