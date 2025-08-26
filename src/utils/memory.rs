impl crate::app::app::App{
    pub fn read<T>(&self,address:usize) -> Option<T> where T:Default {
        match self.game_proc.read_mem::<T>(address) {
            Ok(val) => {Some(val)},
            Err(_) => { None }
        }
    }
    pub fn read_offset<T>(&self,base:usize,class:&str,member:&str) -> Option<T> where T:Default {
        self.read(base + self.dsapi.get_member_offset_unchecked(class, member))
    }
    pub fn read_singleton_offset<T>(&self,base:usize,offset:&str) -> Option<T> where T:Default {
        self.read(base + self.dsapi.get_offset(offset).unwrap() as usize)
    }
    pub fn write<T>(&self,address:usize, value:T) where T:Default {
        self.game_proc.write_mem::<T>(address, value);
    }
    pub fn write_offset<T>(&self,address:usize, class:&str, member:&str, value:T) where T:Default {
        self.game_proc.write_mem::<T>(address + self.dsapi.get_member_offset_unchecked(class, member), value);
    }
}