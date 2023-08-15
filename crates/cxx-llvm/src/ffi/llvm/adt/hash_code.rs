use moveref::expr;

pub use crate::auto::llvm::adt::hash_code::HashCode;

impl HashCode {
    #[must_use]
    #[inline]
    pub fn new() -> impl moveref::New<Output = HashCode> {
        Self::default_new()
    }
}

impl Default for HashCode {
    #[inline]
    fn default() -> Self {
        *expr!(Self::new())
    }
}
