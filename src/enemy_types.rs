#[derive(Debug, Clone, Copy)]
pub enum EnemyType {
    Militia,
    HolyKnight,
}

impl EnemyType {
    pub fn health(&self) -> usize {
        match self {
            EnemyType::Militia => 50,
            EnemyType::HolyKnight => 100,
        }
    }

    pub fn velocity(&self) -> f32 {
        match self {
            EnemyType::Militia => 350.,
            EnemyType::HolyKnight => 300.,
        }
    }

    pub fn sprite(&self) -> &str {
        match self {
            EnemyType::Militia => "sprites/enemies/ball_red_large.png",
            EnemyType::HolyKnight => "sprites/enemies/ball_red_large.png",
        }
    }

    pub fn hit_circle(&self) -> f32 {
        match self {
            EnemyType::Militia => 50.,
            EnemyType::HolyKnight => 0.,
        }
    }
}