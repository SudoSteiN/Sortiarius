use serde::{Deserialize, Serialize};

use crate::units::UnitSystem;

/// Version of the .wfp file format
pub const FORMAT_VERSION: u32 = 1;

/// Complete project state that can be serialized to .wfp JSON
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Project {
    pub format_version: u32,
    pub name: String,
    pub description: String,
    pub created_at: String,
    pub updated_at: String,
    pub unit_system: UnitSystem,
    pub boards: Vec<SavedBoard>,
    pub joints: Vec<SavedJoint>,
    pub finishes: Vec<SavedFinish>,
    pub camera: Option<SavedCamera>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SavedBoard {
    pub id: String,
    pub label: String,
    pub position: [f64; 3],
    pub rotation: [f64; 3],
    pub dimensions: [f64; 3],
    pub parent_id: Option<String>,
    pub lumber_type: String,
    pub species_id: Option<String>,
    pub finish_id: Option<String>,
    pub grain_direction: String,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SavedJoint {
    pub id: String,
    pub joint_type: String,
    pub board_a_id: String,
    pub board_b_id: String,
    pub face_a: String,
    pub face_b: String,
    pub offset: [f64; 3],
    pub parameters: serde_json::Value,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SavedFinish {
    pub id: String,
    pub finish_type: String,
    pub name: String,
    pub color: [f32; 3],
    pub opacity: f32,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SavedCamera {
    pub eye: [f32; 3],
    pub target: [f32; 3],
    pub fov_y: f32,
}

/// Get the current time as an ISO 8601 string using std::time::SystemTime.
fn now_iso8601() -> String {
    use std::time::{SystemTime, UNIX_EPOCH};
    let duration = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap_or_default();
    let secs = duration.as_secs();

    // Convert unix timestamp to date-time components
    let days = secs / 86400;
    let time_of_day = secs % 86400;
    let hours = time_of_day / 3600;
    let minutes = (time_of_day % 3600) / 60;
    let seconds = time_of_day % 60;

    // Calculate year/month/day from days since epoch (1970-01-01)
    let (year, month, day) = days_to_ymd(days);

    format!(
        "{:04}-{:02}-{:02}T{:02}:{:02}:{:02}Z",
        year, month, day, hours, minutes, seconds
    )
}

fn days_to_ymd(mut days: u64) -> (u64, u64, u64) {
    let mut year = 1970;
    loop {
        let days_in_year = if is_leap(year) { 366 } else { 365 };
        if days < days_in_year {
            break;
        }
        days -= days_in_year;
        year += 1;
    }

    let month_days: [u64; 12] = if is_leap(year) {
        [31, 29, 31, 30, 31, 30, 31, 31, 30, 31, 30, 31]
    } else {
        [31, 28, 31, 30, 31, 30, 31, 31, 30, 31, 30, 31]
    };

    let mut month = 1;
    for &md in &month_days {
        if days < md {
            break;
        }
        days -= md;
        month += 1;
    }

    (year, month, days + 1)
}

fn is_leap(year: u64) -> bool {
    (year % 4 == 0 && year % 100 != 0) || year % 400 == 0
}

impl Project {
    /// Create a new empty project
    pub fn new(name: String) -> Self {
        let now = now_iso8601();
        Self {
            format_version: FORMAT_VERSION,
            name,
            description: String::new(),
            created_at: now.clone(),
            updated_at: now,
            unit_system: UnitSystem::default(),
            boards: Vec::new(),
            joints: Vec::new(),
            finishes: Vec::new(),
            camera: None,
        }
    }

    /// Serialize to JSON string (.wfp format)
    pub fn to_json(&self) -> Result<String, serde_json::Error> {
        serde_json::to_string_pretty(self)
    }

    /// Deserialize from JSON string
    pub fn from_json(json: &str) -> Result<Self, ProjectError> {
        let project: Self =
            serde_json::from_str(json).map_err(|e| ProjectError::InvalidJson(e.to_string()))?;

        if project.format_version > FORMAT_VERSION {
            return Err(ProjectError::UnsupportedVersion(project.format_version));
        }

        if project.name.is_empty() {
            return Err(ProjectError::MissingRequiredField("name".to_string()));
        }

        Ok(project)
    }

    /// Update the updated_at timestamp
    pub fn touch(&mut self) {
        self.updated_at = now_iso8601();
    }

    /// Validate project integrity (all joint references exist, etc.)
    pub fn validate(&self) -> Vec<ProjectWarning> {
        let mut warnings = Vec::new();

        let board_ids: std::collections::HashSet<&str> =
            self.boards.iter().map(|b| b.id.as_str()).collect();

        for joint in &self.joints {
            if !board_ids.contains(joint.board_a_id.as_str()) {
                warnings.push(ProjectWarning {
                    message: format!(
                        "Joint '{}' references non-existent board_a '{}'",
                        joint.id, joint.board_a_id
                    ),
                    severity: WarningSeverity::Error,
                });
            }
            if !board_ids.contains(joint.board_b_id.as_str()) {
                warnings.push(ProjectWarning {
                    message: format!(
                        "Joint '{}' references non-existent board_b '{}'",
                        joint.id, joint.board_b_id
                    ),
                    severity: WarningSeverity::Error,
                });
            }
        }

        // Check for boards referencing non-existent parents
        for board in &self.boards {
            if let Some(ref parent_id) = board.parent_id {
                if !board_ids.contains(parent_id.as_str()) {
                    warnings.push(ProjectWarning {
                        message: format!(
                            "Board '{}' references non-existent parent '{}'",
                            board.id, parent_id
                        ),
                        severity: WarningSeverity::Warning,
                    });
                }
            }
        }

        // Check for boards referencing non-existent finishes
        let finish_ids: std::collections::HashSet<&str> =
            self.finishes.iter().map(|f| f.id.as_str()).collect();
        for board in &self.boards {
            if let Some(ref finish_id) = board.finish_id {
                if !finish_ids.contains(finish_id.as_str()) {
                    warnings.push(ProjectWarning {
                        message: format!(
                            "Board '{}' references non-existent finish '{}'",
                            board.id, finish_id
                        ),
                        severity: WarningSeverity::Warning,
                    });
                }
            }
        }

        warnings
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum ProjectError {
    InvalidJson(String),
    UnsupportedVersion(u32),
    MissingRequiredField(String),
}

impl std::fmt::Display for ProjectError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ProjectError::InvalidJson(msg) => write!(f, "Invalid JSON: {}", msg),
            ProjectError::UnsupportedVersion(v) => {
                write!(
                    f,
                    "Unsupported format version: {} (max: {})",
                    v, FORMAT_VERSION
                )
            }
            ProjectError::MissingRequiredField(field) => {
                write!(f, "Missing required field: {}", field)
            }
        }
    }
}

impl std::error::Error for ProjectError {}

#[derive(Clone, Debug)]
pub struct ProjectWarning {
    pub message: String,
    pub severity: WarningSeverity,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum WarningSeverity {
    Info,
    Warning,
    Error,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_project() {
        let project = Project::new("My Workbench".to_string());
        assert_eq!(project.name, "My Workbench");
        assert_eq!(project.format_version, FORMAT_VERSION);
        assert_eq!(project.unit_system, UnitSystem::Imperial);
        assert!(project.boards.is_empty());
        assert!(project.joints.is_empty());
        assert!(project.finishes.is_empty());
        assert!(project.camera.is_none());
        assert!(project.description.is_empty());
        assert!(!project.created_at.is_empty());
        assert!(!project.updated_at.is_empty());
    }

    #[test]
    fn test_roundtrip_json() {
        let mut project = Project::new("Roundtrip Test".to_string());
        project.description = "A test project".to_string();
        project.unit_system = UnitSystem::Metric;
        project.boards.push(SavedBoard {
            id: "board-1".to_string(),
            label: "2x4 Leg".to_string(),
            position: [0.0, 0.0, 0.0],
            rotation: [0.0, 0.0, 0.0],
            dimensions: [1.5, 3.5, 30.0],
            parent_id: None,
            lumber_type: "Standard:2x4".to_string(),
            species_id: Some("pine".to_string()),
            finish_id: None,
            grain_direction: "AlongLength".to_string(),
        });
        project.camera = Some(SavedCamera {
            eye: [10.0, 10.0, 10.0],
            target: [0.0, 0.0, 0.0],
            fov_y: 45.0,
        });

        let json = project.to_json().expect("serialize");
        let restored = Project::from_json(&json).expect("deserialize");

        assert_eq!(restored.name, "Roundtrip Test");
        assert_eq!(restored.description, "A test project");
        assert_eq!(restored.unit_system, UnitSystem::Metric);
        assert_eq!(restored.format_version, FORMAT_VERSION);
        assert_eq!(restored.boards.len(), 1);
        assert_eq!(restored.boards[0], project.boards[0]);
        assert_eq!(restored.camera, project.camera);
    }

    #[test]
    fn test_invalid_json() {
        let result = Project::from_json("not valid json at all");
        assert!(result.is_err());
        match result.unwrap_err() {
            ProjectError::InvalidJson(_) => {}
            other => panic!("Expected InvalidJson, got: {:?}", other),
        }
    }

    #[test]
    fn test_validate_missing_board_ref() {
        let mut project = Project::new("Validation Test".to_string());
        project.boards.push(SavedBoard {
            id: "board-a".to_string(),
            label: "Left Leg".to_string(),
            position: [0.0, 0.0, 0.0],
            rotation: [0.0, 0.0, 0.0],
            dimensions: [1.5, 3.5, 30.0],
            parent_id: None,
            lumber_type: "Standard:2x4".to_string(),
            species_id: None,
            finish_id: None,
            grain_direction: "AlongLength".to_string(),
        });
        project.joints.push(SavedJoint {
            id: "joint-1".to_string(),
            joint_type: "mortise_tenon".to_string(),
            board_a_id: "board-a".to_string(),
            board_b_id: "board-nonexistent".to_string(),
            face_a: "end".to_string(),
            face_b: "face".to_string(),
            offset: [0.0, 0.0, 0.0],
            parameters: serde_json::json!({}),
        });

        let warnings = project.validate();
        assert!(!warnings.is_empty());
        assert!(warnings
            .iter()
            .any(|w| w.message.contains("board-nonexistent")));
        assert!(warnings
            .iter()
            .any(|w| w.severity == WarningSeverity::Error));
    }

    #[test]
    fn test_unsupported_version() {
        let json = r#"{
            "format_version": 999,
            "name": "Future Project",
            "description": "",
            "created_at": "2026-01-01T00:00:00Z",
            "updated_at": "2026-01-01T00:00:00Z",
            "unit_system": "Imperial",
            "boards": [],
            "joints": [],
            "finishes": [],
            "camera": null
        }"#;
        let result = Project::from_json(json);
        assert!(result.is_err());
        match result.unwrap_err() {
            ProjectError::UnsupportedVersion(999) => {}
            other => panic!("Expected UnsupportedVersion(999), got: {:?}", other),
        }
    }

    #[test]
    fn test_touch_updates_timestamp() {
        let mut project = Project::new("Touch Test".to_string());
        let original = project.updated_at.clone();
        // Touch should update the timestamp (may be same if called within same second)
        project.touch();
        // Just verify it's a valid timestamp string
        assert!(!project.updated_at.is_empty());
        assert!(project.updated_at.ends_with('Z'));
        // created_at should remain unchanged
        assert_eq!(project.created_at, original);
    }
}
