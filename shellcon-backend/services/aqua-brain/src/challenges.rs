use serde_json::json;
use shuttle_axum::axum::{
    response::IntoResponse,
    Json,
};

use crate::{AnalysisParams, AnalysisResult, ParameterStatus, FeedingStatus, OverallHealth}; // Import types from main.rs

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
    // Determine status using enums from main.rs
    let temperature_status = ParameterStatus::Warning;
    let ph_status = ParameterStatus::Critical;
    let oxygen_status = ParameterStatus::Normal;
    let feeding_status = FeedingStatus::Overdue;
    let overall_health = OverallHealth::AtRisk;
    let recommendations: Vec<String> = vec![
        "Reduce temperature by 2°C".to_string(),
        "Adjust pH to 7.2-7.5 range".to_string(),
        "Administer emergency feeding".to_string(),
    ];

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
            recommendations,
        },
        _ => AnalysisResult {
            tank_id: tank_id.clone(),
            species_id: params.species_id.unwrap_or(0),
            timestamp: chrono::Utc::now().to_rfc3339(),
            temperature_status: ParameterStatus::Unknown,
            ph_status: ParameterStatus::Unknown,
            oxygen_status: ParameterStatus::Unknown,
            feeding_status: FeedingStatus::Unknown,
            overall_health: OverallHealth::Unknown,
            recommendations: vec![
                "Verify tank ID".to_string(),
                "Setup monitoring system".to_string(),
            ],
        },
    }
}
// ⚠️ END CHALLENGE CODE ⚠️
