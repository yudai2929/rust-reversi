use crate::domains::player_color::PlayerColor;
pub struct Player {
    name: String,
    color: PlayerColor,
    is_ai: bool,
}

impl Player {
    pub fn new(name: String, color: PlayerColor, is_ai: bool) -> Player {
        Player { name, color, is_ai }
    }

    pub fn get_name(&self) -> &String {
        &self.name
    }

    pub fn get_color(&self) -> &PlayerColor {
        &self.color
    }
}
