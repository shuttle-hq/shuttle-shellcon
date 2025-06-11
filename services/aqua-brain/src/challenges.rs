use serde_json::json;
use shuttle_axum::axum::{
    response::IntoResponse,
    Json,
};
use tracing;

use crate::AnalysisParams;
use crate::AnalysisResult;

// Dedicated endpoint for testing Challenge #1
pub async fn test_challenge_1() -> impl IntoResponse {
    // Create a span for tracking sensor latency diagnostics
    let span = tracing::info_span!("sensor_latency_diagnostic");
    let _guard = span.enter();
    
    tracing::info!("Sensor response time diagnostic requested");
    
    let response = json!({
        "id": 1,
        "name": "The Sluggish Sensor ",
        "message": "For validation, please call the aqua-monitor service at /api/challenges/1/validate",
        "hint": "Replace std::fs::read_to_string with tokio::fs::read_to_string and add .await to make the file I/O operation async.",
        "system_component": {
            "name": "Analysis Engine ",
            "status": "normal",
            "description": "Analysis engine operating normally "
        }
    });
    
    tracing::info!(
        challenge_id = 1,
        challenge_name = "latency-issue",
        "Challenge #1 test endpoint called - redirecting to service-specific validation"
    );
    
    Json(response)
}

// ⚠️ CHALLENGE #3: STRING ALLOCATION OPTIMIZATION ⚠️
// This function creates new String objects for every analysis result
// Your task: Optimize memory usage by reducing unnecessary String allocations
// Hint: Consider using static &str references, Cow<'a, str>, or string interning
pub fn get_analysis_result(params: AnalysisParams) -> AnalysisResult {
    // Get tank_id or default to Tank-A1
    let tank_id = params.tank_id.clone().unwrap_or_else(|| "Tank-A1".to_string());
    // Dynamically allocate status and recommendation strings
    let temperature_status = "warning".to_string();
    let ph_status = "critical".to_string();
    let oxygen_status = "normal".to_string();
    let feeding_status = "overdue".to_string();
    let overall_health = "at_risk".to_string();
    let mut recommendations: Vec<String> = Vec::new();
    recommendations.push("Reduce temperature by 2°C".to_string());
    recommendations.push("Adjust pH to 7.2-7.5 range".to_string());
    recommendations.push("Administer emergency feeding".to_string());

    // Generate analysis result based on tank ID
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
// ⚠️ END CHALLENGE CODE ⚠️
