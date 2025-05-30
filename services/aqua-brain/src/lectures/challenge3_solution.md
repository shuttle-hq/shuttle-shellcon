```rust
// Before: Using String objects that allocate memory
pub fn analyze_tank_conditions(temperature: f32, ph_level: f32) -> (String, String) {
    let temp_status = if temperature > 26.0 {
        String::from("warning")
    } else {
        String::from("normal")
    };
    
    let ph_status = if ph_level < 6.5 || ph_level > 8.0 {
        String::from("warning")
    } else {
        String::from("normal")
    };
    
    (temp_status, ph_status)
}

// After: Using static string references to avoid allocation
pub fn analyze_tank_conditions(temperature: f32, ph_level: f32) -> (&'static str, &'static str) {
    let temp_status = if temperature > 26.0 {
        "warning"
    } else {
        "normal"
    };
    
    let ph_status = if ph_level < 6.5 || ph_level > 8.0 {
        "warning"
    } else {
        "normal"
    };
    
    (temp_status, ph_status)
}
```

This solution addresses memory usage by replacing dynamic String allocations with static string references (&'static str). When working with fixed, known string values, using references instead of creating new String objects for each function call significantly reduces memory allocations and improves performance. The &'static str type indicates that these string references have a 'static lifetime, meaning they live for the entire duration of the program, typically stored in the binary itself rather than on the heap.
