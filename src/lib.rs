//! A tiny crate, which provides slices to the memory which backs an instance of a struct

/// Return a slice of the memory that contains the struct
pub trait AsBytes: Sized {
    unsafe fn as_bytes(&self) -> &[u8] {
        let ptr = self as *const _ as *const u8;
        let len = std::mem::size_of::<Self>();
        std::slice::from_raw_parts(ptr, len)
    }
    unsafe fn as_bytes_mut(&mut self) -> &mut [u8] {
        let ptr = self as *mut _ as *mut u8;
        let len = std::mem::size_of::<Self>();
        std::slice::from_raw_parts_mut(ptr, len)
    }
}

impl<T: Sized> AsBytes for T {}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
