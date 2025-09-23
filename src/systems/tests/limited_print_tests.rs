
// Helper to check if the limited_print_system would execute its 'print' logic
fn should_print(counter_val: u32, max_iterations_val: u32) -> bool {
    counter_val < max_iterations_val
}

// Helper to check if the limited_print_system would execute its 'limit reached' logic
fn should_print_limit_reached(counter_val: u32, max_iterations_val: u32) -> bool {
    counter_val == max_iterations_val
}

#[test]
fn test_limited_print_conditions() {
    let max_iter = 3;

    // Before limit
    assert!(should_print(0, max_iter));
    assert!(!should_print_limit_reached(0, max_iter));

    assert!(should_print(1, max_iter));
    assert!(!should_print_limit_reached(1, max_iter));

    assert!(should_print(2, max_iter));
    assert!(!should_print_limit_reached(2, max_iter));

    // At limit
    assert!(!should_print(3, max_iter));
    assert!(should_print_limit_reached(3, max_iter));

    // After limit
    assert!(!should_print(4, max_iter));
    assert!(!should_print_limit_reached(4, max_iter));
}
