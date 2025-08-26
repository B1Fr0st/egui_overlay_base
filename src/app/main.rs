use crate::app::app::App;

#[derive(Debug, Clone,Default)]
pub struct Offsets{
    pub uworld: usize,
    pub persistent_level: usize,
    pub world_settings: usize,
    pub game_instance: usize,
    pub local_players: usize,
    pub local_player: usize,
    pub player_controller: usize,
    pub camera_manager: usize,
}

pub struct AActorData{
    pub actor: usize,
    pub root: usize,
    pub location: crate::models::fvector::FVector,
    pub objectid: Option<i32>,
    pub name: Option<String>,
}

impl App {
    pub fn update_offsets(&mut self) {
        self.offsets.uworld = self.read::<usize>(self.game_proc.process_base_address + self.dsapi.get_offset("OFFSET_GWORLD").unwrap() as usize).unwrap();
        self.offsets.persistent_level = self.read_offset(self.offsets.uworld, "UWorld", "PersistentLevel").unwrap();
        self.offsets.world_settings = self.read_offset(self.offsets.persistent_level, "ULevel", "WorldSettings").unwrap();
        self.offsets.game_instance = self.read_offset(self.offsets.uworld, "UWorld","OwningGameInstance").unwrap();
        self.offsets.local_players = self.read_offset(self.offsets.game_instance, "UGameInstance", "LocalPlayers").unwrap();
        self.offsets.local_player = self.read(self.offsets.local_players).unwrap();
        self.offsets.player_controller = self.read_offset(self.offsets.local_player, "UPlayer", "PlayerController").unwrap();
        self.offsets.camera_manager = self.read_offset(self.offsets.player_controller, "APlayerController", "PlayerCameraManager").unwrap();
    }
}