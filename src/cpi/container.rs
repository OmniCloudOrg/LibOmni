use std::collections::HashMap;

use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq,)]
#[serde(rename_all = "snake_case")]
pub enum CpiCommandType {
    #[serde(rename = "create_container")]
    CreateContainer {
        image: String,
        name: String,
        ports: Vec<String>,
        env: HashMap<String, String>,
    },
    #[serde(rename = "delete_container")]
    DeleteContainer {
        name: String,
    },
    #[serde(rename = "start_container")]
    StartContainer {
        name: String,
    },
    #[serde(rename = "stop_container")]
    StopContainer {
        name: String,
    },
    #[serde(rename = "restart_container")]
    RestartContainer {
        name: String,
    },
    #[serde(rename = "inspect_container")]
    InspectContainer {
        name: String,
    },
    #[serde(rename = "list_containers")]
    ListContainers,
}

impl ToString for CpiCommandType {
    fn to_string(&self) -> String {
        match self {
            CpiCommandType::CreateContainer { .. } => "create_container".to_string(),
            CpiCommandType::DeleteContainer { .. } => "delete_container".to_string(),
            CpiCommandType::StartContainer { .. } => "start_container".to_string(),
            CpiCommandType::StopContainer { .. } => "stop_container".to_string(),
            CpiCommandType::RestartContainer { .. } => "restart_container".to_string(),
            CpiCommandType::InspectContainer { .. } => "inspect_container".to_string(),
            CpiCommandType::ListContainers => "list_containers".to_string(),
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Container {
    pub id: String,
    pub state: String,
    pub image: String,
    pub name: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ContainerList {
    pub containers: Vec<Container>,
}
pub struct CpiCommand {
    pub config: String,
}
#[allow(dead_code)]
pub struct CpiApi {
    cmd: CpiCommand,
}