use crate::gen::llvm::adt::twine;
use moveref::expr;

pub use crate::auto::llvm::adt::twine::Twine;

impl<'a> Twine<'a> {
    #[inline]
    pub fn new() -> impl moveref::New<Output = Twine<'a>> {
        Self::default_new()
    }
}

impl Default for Twine<'_> {
    #[inline]
    fn default() -> Self {
        *expr!(Self::new())
    }
}

impl<'a> From<&'a str> for Twine<'a> {
    #[inline]
    fn from(value: &'a str) -> Self {
        twine::new_from_rust_str(value)
    }
}

impl<'a> From<&'a std::path::Path> for Twine<'a> {
    #[inline]
    fn from(path: &'a std::path::Path) -> Self {
        let slice = cxx_auto::ctypes::c_char::from_path(path);
        twine::new_from_rust_slice(slice)
    }
}
