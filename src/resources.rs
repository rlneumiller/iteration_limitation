use bevy::prelude::*;
use std::collections::HashMap;

#[derive(Resource)]
pub struct HasRun(pub bool);

#[derive(Resource)]
pub struct MaxIterations(pub usize);

#[derive(Resource)]
pub struct Counter(pub usize);

#[derive(Resource)]
pub struct CounterManager {
    pub counters: HashMap<String, usize>,
    pub limits: HashMap<String, usize>,
}

impl CounterManager {
    pub fn new() -> Self {
        Self {
            counters: HashMap::new(),
            limits: HashMap::new(),
        }
    }

    pub fn add_counter(&mut self, name: &str, limit: usize) {
        self.counters.insert(name.to_string(), 0);
        self.limits.insert(name.to_string(), limit);
    }

    pub fn increment(&mut self, name: &str) -> bool {
        if let Some(counter) = self.counters.get_mut(name) {
            if let Some(limit) = self.limits.get(name) {
                if *counter < *limit {
                    *counter += 1;
                    return true;
                }
            }
        }
        false
    }

    pub fn can_increment(&self, name: &str) -> bool {
        if let (Some(counter), Some(limit)) = (self.counters.get(name), self.limits.get(name)) {
            *counter < *limit
        } else {
            false
        }
    }

    pub fn get_count(&self, name: &str) -> usize {
        self.counters.get(name).copied().unwrap_or(0)
    }

    pub fn get_limit(&self, name: &str) -> usize {
        self.limits.get(name).copied().unwrap_or(0)
    }

    pub fn reset(&mut self, name: &str) {
        if let Some(counter) = self.counters.get_mut(name) {
            *counter = 0;
        }
    }
}
