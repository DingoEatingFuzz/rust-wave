use std::{thread, time};
use std::collections::LinkedList;
use std::f32;
use rand::random;

fn main() {
    let two_pi = f32::consts::PI * 2.0;
    let wait_duration = time::Duration::from_millis(500);

    // Length of the list when wave as at 0
    let baseline = 1500000.0;
    // Max distance from baseline
    let magnitude = 1000000.0;

    // Number of ticks to make a full wave
    let period = 50.0;

    // Just a list used to take up memory
    let mut list: LinkedList<f32> = LinkedList::new();

    // Make sure the desired length can't go below 0
    assert!(baseline > magnitude);

    let mut tick = 0.0;
    loop {
        // Computed the desired length, based on the tick, baseline, and magnitude
        let desired_len:i32 = (baseline + (tick / period * two_pi).sin() * magnitude) as i32;

        // Figure out how many elements need to be added or removed
        let delta:i32 = desired_len - (list.len() as i32);

        println!("Tick: {}, Current Len: {}, Desired Len: {} Delta: {}", tick, list.len(), desired_len, delta);

        if delta > 0 {
            // Add things to the list
            for _ in 1..delta {
                list.push_back(random());
            }
        } else {
            // Remove things from the list
            let range = delta.abs();
            for _ in 1..range {
                list.pop_back();
            }
        }

        thread::sleep(wait_duration);

        // Increment tick but prevent it from going beyond the period value
        tick = (tick + 1.0) % period;
    }
}
