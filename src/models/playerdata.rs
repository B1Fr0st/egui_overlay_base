use crate::models::fvector::FVector;
#[derive(Default)]
pub struct PlayerData {
    pub player: usize,
    pub pawn: usize,
    pub controller: usize,
    pub character: usize,
    pub character_movement: usize,
    pub last_update_vector: FVector
}