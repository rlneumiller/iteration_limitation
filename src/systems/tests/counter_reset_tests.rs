
use crate::resources::CounterManager;

#[test]
fn test_counter_manager_reset_functionality() {
    let mut manager = CounterManager::new();
    manager.add_counter("test_counter_1", 5);
    manager.add_counter("test_counter_2", 10);

    // Increment counters
    manager.increment("test_counter_1");
    manager.increment("test_counter_1");
    manager.increment("test_counter_2");
    assert_eq!(manager.get_count("test_counter_1"), 2);
    assert_eq!(manager.get_count("test_counter_2"), 1);

    // Reset a specific counter
    manager.reset("test_counter_1");
    assert_eq!(manager.get_count("test_counter_1"), 0);
    assert_eq!(manager.get_count("test_counter_2"), 1); // Other counter should be unaffected

    // Reset a non-existent counter (should not panic)
    manager.reset("non_existent_counter");
}
