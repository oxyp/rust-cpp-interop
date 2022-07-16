#[repr(C)]
pub struct TheMemory {
    // We probably don't need to know the internal data structues so let's just make this a blob.
    opaque_memory: [u8;TheMemory::SIZE]
}
impl TheMemory {
    pub const SIZE: usize = 128;
}
impl Default for TheMemory {
    fn default() -> TheMemory {
      TheMemory{ opaque_memory: [0u8;TheMemory::SIZE] }
    }
}


extern "C" {
    pub fn init(
        memory: *mut TheMemory,
        width: i32,
        height: i32,
    );
}
