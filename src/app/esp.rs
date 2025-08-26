use egui::{Align2, Color32};

use crate::{app::app::App, math::{calculateaim::{calculate_aim, normalize_angle, smooth_angle}, w2s::world2screen}, models::{cameramodels::FMinimalViewInfo, frotator::FRotator, fvector::FVector}};

impl App{
    pub fn esp(&mut self, painter: egui::Painter){
        let pov = if let Some(res) = self.read::<FMinimalViewInfo>(self.offsets.camera_manager + self.dsapi.get_member_offset_unchecked("APlayerCameraManager", "CameraCachePrivate") as usize + self.dsapi.get_member_offset_unchecked("FCameraCacheEntry", "POV")){res} else {return;};
        for i in self.aactors.lock().unwrap().iter(){
            if pov.location.distance(i.location) >= self.config.esp_distance as f64 {
                continue; //skip if too far
            }
            let screen_pos = world2screen(i.location, pov, self.window_size);
            if self.config.esp_enabled && (screen_pos.x != 0.0 || screen_pos.y != 0.0) {
                painter.text(
                    screen_pos.to_egui(),
                    Align2::RIGHT_CENTER,
                    i.name.as_ref().unwrap_or(&"Unknown".to_string()),
                    egui::FontId::default(),
                    Color32::WHITE
                );
                if i.name.as_ref().unwrap_or(&"".to_string()).contains("BP_Zombie"){
                    self.draw_centered_box(&painter, 
                        self.config.esp_box_type.clone(),
                        screen_pos,
                        100000.0 / pov.location.distance(i.location) as f32,
                        200000.0 / pov.location.distance(i.location) as f32,
                        self.config.esp_color
                    );
                }
            }
        }
    }
    pub fn read_aactors(&mut self) {
        self.update_offsets();
        let actors: usize = self.read(self.offsets.persistent_level + 0x98).unwrap();
        let actors_size:i32 = self.read(self.offsets.persistent_level + 0xA0).unwrap();
        let mut new_aactors = Vec::new();
        for i in 0..actors_size{
            let actor = if let Some(res) = self.read(actors + i as usize * 0x8){res} else {continue};
            let root:usize = if let Some(res) = self.read_offset(actor, "AActor", "RootComponent"){res} else {continue};
            if root == 0 {continue;}
            let location: FVector = if let Some(res) =  self.read_offset(root, "USceneComponent", "RelativeLocation"){res} else {continue};
            let objectid:i32 = self.read(actor + 0x18).unwrap();

            //convert objectid to string
            let name_str = if self.fname_cache.contains_key(&objectid) {
                let name_str = self.fname_cache.get(&objectid).unwrap();
                if name_str.is_empty() || name_str == "None" {
                    continue;
                }
                name_str.clone()
            } else {
                let chunkoffset = (objectid >> 16) as u32;
                let nameoffset = objectid as u16;
                let name_pool_chunk_ptr = self.read::<u64>(self.game_proc.process_base_address + self.dsapi.get_offset("OFFSET_GNAMES").unwrap() as usize + 8 * (chunkoffset as usize + 2)).unwrap_or(0) as usize;
                let entry_offset = name_pool_chunk_ptr + 2 * nameoffset as usize;
                let name_length_raw = self.read::<u16>(entry_offset).unwrap_or(0);
                let name_length = (name_length_raw >> 6) as usize;
                if name_length == 0 {
                    continue;
                }
                let safe_length = name_length.min(1024);
                let mut name_bytes = vec![0u8; safe_length];
                if !self.game_proc.read_bytes(entry_offset + 2, name_bytes.as_mut_ptr(), safe_length) {
                    continue;
                }
                let name_str = match std::str::from_utf8(&name_bytes) {
                    Ok(s) => s.trim_end_matches('\0'),
                    Err(_) => continue,
                };
                if name_str.is_empty() || name_str == "None" {
                    continue;
                }
                self.fname_cache.insert(objectid, name_str.to_string());
                name_str.to_string()
            };

            new_aactors.push(crate::app::main::AActorData{
                actor,
                root,
                location,
                objectid: Some(objectid),
                name: Some(name_str),
            });
        }
        *self.aactors.lock().unwrap() = new_aactors;

    }
    pub fn apply_aimbot(&mut self){
        let pov = if let Some(res) = self.read::<FMinimalViewInfo>(self.offsets.camera_manager + self.dsapi.get_member_offset_unchecked("APlayerCameraManager", "CameraCachePrivate") as usize + self.dsapi.get_member_offset_unchecked("FCameraCacheEntry", "POV")){res} else {return;};
        let mut best_target = FVector {
            x: f64::MAX,
            y: f64::MAX,
            z: f64::MAX,
        };
        for i in self.aactors.lock().unwrap().iter(){
            if pov.location.distance(i.location) >= self.config.esp_distance as f64 {
                continue; //skip if too far
            }
            if i.name.as_ref().unwrap_or(&"".to_string()).contains("BP_Zombie"){
                if !i.name.as_ref().unwrap_or(&"".to_string()).contains("Crawler"){
                    let mut head_location = i.location;
                    head_location.z += 50.0;
                    if pov.location.distance(head_location) < best_target.distance(pov.location) {
                        best_target = head_location;
                    }
                } else {
                    if pov.location.distance(i.location) < best_target.distance(pov.location) {
                        best_target = i.location;
                    }
                }
            }
        }
        if best_target.x == f64::MAX {
            return;
        }
        if self.config.aim_enabled && (self.config.aim_key.is_some() && self.keys.contains(&self.config.aim_key.unwrap())) {
            let aim_angles = calculate_aim(&pov.location, &best_target, 0.0);
            let aim_diff = aim_angles - pov.rotation;
            let aim = FRotator{
                pitch: smooth_angle(pov.rotation.pitch, aim_angles.pitch, self.config.aim_smoothness as f64 * 20.0),
                yaw: normalize_angle(smooth_angle(pov.rotation.yaw, aim_angles.yaw, self.config.aim_smoothness as f64 * 20.0)),
                roll: 0.0,
            };
            self.write(self.offsets.player_controller + self.dsapi.get_member_offset_unchecked("AController", "ControlRotation") as usize, aim);
        }
    }
    pub fn mock_esp(&mut self, painter: egui::Painter){
        
        self.update_offsets();
        let actors: usize = self.read(self.offsets.persistent_level + 0x98).unwrap();
        let actors_size:i32 = self.read(self.offsets.persistent_level + 0xA0).unwrap();

        // let camera_location = if let Some(res) = self.read::<FVector>(self.offsets.camera_manager + self.dsapi.get_member_offset_unchecked("APlayerCameraManager", "CameraCachePrivate") as usize + self.dsapi.get_member_offset_unchecked("FCameraCacheEntry", "POV") as usize + self.dsapi.get_member_offset_unchecked("FMinimalViewInfo", "Location")){res} else {return;};
        // let camera_rotation = if let Some(res) = self.read::<FRotator>(self.offsets.camera_manager + self.dsapi.get_member_offset_unchecked("APlayerCameraManager", "CameraCachePrivate") as usize + self.dsapi.get_member_offset_unchecked("FCameraCacheEntry", "POV") as usize + self.dsapi.get_member_offset_unchecked("FMinimalViewInfo", "Rotation")){res} else {return;};
        // let fov = if let Some(res) = self.read::<f32>(self.offsets.camera_manager + self.dsapi.get_member_offset_unchecked("APlayerCameraManager", "CameraCachePrivate") as usize + self.dsapi.get_member_offset_unchecked("FCameraCacheEntry", "POV") as usize + self.dsapi.get_member_offset_unchecked("FMinimalViewInfo", "FOV")){res} else {return;};
        let pov = if let Some(res) = self.read::<FMinimalViewInfo>(self.offsets.camera_manager + self.dsapi.get_member_offset_unchecked("APlayerCameraManager", "CameraCachePrivate") as usize + self.dsapi.get_member_offset_unchecked("FCameraCacheEntry", "POV")){res} else {return;};


        self.write(self.offsets.world_settings + self.dsapi.get_member_offset_unchecked("AWorldSettings", "TimeDilation") as usize, self.config.time_dilation);


        let mut best_target = FVector {
            x: f64::MAX,
            y: f64::MAX,
            z: f64::MAX,
        };
        for i in 0..actors_size{
            let actor = if let Some(res) = self.read(actors + i as usize * 0x8){res} else {continue};
            let root:usize = if let Some(res) = self.read_offset(actor, "AActor", "RootComponent"){res} else {continue};
            if root == 0 {continue;}
            let mut location: FVector = if let Some(res) =  self.read_offset(root, "USceneComponent", "RelativeLocation"){res} else {continue};

            if pov.location.distance(location) >= self.config.esp_distance as f64 {
                continue; //skip if too far
            }
            
            let objectid:i32 = self.read(actor + 0x18).unwrap();


            //convert objectid to string
            let name_str = if self.fname_cache.contains_key(&objectid) {
                let name_str = self.fname_cache.get(&objectid).unwrap();
                if name_str.is_empty() || name_str == "None" {
                    continue;
                }
                name_str.clone()
            } else {
                let chunkoffset = (objectid >> 16) as u32;
                let nameoffset = objectid as u16;
                let name_pool_chunk_ptr = self.read::<u64>(self.game_proc.process_base_address + self.dsapi.get_offset("OFFSET_GNAMES").unwrap() as usize + 8 * (chunkoffset as usize + 2)).unwrap_or(0) as usize;
                let entry_offset = name_pool_chunk_ptr + 2 * nameoffset as usize;
                let name_length_raw = self.read::<u16>(entry_offset).unwrap_or(0);
                let name_length = (name_length_raw >> 6) as usize;
                if name_length == 0 {
                    continue;
                }
                let safe_length = name_length.min(1024);
                let mut name_bytes = vec![0u8; safe_length];
                if !self.game_proc.read_bytes(entry_offset + 2, name_bytes.as_mut_ptr(), safe_length) {
                    continue;
                }
                let name_str = match std::str::from_utf8(&name_bytes) {
                    Ok(s) => s.trim_end_matches('\0'),
                    Err(_) => continue,
                };
                if name_str.is_empty() || name_str == "None" {
                    continue;
                }
                self.fname_cache.insert(objectid, name_str.to_string());
                name_str.to_string()
            };

            
            let screen_pos = world2screen(location, pov, self.window_size);
            
            if self.config.esp_enabled && (screen_pos.x != 0.0 || screen_pos.y != 0.0) {
                painter.text(
                    screen_pos.to_egui(),
                    Align2::RIGHT_CENTER,
                    &name_str,
                    egui::FontId::default(),
                    Color32::WHITE
                );
                if name_str.contains("BP_Zombie"){
                    self.draw_centered_box(&painter, 
                        self.config.esp_box_type.clone(),
                        screen_pos,
                        100000.0 / pov.location.distance(location) as f32,
                        200000.0 / pov.location.distance(location) as f32,
                        self.config.esp_color
                    );
                }
            }

            if name_str.contains("BP_Zombie"){
                // let character_movement: usize = if let Some(res) = self.read_offset(actor, "ACharacter", "CharacterMovement"){res} else {continue};
                // let pending_force = FVector {
                //     x: 0.0,
                //     y: 0.0,
                //     z: 19000.0,
                // };
                // self.write(character_movement + self.dsapi.get_member_offset_unchecked("UCharacterMovementComponent", "PendingForceToApply") as usize, pending_force);

                


                


                // let mesh: usize = if let Some(res) = self.read_offset(actor, "ACharacter", "Mesh"){res} else {continue};
                // let bone_array: usize = if let Some(res) = self.read(mesh + self.dsapi.get_member_offset_unchecked("USkinnedMeshComponent", "MeshDeformerInstance") + 0x8){res} else {continue};
                // let bone_array_size: i32 = if let Some(res) = self.read(bone_array + 0x8){res} else {continue};
                // self.add_debug(format!("Bone Array Size: {}", bone_array_size));
                // let c2w: FTransform = if let Some(res) = self.read(root + self.dsapi.get_member_offset_unchecked("USceneComponent", "PhysicsVolumeChangedDelegate") as usize + 0x125){res} else {continue};
                // //self.add_debug(format!("c2w: {:?}", c2w));

                // let head_bone:FTransform = if let Some(res) = self.read(bone_array + 0x8){res} else {continue};
                
                // let component_matrix = c2w.to_matrix_with_scale();
                // let bone_matrix = head_bone.to_matrix_with_scale();
                // let world_matrix = component_matrix.multiply(&bone_matrix);

                // let head_location = FVector {
                //     x: world_matrix.m[3][0] as f64,
                // y: world_matrix.m[3][1] as f64,
                // z: world_matrix.m[3][2] as f64,
                // };
                // //self.add_debug(format!("Head Location: {:?}", head_location));
                // let screen_pos = world2screen(head_location, FMinimalViewInfo {
                //     location: camera_location,
                //     rotation: camera_rotation,
                //     fov,
                // }, self.window_size);
                // painter.circle_filled(
                //     screen_pos.to_egui(),
                //     5.0,
                //     Color32::from_rgb(255, 0, 0)
                // );
                if !name_str.contains("Crawler"){
                    location.z += 50.0;
                }
                if pov.location.distance(location) < best_target.distance(pov.location) {
                    best_target = location;
                }
            }
        }
        if best_target.x == f64::MAX {
            return;
        }
        if self.config.aim_enabled && (self.config.aim_key.is_some() && self.keys.contains(&self.config.aim_key.unwrap())) {
            let aim_angles = calculate_aim(&pov.location, &best_target, 0.0);
            //let aim_diff = aim_angles - camera_rotation;
            //let aim = camera_rotation + aim_diff * 0.1; // Adjust the smoothing factor as needed
            self.write(self.offsets.player_controller + self.dsapi.get_member_offset_unchecked("AController", "ControlRotation") as usize, aim_angles);
        }
    }
}