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

// ⚠️ CHALLENGE #3: MEMORY OPTIMIZATION ⚠️
// This function creates new String objects for every analysis result
// Your task: Optimize memory usage by using static string references instead of creating new String objects
pub fn get_analysis_result(params: AnalysisParams) -> AnalysisResult {
    // Get tank_id or default to Tank-A1
    let tank_id = params.tank_id.clone().unwrap_or_else(|| "Tank-A1".to_string());
    
    // Log the analysis operation with structured fields
    tracing::debug!(
        tank_id = %tank_id,
        species_id = params.species_id,
        analysis_type = "environmental",
        operation = "tank_analysis",
        "Analyzing tank environmental conditions"
    );
    
    // Generate analysis result based on tank ID
    match tank_id.as_str() {
        "Tank-A1" => AnalysisResult {
            tank_id: tank_id,
            species_id: params.species_id.unwrap_or(1),
            timestamp: chrono::Utc::now().to_rfc3339(),
            temperature_status: "warning",
            ph_status: "critical",
            oxygen_status: "normal",
            feeding_status: "overdue",
            overall_health: "at_risk",
            recommendations: vec![
                "Reduce temperature by 2°C",
                "Adjust pH to 7.2-7.5 range",
                "Administer emergency feeding",
            ],
        },
        "Tank-B2" => AnalysisResult {
            tank_id: tank_id,
            species_id: params.species_id.unwrap_or(2),
            timestamp: chrono::Utc::now().to_rfc3339(),
            temperature_status: "normal",
            ph_status: "normal",
            oxygen_status: "low",
            feeding_status: "normal",
            overall_health: "good",
            recommendations: vec![
                "Increase aeration slightly",
                "Monitor oxygen levels daily",
            ],
        },
        "Tank-C3" => AnalysisResult {
            tank_id: tank_id,
            species_id: params.species_id.unwrap_or(3),
            timestamp: chrono::Utc::now().to_rfc3339(),
            temperature_status: "normal",
            ph_status: "high",
            oxygen_status: "normal",
            feeding_status: "excess",
            overall_health: "caution",
            recommendations: vec![
                "Reduce feeding frequency",
                "Perform 25% water change",
                "Test ammonia levels",
            ],
        },
        _ => AnalysisResult {
            tank_id: tank_id,
            species_id: params.species_id.unwrap_or(0),
            timestamp: chrono::Utc::now().to_rfc3339(),
            temperature_status: "unknown",
            ph_status: "unknown",
            oxygen_status: "unknown",
            feeding_status: "unknown",
            overall_health: "unknown",
            recommendations: vec![
                "Verify tank ID",
                "Setup monitoring system",
            ],
        },
    }
}
// ⚠️ END CHALLENGE CODE ⚠️


