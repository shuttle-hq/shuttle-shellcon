# Challenge 3: Memory Optimization - Solutions

## Original Code with Excessive Allocations

```rust
// Before: Using dynamic String allocations for analysis results
pub fn get_analysis_result(params: AnalysisParams) -> AnalysisResult {
    let tank_id = params.tank_id.clone().unwrap_or_else(|| "Tank-A1".to_string());
    let temperature_status = "warning".to_string();
    let ph_status = "critical".to_string();
    let oxygen_status = "normal".to_string();
    let feeding_status = "overdue".to_string();
    let overall_health = "at_risk".to_string();
    let mut recommendations: Vec<String> = Vec::new();
    recommendations.push("Reduce temperature by 2°C".to_string());
    recommendations.push("Adjust pH to 7.2-7.5 range".to_string());
    recommendations.push("Administer emergency feeding".to_string());

    match tank_id.as_str() {
        "Tank-A1" => AnalysisResult {
            tank_id: tank_id.clone(),
            species_id: params.species_id.unwrap_or(1),
            timestamp: chrono::Utc::now().to_rfc3339(),
            temperature_status,
            ph_status,
            oxygen_status,
            feeding_status,
            overall_health,
            recommendations: recommendations.clone(),
        },
        _ => AnalysisResult {
            tank_id: tank_id.clone(),
            species_id: params.species_id.unwrap_or(0),
            timestamp: chrono::Utc::now().to_rfc3339(),
            temperature_status: "unknown".to_string(),
            ph_status: "unknown".to_string(),
            oxygen_status: "unknown".to_string(),
            feeding_status: "unknown".to_string(),
            overall_health: "unknown".to_string(),
            recommendations: vec![
                "Verify tank ID".to_string(),
                "Setup monitoring system".to_string(),
            ],
        },
    }
}
```

## Solution 1: Using Static String References

```rust
// After: Using static string references to eliminate allocations
pub fn get_analysis_result(params: AnalysisParams) -> AnalysisResult {
    // Define static references for commonly used strings
    const WARNING: &str = "warning";
    const CRITICAL: &str = "critical";
    const NORMAL: &str = "normal";
    const OVERDUE: &str = "overdue";
    const AT_RISK: &str = "at_risk";
    const UNKNOWN: &str = "unknown";

    // Define static recommendation strings
    const REC_TEMP: &str = "Reduce temperature by 2°C";
    const REC_PH: &str = "Adjust pH to 7.2-7.5 range";
    const REC_FEED: &str = "Administer emergency feeding";
    const REC_VERIFY: &str = "Verify tank ID";
    const REC_SETUP: &str = "Setup monitoring system";

    // Get tank_id or default to Tank-A1
    let tank_id = params.tank_id.clone().unwrap_or_else(|| "Tank-A1".into());
    
    // Prepare recommendations using static references converted to String
    let recommendations: Vec<String> = vec![REC_TEMP.into(), REC_PH.into(), REC_FEED.into()];

    match tank_id.as_str() {
        "Tank-A1" => AnalysisResult {
            tank_id: tank_id.clone(),
            species_id: params.species_id.unwrap_or(1),
            timestamp: chrono::Utc::now().to_rfc3339(),
            temperature_status: WARNING.into(),
            ph_status: CRITICAL.into(),
            oxygen_status: NORMAL.into(),
            feeding_status: OVERDUE.into(),
            overall_health: AT_RISK.into(),
            recommendations: recommendations.clone(),
        },
        _ => AnalysisResult {
            tank_id: tank_id.clone(),
            species_id: params.species_id.unwrap_or(0),
            timestamp: chrono::Utc::now().to_rfc3339(),
            temperature_status: UNKNOWN.into(),
            ph_status: UNKNOWN.into(),
            oxygen_status: UNKNOWN.into(),
            feeding_status: UNKNOWN.into(),
            overall_health: UNKNOWN.into(),
            recommendations: vec![
                REC_VERIFY.into(),
                REC_SETUP.into(),
            ],
        },
    }
}
```

## Solution 2: Using String Interning

```rust
// After: Using string interning to eliminate duplicate allocations
use internment::Intern;

pub fn get_analysis_result(params: AnalysisParams) -> AnalysisResult {
    // Get tank_id or default to Tank-A1
    let tank_id = params.tank_id.clone().unwrap_or_else(|| "Tank-A1".into());

    // Intern frequently used status strings
    let warning = Intern::new("warning");
    let critical = Intern::new("critical");
    let normal = Intern::new("normal");
    let overdue = Intern::new("overdue");
    let at_risk = Intern::new("at_risk");
    let unknown = Intern::new("unknown");

    // Intern recommendation strings
    let rec_temp = Intern::new("Reduce temperature by 2°C");
    let rec_ph = Intern::new("Adjust pH to 7.2-7.5 range");
    let rec_feed = Intern::new("Administer emergency feeding");
    let rec_verify = Intern::new("Verify tank ID");
    let rec_setup = Intern::new("Setup monitoring system");

    // Prepare recommendations converted to `String` once
    let recommendations: Vec<String> = vec![
        rec_temp.as_ref().into(),
        rec_ph.as_ref().into(),
        rec_feed.as_ref().into(),
    ];

    match tank_id.as_str() {
        "Tank-A1" => AnalysisResult {
            tank_id: tank_id.clone(),
            species_id: params.species_id.unwrap_or(1),
            timestamp: chrono::Utc::now().to_rfc3339(),
            temperature_status: warning.as_ref().into(),
            ph_status: critical.as_ref().into(),
            oxygen_status: normal.as_ref().into(),
            feeding_status: overdue.as_ref().into(),
            overall_health: at_risk.as_ref().into(),
            recommendations: recommendations.clone(),
        },
        _ => AnalysisResult {
            tank_id: tank_id.clone(),
            species_id: params.species_id.unwrap_or(0),
            timestamp: chrono::Utc::now().to_rfc3339(),
            temperature_status: unknown.as_ref().into(),
            ph_status: unknown.as_ref().into(),
            oxygen_status: unknown.as_ref().into(),
            feeding_status: unknown.as_ref().into(),
            overall_health: unknown.as_ref().into(),
            recommendations: vec![
                rec_verify.as_ref().into(),
                rec_setup.as_ref().into(),
            ],
        },
    }
}
```

## Comparing the Solutions

### Solution 1: Static String References
- **Pros**: Simple to implement, no external dependencies, very memory efficient for static text
- **Cons**: Still requires some `.to_string()` calls when creating the `AnalysisResult` since it needs `String` values
- **Best for**: Simpler systems without extremely repetitive string values

### Solution 2: String Interning
- **Pros**: More efficient for highly repetitive strings, eliminates heap allocations for duplicates
- **Cons**: Adds dependency on the `internment` crate, slightly more complex implementation
- **Best for**: Systems with many duplicate string values across larger datasets

Both solutions significantly reduce memory usage by eliminating unnecessary string allocations, with the interning approach being more sophisticated and potentially more efficient at scale.
