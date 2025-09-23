pub fn once_static_system() {
    static ONESHOT_A: std::sync::Once = std::sync::Once::new();
    ONESHOT_A.call_once(|| {
        println!("Demonstrating call_once");
    });
}