#[derive(Debug)]
pub enum BulletType {
    RedBlob,
    Arrow,
}

impl BulletType {
    pub fn velocity(&self) -> f32 {
        match self {
            BulletType::RedBlob => 650.,
            BulletType::Arrow => 600.,
        }
    }
    pub fn sprite(&self) -> &str {
        match self {
            BulletType::RedBlob => "",
            BulletType::Arrow => "x",
        }
    }
}