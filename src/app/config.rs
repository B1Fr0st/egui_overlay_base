use serde_derive::{Deserialize,Serialize};
use std::collections::HashMap;
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct AppConfig{
    pub window_positions:HashMap<String, [i32;2]>
}

impl AppConfig{
    pub fn from_file(_path:&str) -> Self{
        //fixme
        let mut window_positions: HashMap<String, [i32; 2]> = HashMap::new();
        window_positions.insert("Placeholder".to_string(),[2000,200]);
        Self {
            window_positions
        }
    }
}