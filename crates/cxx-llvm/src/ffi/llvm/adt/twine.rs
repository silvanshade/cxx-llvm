use crate::gen::llvm::adt::twine;

pub use crate::abi::llvm::adt::twine::Twine;

impl<'a> From<&'a str> for Twine<'a> {
    #[inline]
    fn from(value: &'a str) -> Self {
        twine::new_from_rust_str(value)
    }
}

impl<'a> From<&'a std::path::Path> for Twine<'a> {
    #[inline]
    fn from(path: &'a std::path::Path) -> Self {
        let slice = cxx_memory_abi::ctypes::c_char::from_path(path);
        twine::new_from_rust_slice(slice)
    }
}
