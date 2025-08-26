#[allow(dead_code)]
impl crate::app::app::App{
    pub fn read<T>(&self,address:usize) -> Option<T> where T:Default {
        match self.game_proc.read_mem::<T>(address) {
            Ok(val) => {Some(val)},
            Err(_) => { None }
        }
    }
    pub fn write<T>(&self,address:usize, value:T) where T:Default {
        self.game_proc.write_mem::<T>(address, value);
    }
}