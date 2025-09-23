use crate::resources::*;
use bevy::prelude::*;

pub fn run_once_system(mut has_run: ResMut<HasRun>) {
    if !has_run.0 {
        println!("Running once in a system.");
        has_run.0 = true;
    }
}
