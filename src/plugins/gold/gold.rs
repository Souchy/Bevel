use bevy::prelude::*;

// 1. Define the Global Resource Data
use crate::{Sets, UiEnabled, plugins::gold::gold_ui::GoldUiPlugin};

#[derive(Resource, Default)]
pub struct GoldBank {
    pub amount: f64,
}

#[derive(Component)]
pub struct GoldGenerator {
    pub base_production_rate: f64,
    pub level: u32,
}

pub struct GoldPlugin;
impl Plugin for GoldPlugin {
    fn build(&self, app: &mut App) {
        // app.add_plugins(GoldLogicPlugin);
        app.init_resource::<GoldBank>()
            .add_systems(Startup, startup)
            .add_systems(Update, generate_gold_system.in_set(Sets::Logic));

        if app.world().contains_resource::<UiEnabled>() {
            app.add_plugins(GoldUiPlugin);
        }
    }
}

fn startup(mut commands: Commands) {
    commands.spawn(GoldGenerator {
        base_production_rate: 1.0,
        level: 1,
    });
}

// 4. Write the system that runs every frame
fn generate_gold_system(
    time: Res<Time>,
    mut gold_bank: ResMut<GoldBank>,
    query: Query<&GoldGenerator>,
) {
    let mut total_generation = 0.0;
    for generator in query.iter() {
        total_generation += generator.base_production_rate * generator.level as f64;
    }

    // Multiply by delta_seconds for smooth frame-independent production
    gold_bank.amount += total_generation * time.delta_secs_f64();

    println!("Total gold: {}", gold_bank.amount);
}
