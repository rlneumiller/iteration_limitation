use bevy::prelude::*;

pub fn should_reset_counters(time: Res<Time>) -> bool {
    // Reset every 10 seconds
    time.elapsed_secs() > 10.0 && time.elapsed_secs() < 10.1
}

fn main() {
    println!("This is an example of counter reset logic.");
    println!("In a real Bevy app, you would use should_reset_counters as a run condition.");
}
