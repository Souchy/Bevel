use crate::plugins::gold;
use bevy::{app::PluginGroupBuilder, prelude::*};

pub struct FeaturesPlugins;

impl PluginGroup for FeaturesPlugins {
    fn build(self) -> PluginGroupBuilder {
        PluginGroupBuilder::start::<Self>().add(gold::GoldPlugin)
    }
}
