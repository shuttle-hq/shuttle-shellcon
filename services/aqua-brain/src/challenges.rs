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
    const NORMAL: &str = "normal";
    const WARNING: &str = "warning";
    const CRITICAL: &str = "critical";
    const UNKNOWN: &str = "unknown";
    const GOOD: &str = "good";
    const CAUTION: &str = "caution";
    const AT_RISK: &str = "at_risk";
    const OVERDUE: &str = "overdue";
    const EXCESS: &str = "excess";
    const LOW: &str = "low";
    const HIGH: &str = "high";
    
    // Get tank_id or default to Tank-A1 using &str instead of String
    let tank_id = params.tank_id.as_deref().unwrap_or("Tank-A1");
    
    // Generate analysis result based on tank ID
    match tank_id {
        "Tank-A1" => AnalysisResult {
            tank_id: tank_id.to_string(),
            species_id: params.species_id.unwrap_or(1),
            timestamp: chrono::Utc::now().to_rfc3339(),
            temperature_status: WARNING,
            ph_status: CRITICAL,
            oxygen_status: NORMAL,
            feeding_status: OVERDUE,
            overall_health: AT_RISK,
            recommendations: vec![
                "Reduce temperature by 2°C",
                "Adjust pH to 7.2-7.5 range",
                "Administer emergency feeding",
            ],
        },
        "Tank-B2" => AnalysisResult {
            tank_id: tank_id.to_string(),
            species_id: params.species_id.unwrap_or(2),
            timestamp: chrono::Utc::now().to_rfc3339(),
            temperature_status: NORMAL,
            ph_status: NORMAL,
            oxygen_status: LOW,
            feeding_status: NORMAL,
            overall_health: GOOD,
            recommendations: vec![
                "Increase aeration slightly",
                "Monitor oxygen levels daily",
            ],
        },
        "Tank-C3" => AnalysisResult {
            tank_id: tank_id.to_string(),
            species_id: params.species_id.unwrap_or(3),
            timestamp: chrono::Utc::now().to_rfc3339(),
            temperature_status: NORMAL,
            ph_status: HIGH,
            oxygen_status: NORMAL,
            feeding_status: EXCESS,
            overall_health: CAUTION,
            recommendations: vec![
                "Reduce feeding frequency",
                "Perform 25% water change",
                "Test ammonia levels",
            ],
        },
        _ => AnalysisResult {
            tank_id: tank_id.to_string(),
            species_id: params.species_id.unwrap_or(0),
            timestamp: chrono::Utc::now().to_rfc3339(),
            temperature_status: UNKNOWN,
            ph_status: UNKNOWN,
            oxygen_status: UNKNOWN,
            feeding_status: UNKNOWN,
            overall_health: UNKNOWN,
            recommendations: vec![
                "Verify tank ID",
                "Setup monitoring system",
            ],
        },
    }
}
// ⚠️ END CHALLENGE CODE ⚠️


