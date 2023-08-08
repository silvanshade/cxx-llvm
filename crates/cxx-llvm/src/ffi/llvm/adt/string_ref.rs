use crate::gen::llvm::adt::string_ref;
use core::str::Utf8Error;
use moveref::expr;

pub use crate::auto::llvm::adt::string_ref::StringRef;

impl AsRef<[u8]> for StringRef<'_> {
    #[inline]
    fn as_ref(&self) -> &[u8] {
        let slice = string_ref::as_slice(*self);
        cxx_auto::ctypes::c_char::into_bytes(slice)
    }
}

impl<'a> From<&'a str> for StringRef<'a> {
    #[inline]
    fn from(str: &'a str) -> Self {
        string_ref::new_from_rust_str(str)
    }
}

impl<'a> From<&'a std::path::Path> for StringRef<'a> {
    #[inline]
    fn from(path: &'a std::path::Path) -> Self {
        let slice = cxx_auto::ctypes::c_char::from_path(path);
        string_ref::new_from_rust_slice(slice)
    }
}

impl<'a> StringRef<'a> {
    #[inline]
    pub fn new() -> impl moveref::New<Output = StringRef<'a>> {
        Self::default_new()
    }

    #[inline]
    pub fn as_str(self) -> Result<&'a str, Utf8Error> {
        let slice = string_ref::as_slice(self);
        let bytes = cxx_auto::ctypes::c_char::into_bytes(slice);
        core::str::from_utf8(bytes)
    }
}

impl Default for StringRef<'_> {
    #[inline]
    fn default() -> Self {
        *expr!(Self::new())
    }
}
