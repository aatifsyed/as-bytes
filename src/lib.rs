//! A tiny crate, which provides slices to the memory which backs an instance of a struct.
//!
//! ```
//! use as_bytes::AsBytes;
//! let i = u32::MAX;
//! let bytes = unsafe { i.as_bytes() };
//! assert_eq!(bytes, [255, 255, 255, 255]);
//! ```
//! You can use this to peek at structure layout.
//! One usecase is for testing sending structs sent the wire.
//! The below examples demonstrate two different packing attributes on the same structure.
//! ```
//! # #[repr(packed)]
//! # #[allow(dead_code)] // unread fields
//! # struct ReprPacked {
//! #     a: usize,
//! #     b: u8,
//! #     c: usize,
//! # }
//! # use as_bytes::AsBytes;
//!
//! let packed = ReprPacked {
//!     a: usize::MAX,
//!     b: 0u8,
//!     c: usize::MAX,
//! };
//! let bytes = unsafe { packed.as_bytes() };
//! assert_eq!(
//!     bytes,
//!     [255, 255, 255, 255, 255, 255, 255, 255, 0, 255, 255, 255, 255, 255, 255, 255, 255]
//! );
//! ```
//!
//! ```
//! # #[repr(C)]
//! # struct ReprC {
//! #     a: usize,
//! #     b: u8,
//! #     c: usize,
//! # }
//! # use as_bytes::AsBytes;
//!
//! let packed = ReprC {
//!     a: usize::MAX,
//!     b: 0u8,
//!     c: usize::MAX,
//! };
//! let bytes = unsafe { packed.as_bytes() };
//! assert_eq!(
//!     bytes,
//!     [
//!         255, 255, 255, 255, 255, 255, 255, 255, 0, 0, 0, 0, 0, 0, 0, 0, 255, 255, 255, 255, 255, 255, 255, 255
//!     ]
//! );
//! ```

/// Return a slice of the memory that contains the struct
pub trait AsBytes {
    unsafe fn as_bytes(&self) -> &[u8] {
        let ptr = self as *const _ as *const u8;
        let len = std::mem::size_of_val(self);
        std::slice::from_raw_parts(ptr, len)
    }
    unsafe fn as_bytes_mut(&mut self) -> &mut [u8] {
        let ptr = self as *mut _ as *mut u8;
        let len = std::mem::size_of_val(self);
        std::slice::from_raw_parts_mut(ptr, len)
    }
}

impl<T> AsBytes for T {}

#[cfg(test)]
mod tests {
    use crate::AsBytes;

    #[test]
    fn test_u32() {
        let i = u32::MAX;
        let bytes = unsafe { i.as_bytes() };
        assert_eq!(bytes, [255, 255, 255, 255]);
    }

    #[repr(packed)]
    #[allow(dead_code)] // unread fields
    struct ReprPacked {
        a: usize,
        b: u8,
        c: usize,
    }

    #[test]
    fn test_repr_packed_struct() {
        let packed = ReprPacked {
            a: usize::MAX,
            b: 0u8,
            c: usize::MAX,
        };
        let bytes = unsafe { packed.as_bytes() };
        assert_eq!(
            bytes,
            [255, 255, 255, 255, 255, 255, 255, 255, 0, 255, 255, 255, 255, 255, 255, 255, 255]
        )
    }

    #[repr(C)]
    struct ReprC {
        a: usize,
        b: u8,
        c: usize,
    }

    #[test]
    fn test_repr_c_struct() {
        let packed = ReprC {
            a: usize::MAX,
            b: 0u8,
            c: usize::MAX,
        };
        let bytes = unsafe { packed.as_bytes() };
        assert_eq!(
            bytes,
            [
                255, 255, 255, 255, 255, 255, 255, 255, 0, 0, 0, 0, 0, 0, 0, 0, 255, 255, 255, 255,
                255, 255, 255, 255
            ]
        )
    }
}
