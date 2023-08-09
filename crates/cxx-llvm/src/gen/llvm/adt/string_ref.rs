#[cxx::bridge]
mod ffi {
    extern "C++" {
        include!("cxx-auto/cxx/include/cxx-auto.hxx");
        include!("cxx-llvm-auto/cxx/include/llvm/ADT/StringRef.hxx");

        #[cxx_name = "c_char"]
        type _c_char = cxx_auto::ctypes::c_char;

        #[namespace = "cxx_llvm::llvm::adt::string_ref"]
        type StringRef<'a> = crate::ffi::llvm::adt::string_ref::StringRef<'a>;
    }

    #[namespace = "cxx_llvm::llvm::adt::string_ref"]
    unsafe extern "C++" {
        fn new_from_rust_str<'a>(str: &'a str) -> StringRef<'a>;

        fn new_from_rust_slice<'a>(str: &'a [_c_char]) -> StringRef<'a>;

        #[allow(clippy::needless_lifetimes)]
        fn as_slice<'a>(This: StringRef<'a>) -> &'a [_c_char];
    }
}
pub(crate) use self::ffi::*;
