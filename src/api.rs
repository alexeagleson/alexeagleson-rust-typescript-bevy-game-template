use ae_direction::BodyRelative;
use ae_position::Position;
use bevy::prelude::Component;
use serde::{Deserialize, Serialize};
use typeshare::typeshare;

#[typeshare]
#[derive(Clone, Copy, Serialize, Debug, PartialEq, Eq)]
pub struct UserId {
    pub id: i32,
}

#[typeshare]
#[derive(Clone, Copy, Serialize, Debug, PartialEq, Eq)]
pub struct EntityIndex {
    pub index: u32,
}

#[typeshare]
#[derive(Serialize, Debug)]
#[serde(rename_all = "camelCase")]
/// Information about a specific entity's current position
pub struct EntityPosition {
    pub entity_index: EntityIndex,
    pub pos: Position,
}

#[typeshare]
#[derive(Serialize, Debug)]
#[serde(rename_all = "camelCase")]
/// Information about a entity to render
pub struct EntityRenderData {
    pub entity_position: EntityPosition,
    pub sprite: SpriteTexture,
}

#[typeshare]
#[derive(Serialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
/// Information about an entity
pub struct EntityData {
    pub name: String,
    pub blocks_light: bool,
    pub visible_to_player: bool,
}

#[typeshare]
#[derive(Serialize, Debug)]
#[serde(rename_all = "camelCase")]
/// A single entry in the game log
pub struct LogMessage(pub String);

#[typeshare]
#[derive(Component, Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
/// A sprite to render that represents a visible entity
pub enum SpriteTexture {
    Bunny,
    Carrot,
    Wall,
}

#[typeshare]
#[derive(Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase", tag = "type", content = "content")]
/// An input interaction from the client
pub enum ClientMessage {
    TileHover(Position),
    TileClick(Position),
    Initialize,
    Keypress(BodyRelative),
    Disconnect,
}

#[typeshare]
#[derive(Serialize, Debug)]
#[serde(rename_all = "camelCase", tag = "type", content = "content")]
/// Communicates information about the active game to one client
pub enum ServerMessageSingleClient {
    TileHover(Option<EntityData>),
    AllEntityRenderData(Vec<EntityRenderData>), // For ping/timeout
}

#[typeshare]
#[derive(Serialize, Debug)]
#[serde(rename_all = "camelCase", tag = "type", content = "content")]
/// Communicates information about the active game to one client
pub enum ServerMessageAllClients {
    NewEntity(EntityRenderData), // could be used as better way on player join
    RemovedEntity(EntityIndex),
    AllEntityRenderData(Vec<EntityRenderData>),
    EntityPositionChange(EntityPosition),
    TileClick(LogMessage),
    MoveCount(i32),
}
