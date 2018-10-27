use std::fmt;

pub struct Player {
    name: String,
    score: u16,
}

impl Player {
    pub fn new(player_name: &str) -> Player {
        Player {
            name: player_name.to_string(),
            score: 0,
        }
    }

    pub fn on_board(&self) -> bool {
        self.score >= 500
    }
    
    pub fn score(&self) -> u16 {
		self.score
	}
	
	pub fn increment_score(&mut self, amount: u16) {
		self.score += amount;
	}
}

impl fmt::Display for Player {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.name)
    }
}
