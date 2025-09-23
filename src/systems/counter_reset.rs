use crate::resources::*;
use bevy::prelude::*;

pub fn counter_reset_demo(mut counter_manager: ResMut<CounterManager>, time: Res<Time>) {
    // Reset counters every 10 seconds for demonstration
    if time.elapsed_secs() > 10.0 && time.elapsed_secs() < 10.1 {
        counter_manager.reset("prints");
        counter_manager.reset("spawns");
        counter_manager.reset("effects");
        println!("--- Counters reset! ---");
    }
}
