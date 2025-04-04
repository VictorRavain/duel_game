#[derive(Debug)]
pub struct Player {
    pub name: String,
    pub vitality: i32,
    pub speed: u64,
    pub strength: i32,
}

impl Player {
    pub fn new(name: &str, vitality: i32, speed: u64, strength: i32) -> Self {
        Self {
            name: name.to_string(),
            vitality,
            speed,
            strength,
        }
    }

    pub fn apply_poison(&mut self, poison: u8) {
        match poison {
            1 => self.speed = self.speed.saturating_sub(5),
            2 => self.strength = self.strength.saturating_sub(5),
            _ => {}
        }
    }
}
