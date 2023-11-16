use bevy::prelude::*;

#[derive(Resource, Default)]
pub struct Score {
    pub value: usize,
}

#[derive(Resource, Default, Debug)]
pub struct HighScores {
    pub scores: Vec<(String, usize)>,
}
