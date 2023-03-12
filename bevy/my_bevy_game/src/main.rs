use bevy::prelude::*;

use my_bevy_game::plugin::HelloPlugin;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugin(HelloPlugin)
        .run();
}
