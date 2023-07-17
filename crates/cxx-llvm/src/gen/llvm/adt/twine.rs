#[cxx::bridge]
mod ffi {
    extern "C++" {
        include!("cxx-memory-abi/cxx/include/cxx-memory-abi.hxx");
        include!("cxx-llvm-abi/cxx/include/llvm/ADT/Twine.hxx");

        #[cxx_name = "c_char"]
        type _c_char = cxx_memory_abi::ctypes::c_char;

        #[namespace = "cxx_llvm::llvm::adt::twine"]
        type Twine<'a> = crate::ffi::llvm::adt::twine::Twine<'a>;
    }

    #[namespace = "cxx_llvm::llvm::adt::twine"]
    unsafe extern "C++" {
        fn new_from_rust_str<'a>(str: &'a str) -> Twine<'a>;

        fn new_from_rust_slice<'a>(str: &'a [_c_char]) -> Twine<'a>;
    }
}
pub(crate) use self::ffi::*;
