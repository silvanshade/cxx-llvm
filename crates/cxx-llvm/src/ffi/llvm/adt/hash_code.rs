use cxx_memory::cxx;

pub use crate::abi::llvm::adt::hash_code::HashCode;

impl HashCode {
    #[inline]
    pub fn new() -> impl cxx_memory::New<Output = HashCode> {
        Self::default_new()
    }
}

impl Default for HashCode {
    #[inline]
    fn default() -> Self {
        *cxx!(Self::new())
    }
}
