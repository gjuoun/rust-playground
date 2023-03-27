
use bevy::{
    prelude::{Query, Res, ResMut, Resource, With},
    time::{Time, Timer},
};

use super::{Name, Person};

#[derive(Resource)]
pub struct GreetTimer(pub Timer);

