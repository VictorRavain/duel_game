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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_player_creation() {
        let player = Player::new("Michel", 100, 50, 75);

        assert_eq!(player.name, "Michel");
        assert_eq!(player.vitality, 100);
        assert_eq!(player.speed, 50);
        assert_eq!(player.strength, 75);
    }

    #[test]
    fn test_apply_poison_type_1_reduces_speed() {
        let mut player = Player::new("Speedy", 100, 50, 75);
        player.apply_poison(1);
        assert_eq!(player.speed, 45);
        assert_eq!(player.strength, 75); // should not be changed
    }

    #[test]
    fn test_apply_poison_type_2_reduces_strength() {
        let mut player = Player::new("Strongman", 100, 50, 75);
        player.apply_poison(2);
        assert_eq!(player.strength, 70);
        assert_eq!(player.speed, 50); // should not be changed
    }

    #[test]
    fn test_apply_poison_invalid_type_does_nothing() {
        let mut player = Player::new("Immune", 100, 50, 75);
        player.apply_poison(99); // invalid type
        assert_eq!(player.speed, 50);
        assert_eq!(player.strength, 75);
    }

    #[test]
    fn test_poison_does_not_underflow_speed() {
        let mut player = Player::new("LowSpeed", 100, 3, 75);
        player.apply_poison(1);
        assert_eq!(player.speed, 0); // should not go below 0
    }

    #[test]
    fn test_poison_does_not_underflow_strength() {
        let mut player = Player::new("Weak", 100, 50, 4);
        player.apply_poison(2);
        assert_eq!(player.strength, 0); // should not go below 0
    }
}
