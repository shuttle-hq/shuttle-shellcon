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
        "Challenge #1 test endpoint called - redirecting to service-specific validation",
    );

    Json(response)
}

// ⚠️ CHALLENGE #3: STRING ALLOCATION OPTIMIZATION ⚠️
// This function creates new String objects for every analysis result
// Your task: Optimize memory usage by reducing unnecessary String allocations
// Hint: Consider using static &str references, Cow<'a, str>, or string interning
pub fn get_analysis_result(params: AnalysisParams) -> AnalysisResult {
    // Determine which tank we are analysing – default to "Tank-A1" if none provided
    let tank_id = params
        .tank_id
        .clone()
        .unwrap_or_else(|| "Tank-A1".to_string());

    // Helper to minimise repeated `String` allocations for static text
    fn to_string_vec(items: &[&'static str]) -> Vec<String> {
        items.iter().map(|s| (*s).to_string()).collect()
    }

    match tank_id.as_str() {
        // Existing demo tank (slightly unhealthy)
        "Tank-A1" => AnalysisResult {
            tank_id: tank_id.clone(),
            species_id: params.species_id.unwrap_or(1), // Neon Tetra
            timestamp: chrono::Utc::now().to_rfc3339(),
            temperature_status: ParameterStatus::Warning,
            ph_status: ParameterStatus::Critical,
            oxygen_status: ParameterStatus::Normal,
            feeding_status: FeedingStatus::Overdue,
            overall_health: OverallHealth::AtRisk,
            recommendations: to_string_vec(&[
                "Reduce temperature by 2°C",
                "Adjust pH to 7.2–7.5 range",
                "Administer emergency feeding",
            ]),
        },
        // Healthy clownfish tank
        "Tank-B2" => AnalysisResult {
            tank_id: tank_id.clone(),
            species_id: params.species_id.unwrap_or(2), // Clownfish
            timestamp: chrono::Utc::now().to_rfc3339(),
            temperature_status: ParameterStatus::Normal,
            ph_status: ParameterStatus::Normal,
            oxygen_status: ParameterStatus::Normal,
            feeding_status: FeedingStatus::Normal,
            overall_health: OverallHealth::Good,
            recommendations: to_string_vec(&[
                "Maintain current parameters",
                "Continue regular feeding schedule",
            ]),
        },
        // Tank showing early warnings
        "Tank-C3" => AnalysisResult {
            tank_id: tank_id.clone(),
            species_id: params.species_id.unwrap_or(3), // Blue Tang
            timestamp: chrono::Utc::now().to_rfc3339(),
            temperature_status: ParameterStatus::Warning,
            ph_status: ParameterStatus::Normal,
            oxygen_status: ParameterStatus::Warning,
            feeding_status: FeedingStatus::Overdue,
            overall_health: OverallHealth::AtRisk,
            recommendations: to_string_vec(&[
                "Reduce feeding frequency",
                "Perform partial water change",
            ]),
        },
        // Unknown / unregistered tank
        _ => AnalysisResult {
            tank_id: tank_id.clone(),
            species_id: params.species_id.unwrap_or(0),
            timestamp: chrono::Utc::now().to_rfc3339(),
            temperature_status: ParameterStatus::Unknown,
            ph_status: ParameterStatus::Unknown,
            oxygen_status: ParameterStatus::Unknown,
            feeding_status: FeedingStatus::Unknown,
            overall_health: OverallHealth::Unknown,
            recommendations: to_string_vec(&[
                "Verify tank ID",
                "Setup monitoring system",
            ]),
        },
    }
}
// ⚠️ END CHALLENGE CODE ⚠️
