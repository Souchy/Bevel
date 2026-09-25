use bevy::prelude::*;
pub mod plugins;

#[derive(Resource, Default)]
pub struct UiEnabled;

#[derive(SystemSet, Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Sets {
    Logic,
    Ui,
}

fn main() {
    let headless = std::env::args().any(|arg| arg == "--headless");
    let mut app = App::new();

    if headless {
        app.add_plugins(MinimalPlugins);
        // Maybe limit to 10fps rather than unbound?
        // app.add_plugins(
        //     MinimalPlugins.set(bevy::app::ScheduleRunnerPlugin::run_loop(
        //         std::time::Duration::from_millis(100),
        //     )),
        // );
    } else {
        app.init_resource::<UiEnabled>()
            .add_plugins(DefaultPlugins)
            .add_systems(Startup, setup_ui.in_set(Sets::Ui));
    }

    app.add_plugins(plugins::FeaturesPlugins).run();
}

fn setup_ui(mut commands: Commands) {
    // commands.spawn(Camera2dBundle::default());
    commands.spawn(Camera2d);
}
