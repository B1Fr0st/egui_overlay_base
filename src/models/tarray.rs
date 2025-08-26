//pasted AF

// TArray - Unreal Engine's dynamic array
#[derive(Debug, Clone, Copy, Default)]
#[repr(C)]
pub struct TArray<T> {
    pub data: usize, // T* Data
    pub count: i32,  // int32_t Count
    pub max: i32,    // int32_t Max
    _marker: std::marker::PhantomData<T>,
}

impl<T: Copy + 'static> TArray<T> {
    pub fn num(&self) -> i32 {
        self.count
    }

    pub fn is_valid_index(&self, index: i32) -> bool {
        index >= 0 && index < self.num()
    }

    pub fn slack(&self) -> i32 {
        self.max - self.count
    }

    pub fn is_valid(&self) -> bool {
        self.data != 0 && self.count >= 0 && self.count <= self.max
    }

    pub fn is_empty(&self) -> bool {
        self.count == 0
    }

    pub fn get_data<U>(&self, index: i32) -> usize
    where
        U: Copy + 'static,
    {
        if !self.is_valid_index(index) {
            return 0;
        }
        // Get Type of _marker and use that element size
        let element_size = std::mem::size_of::<U>();
        let element_address = self.data + (index as usize * element_size);
        element_address
    }

    pub fn get<U>(&self,proc:&mut proc_mem::Process, index: i32) -> Result<U, Box<dyn std::error::Error>>
    where
        U: Copy + 'static + Default,
    {
        if !self.is_valid_index(index) {
            return Err(format!("Index {} out of bounds (count: {})", index, self.count).into());
        }

        let element_size = std::mem::size_of::<U>();
        let element_address = self.data + (index as usize * element_size);
        Ok(proc.read_mem::<U>(element_address).map_err(|e| {
            format!("Failed to read element at index {}: {:?}", index, e)
        })?)
    }
    pub fn read_all<U>(&self, proc:&mut proc_mem::Process) -> Result<Vec<U>, Box<dyn std::error::Error>>
    where
        U: Copy + 'static + Default,
    {

        if !self.is_valid() || self.is_empty() {
            return Ok(Vec::new());
        }

        let element_size = std::mem::size_of::<U>();
        let total_size = self.count as usize * element_size;
        let mut buffer = vec![0u8; total_size];

        proc.read_bytes(self.data, buffer.as_mut_ptr(), total_size);
        let ptr = buffer.as_ptr() as *const U;
        let slice = unsafe { std::slice::from_raw_parts(ptr, self.count as usize) };
        Ok(slice.to_vec())
    }

    pub fn for_each<U, F>(&self, proc:&mut proc_mem::Process, mut func: F) -> Result<(), Box<dyn std::error::Error>>
    where
        U: Copy + 'static,
        F: FnMut(i32, U),
    {
        if !self.is_valid() || self.is_empty() {
            return Ok(());
        }

        let element_size = std::mem::size_of::<U>();
        let total_size = self.count as usize * element_size;
        let mut buffer = vec![0u8; total_size];

        proc.read_bytes(self.data,  buffer.as_mut_ptr(), total_size);

        let ptr = buffer.as_ptr() as *const U;
        let slice = unsafe { std::slice::from_raw_parts(ptr, self.count as usize) };

        for (i, &item) in slice.iter().enumerate() {
            func(i as i32, item);
        }

        Ok(())
    }
}
