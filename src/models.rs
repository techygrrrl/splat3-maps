use serde::Deserialize;

#[derive(Deserialize, Debug, Clone)]
#[serde(rename_all(deserialize = "camelCase"))]
pub struct Splat3Response {
    pub data: Splat3ResponseData,
}

#[derive(Deserialize, Debug, Clone)]
#[serde(rename_all(deserialize = "camelCase"))]
pub struct Splat3ResponseData {
    pub regular_schedules: RegularSchedules,
}

#[derive(Deserialize, Debug, Clone)]
#[serde(rename_all(deserialize = "camelCase"))]
pub struct RegularSchedules {
    pub nodes: Vec<MatchNode>,
}

#[derive(Deserialize, Debug, Clone)]
#[serde(rename_all(deserialize = "camelCase"))]
pub struct MatchNode {
    pub regular_match_setting: MatchSettings,
}

#[derive(Deserialize, Debug, Clone)]
#[serde(rename_all(deserialize = "camelCase"))]
pub struct MatchSettings {
    pub vs_stages: Vec<Stage>,
}

#[derive(Deserialize, Debug, Clone)]
#[serde(rename_all(deserialize = "camelCase"))]
pub struct Stage {
    pub name: String,
}
