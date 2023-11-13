use bevy::prelude::*;

pub const STAR_SPAWN_INTERVAL: f32 = 1.0;
pub const ENEMY_SPAWN_INTERVAL: f32 = 5.0;

#[derive(Resource, Default)]
pub struct Score {
    pub value: usize,
}

#[derive(Resource, Default, Debug)]
pub struct HighScores {
    pub scores: Vec<(String, usize)>,
}

#[derive(Resource)]
pub struct StarSpawnTimer {
    pub timer: Timer,
}

impl Default for StarSpawnTimer {
    fn default() -> Self {
        StarSpawnTimer {
            timer: Timer::from_seconds(STAR_SPAWN_INTERVAL, TimerMode::Repeating),
        }
    }
}

#[derive(Resource)]
pub struct EnemySpawnTimer {
    pub timer: Timer,
}

impl Default for EnemySpawnTimer {
    fn default() -> Self {
        EnemySpawnTimer {
            timer: Timer::from_seconds(ENEMY_SPAWN_INTERVAL, TimerMode::Repeating),
        }
    }
}
