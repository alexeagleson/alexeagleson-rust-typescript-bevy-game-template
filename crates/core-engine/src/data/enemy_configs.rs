use bevy::prelude::Resource;
use serde::{Deserialize, Serialize};

use super::enemy_config::EnemyConfig;

#[derive(Debug, Deserialize, Serialize, Resource)]
pub struct EnemyConfigs {
    pub carrot: EnemyConfig,
}
