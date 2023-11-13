use bevy::prelude::*;

pub const ENEMY_SPAWN_INTERVAL: f32 = 5.0;

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
