#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
pub enum EnemyType {
    Militia,
    HolyKnight,
}

impl EnemyType {
    pub fn health(&self) -> usize {
        match self {
            EnemyType::Militia => 40,
            EnemyType::HolyKnight => 5,
        }
    }

    pub fn velocity(&self) -> f32 {
        match self {
            EnemyType::Militia => 350.,
            EnemyType::HolyKnight => 500.,
        }
    }

    pub fn sprite(&self) -> &'static str {
        match self {
            EnemyType::Militia => "sprites/enemies/ball_red_large.png",
            EnemyType::HolyKnight => "sprites/enemies/ball_blue_large.png",
        }
    }

    pub fn hit_circle(&self) -> f32 {
        match self {
            EnemyType::Militia => 50.,
            EnemyType::HolyKnight => 40.,
        }
    }

    pub fn bounty(&self) -> usize {
        match self {
            EnemyType::Militia => 10,
            EnemyType::HolyKnight => 15,
        }
    }
}