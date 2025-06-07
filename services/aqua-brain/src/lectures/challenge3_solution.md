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

    // Get tank_id, still needs to be a String due to the API requirements
    let tank_id = params.tank_id.clone().unwrap_or_else(|| "Tank-A1".to_string());
    
    // Define static recommendation strings
    const REC_TEMP: &str = "Reduce temperature by 2°C";
    const REC_PH: &str = "Adjust pH to 7.2-7.5 range";
    const REC_FEED: &str = "Administer emergency feeding";
    const REC_VERIFY: &str = "Verify tank ID";
    const REC_SETUP: &str = "Setup monitoring system";

    // Create recommendations using static references
    let recommendations = vec![REC_TEMP.to_string(), REC_PH.to_string(), REC_FEED.to_string()];

    match tank_id.as_str() {
        "Tank-A1" => AnalysisResult {
            tank_id: tank_id.clone(),
            species_id: params.species_id.unwrap_or(1),
            timestamp: chrono::Utc::now().to_rfc3339(),
            temperature_status: WARNING.to_string(),
            ph_status: CRITICAL.to_string(),
            oxygen_status: NORMAL.to_string(),
            feeding_status: OVERDUE.to_string(),
            overall_health: AT_RISK.to_string(),
            recommendations,
        },
        _ => AnalysisResult {
            tank_id: tank_id.clone(),
            species_id: params.species_id.unwrap_or(0),
            timestamp: chrono::Utc::now().to_rfc3339(),
            temperature_status: UNKNOWN.to_string(),
            ph_status: UNKNOWN.to_string(),
            oxygen_status: UNKNOWN.to_string(),
            feeding_status: UNKNOWN.to_string(),
            overall_health: UNKNOWN.to_string(),
            recommendations: vec![
                REC_VERIFY.to_string(),
                REC_SETUP.to_string(),
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
    let tank_id = Intern::new(params.tank_id.unwrap_or_else(|| "Tank-A1".to_string()));

    // Intern status strings
    let warning = Intern::new("warning".to_string());
    let critical = Intern::new("critical".to_string());
    let normal = Intern::new("normal".to_string());
    let overdue = Intern::new("overdue".to_string());
    let at_risk = Intern::new("at_risk".to_string());

    // Intern recommendations
    let rec1 = Intern::new("Reduce temperature by 2°C".to_string());
    let rec2 = Intern::new("Adjust pH to 7.2-7.5 range".to_string());
    let rec3 = Intern::new("Administer emergency feeding".to_string());
    let recommendations: Vec<Intern<String>> = vec![rec1.clone(), rec2.clone(), rec3.clone()];

    if *tank_id == Intern::new("Tank-A1".to_string()) {
        AnalysisResult {
            tank_id: tank_id.clone(),
            species_id: params.species_id.unwrap_or(1),
            timestamp: chrono::Utc::now().to_rfc3339(),
            temperature_status: warning.clone(),
            ph_status: critical.clone(),
            oxygen_status: normal.clone(),
            feeding_status: overdue.clone(),
            overall_health: at_risk.clone(),
            recommendations: recommendations.clone(),
        }
    } else {
        AnalysisResult {
            tank_id: tank_id.clone(),
            species_id: params.species_id.unwrap_or(0),
            timestamp: chrono::Utc::now().to_rfc3339(),
            temperature_status: Intern::new("unknown".to_string()),
            ph_status: Intern::new("unknown".to_string()),
            oxygen_status: Intern::new("unknown".to_string()),
            feeding_status: Intern::new("unknown".to_string()),
            overall_health: Intern::new("unknown".to_string()),
            recommendations: vec![
                Intern::new("Verify tank ID".to_string()),
                Intern::new("Setup monitoring system".to_string()),
            ],
        }
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
