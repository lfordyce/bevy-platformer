use bevy_asset_loader::prelude::AssetCollection;
use bevy_ecs_ldtk::LdtkAsset;
use bevy::asset::{AssetServer, HandleUntyped};
use bevy::ecs::world::{Mut, World};
use bevy::prelude::{Handle};

#[derive(AssetCollection)]
pub struct GameAssets {
    #[asset(path = "levels/level.ldtk")]
    pub level: Handle<LdtkAsset>,
}