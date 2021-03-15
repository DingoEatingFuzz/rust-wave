use std::{thread, time};
use std::collections::LinkedList;
use std::f32;
use rand::random;
use clap::clap_app;

fn main() {
    let matches = clap_app!(myapp =>
        (version: "0.2")
        (author: "Michael Lange @DingoEatingFuzz")
        (about: "A simple program that uses a sinusoidal amount of memory")
        (@arg baseline: -b --baseline +takes_value default_value("1500000") "What is the middle value of the sine wave in bytes?. MUST be larger than the magnitude")
        (@arg magnitude: -m --magnitude +takes_value default_value("1000000") "The magnitude of the sine wave in bytes. How far from the baseline are the min and max of the wave?")
        (@arg period: -p --period +takes_value default_value("50") "The period of the sine wave in ticks (500ms). How much time to complete one wave?")
    ).get_matches();

    let two_pi = f32::consts::PI * 2.0;
    let wait_duration = time::Duration::from_millis(500);

    let period = matches.value_of("period").unwrap().parse::<f32>().unwrap();
    let baseline = matches.value_of("baseline").unwrap().parse::<f32>().unwrap();
    let magnitude = matches.value_of("magnitude").unwrap().parse::<f32>().unwrap();

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
