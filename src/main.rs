use bevy::prelude::*;
use bevy_panorbit_camera::PanOrbitCameraPlugin;
use iyes_perf_ui::prelude::*;

mod components;
mod systems;

use systems::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(bevy::diagnostic::FrameTimeDiagnosticsPlugin)
        .add_plugins(bevy::diagnostic::EntityCountDiagnosticsPlugin)
        .add_plugins(bevy::diagnostic::SystemInformationDiagnosticsPlugin)
        .add_plugins(PerfUiPlugin)
        .add_plugins(PanOrbitCameraPlugin)
        .add_systems(Startup, (setup::camera, setup::ui, setup::scene))
        .run();
}
