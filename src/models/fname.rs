use std::sync::{LazyLock, Mutex};


pub static STRING_CACHE: LazyLock<Mutex<Vec<Option<&'static str>>>> =
    LazyLock::new(|| Mutex::new(Vec::new()));


// Unreal Engine 5....
#[derive(Debug, Clone, Copy, Default)]
pub struct FName {
    pub comparison_index: u32,
    pub _number: u32,
}

impl FName {
    pub fn to_str(&self, proc:proc_mem::Process,gnames:usize) -> &str {
        let index = self.comparison_index as usize;

        // Try to get cached value with minimal lock time
        {
            let cache = STRING_CACHE.lock().unwrap();
            if index < cache.len() {
                if let Some(ref s) = cache[index] {
                    return s;
                }
            }
        }

        if self.comparison_index == 0 {
            return "None";
        }
        const NAME_SIZE: usize = 1024;

        let chunk_offset = (self.comparison_index >> 16) as u32;
        let name_offset = self.comparison_index as u16;

        let chunk_ptr_address = gnames + 8 * (chunk_offset as usize + 2);

        let name_pool_chunk_ptr = match proc.read_mem::<u64>(chunk_ptr_address) {
            Ok(ptr) if ptr != 0 => ptr as usize,
            _ => return "None",
        };

        let name_pool_chunk = name_pool_chunk_ptr + 2 * name_offset as usize;
        let name_length_raw = match proc.read_mem::<u16>(name_pool_chunk) {
            Ok(len) if len != 0 => len,
            _ => return "None",
        };

        let name_length = (name_length_raw >> 6) as usize;
        if name_length == 0 {
            return "None";
        }

        let safe_length = name_length.min(NAME_SIZE);
        let mut name_bytes = vec![0u8; safe_length];

        // If read_buf fails, return "None"
        if !proc.read_bytes(name_pool_chunk + 2, name_bytes.as_mut_ptr(),safe_length) {
            return "None";
        }

        let name_str = match std::str::from_utf8(&name_bytes) {
            Ok(s) => s.trim_end_matches('\0'),
            Err(_) => return "None",
        };

        let result = if name_str.is_empty() || name_str == "None" {
            "None".to_owned()
        } else {
            name_str.to_owned()
        };

        // Cache the result as an owned String and return a reference to a static "None" or leak the String for a static reference
        if result == "None" {
            return "None";
        } else {
            // Leak the String to get a &'static str
            let static_str: &'static str = Box::leak(result.into_boxed_str());
            let mut cache = STRING_CACHE.lock().unwrap();
            if index >= cache.len() {
                cache.resize_with(index + 1, || None);
            }
            cache[index] = Some(static_str);
            static_str
        }
    }
}