use crate::gen::llvm::adt::string_ref;

pub use crate::abi::llvm::adt::string_ref::StringRef;

impl<'a> From<&'a str> for StringRef<'a> {
    #[inline]
    fn from(str: &'a str) -> Self {
        string_ref::new_from_rust_str(str)
    }
}

impl<'a> From<&'a std::path::Path> for StringRef<'a> {
    #[inline]
    fn from(path: &'a std::path::Path) -> Self {
        let slice = cxx_memory_abi::ctypes::c_char::from_path(path);
        string_ref::new_from_rust_slice(slice)
    }
}
