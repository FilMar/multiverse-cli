//! Data models for Multiverse CLI

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct World {
    pub id: i32,
    pub name: String,
    pub description: Option<String>,
    pub visual_identity: Option<HashMap<String, String>>,
    pub config: Option<HashMap<String, serde_json::Value>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Series {
    pub id: i32,
    pub world_id: i32,
    pub name: String,
    pub category: SeriesCategory,
    pub series_type: String,
    pub narrator: Option<String>,
    pub public_signature: Option<String>,
    pub created_at: chrono::DateTime<chrono::Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum SeriesCategory {
    Diary,
    Extra,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Episode {
    pub id: i32,
    pub series_id: i32,
    pub number: i32,
    pub title: String,
    pub file_path: String,
    pub status: EpisodeStatus,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub modified_at: chrono::DateTime<chrono::Utc>,
    pub word_count: Option<i32>,
    pub reading_time_minutes: Option<i32>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum EpisodeStatus {
    Draft,
    InProduzione,
    Ready,
    Pubblicato,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Character {
    pub id: i32,
    pub world_id: i32,
    pub name: String,
    pub race: Option<String>,
    pub has_magic_abilities: bool,
    pub abilities: Vec<String>,
    pub limitations: Vec<String>,
    pub age: Option<i32>,
    pub origin: Option<String>,
    pub profession: Option<String>,
    pub notes: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Location {
    pub id: i32,
    pub world_id: i32,
    pub name: String,
    pub location_type: String,
    pub parent_location_id: Option<i32>,
    pub characteristics: Vec<String>,
    pub status: String,
    pub notes: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TimelineEvent {
    pub id: i32,
    pub episode_id: i32,
    pub event_description: String,
    pub event_date: Option<String>,
    pub event_order: Option<i32>,
    pub event_type: EventType,
    pub character_id: Option<i32>,
    pub location_id: Option<i32>,
    pub extracted_automatically: bool,
    pub verified_by_user: bool,
    pub notes: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum EventType {
    Major,
    Minor,
    Reference,
}