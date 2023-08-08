#[cxx::bridge]
mod ffi {
    extern "C++" {
        include!("cxx-auto/cxx/include/cxx-auto.hxx");
        include!("cxx-llvm-auto/cxx/include/llvm/ADT/Twine.hxx");

        #[cxx_name = "c_char"]
        type _c_char = cxx_auto::ctypes::c_char;

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
