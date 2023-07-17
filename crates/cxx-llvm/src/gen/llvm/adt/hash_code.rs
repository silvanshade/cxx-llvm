#[cxx::bridge]
mod ffi {
    extern "C++" {
        include!("cxx-llvm-abi/cxx/include/llvm/ADT/HashCode.hxx");

        // #[namespace = "cxx_llvm::llvm::adt::hash_code"]
        // type HashCode = crate::ffi::llvm::adt::hash_code::HashCode;
    }

    #[namespace = "cxx_llvm::llvm::adt::hash_code"]
    unsafe extern "C++" {}
}
// pub(crate) use self::ffi::*;
