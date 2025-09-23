
use crate::resources::CounterManager;

#[test]
fn test_counter_manager_add_and_get() {
    let mut manager = CounterManager::new();
    manager.add_counter("test_counter", 5);

    assert_eq!(manager.get_limit("test_counter"), 5);
    assert_eq!(manager.get_count("test_counter"), 0);
    assert_eq!(manager.get_limit("non_existent"), 0);
    assert_eq!(manager.get_count("non_existent"), 0);
}

#[test]
fn test_counter_manager_increment() {
    let mut manager = CounterManager::new();
    manager.add_counter("inc_counter", 2);

    // Increment within limit
    assert!(manager.increment("inc_counter"));
    assert_eq!(manager.get_count("inc_counter"), 1);

    assert!(manager.increment("inc_counter"));
    assert_eq!(manager.get_count("inc_counter"), 2);

    // Increment beyond limit
    assert!(!manager.increment("inc_counter"));
    assert_eq!(manager.get_count("inc_counter"), 2);

    // Increment non-existent counter
    assert!(!manager.increment("non_existent"));
}

#[test]
fn test_counter_manager_can_increment() {
    let mut manager = CounterManager::new();
    manager.add_counter("can_inc_counter", 1);

    // Can increment within limit
    assert!(manager.can_increment("can_inc_counter"));
    assert_eq!(manager.get_count("can_inc_counter"), 0); // Should not increment

    manager.increment("can_inc_counter");

    // Cannot increment beyond limit
    assert!(!manager.can_increment("can_inc_counter"));
    assert_eq!(manager.get_count("can_inc_counter"), 1);

    // Can increment non-existent counter (returns false)
    assert!(!manager.can_increment("non_existent"));
}

#[test]
fn test_counter_manager_reset() {
    let mut manager = CounterManager::new();
    manager.add_counter("reset_counter", 3);

    manager.increment("reset_counter");
    manager.increment("reset_counter");
    assert_eq!(manager.get_count("reset_counter"), 2);

    manager.reset("reset_counter");
    assert_eq!(manager.get_count("reset_counter"), 0);

    // Reset non-existent counter (should not panic)
    manager.reset("non_existent");
}
