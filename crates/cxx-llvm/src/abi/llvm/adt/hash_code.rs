#![doc = r" NOTE: This module is auto-generated and should not be edited."]
#[derive(Clone, Copy)]
#[repr(C, align(8))]
pub struct HashCode {
    _layout: [u8; 8],
    _neither_send_nor_sync: ::core::marker::PhantomData<[*const u8; 0]>,
}
unsafe impl ::cxx::ExternType for HashCode {
    type Id = ::cxx::type_id!("cxx_llvm::llvm::adt::hash_code::HashCode");
    type Kind = ::cxx::kind::Trivial;
}
#[cfg(feature = "debug")]
impl ::core::fmt::Debug for HashCode {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("HashCode").finish()
    }
}
impl HashCode {
    #[inline]
    pub fn default() -> impl ::cxx_memory::New<Output = HashCode> {
        unsafe {
            ::cxx_memory::new::by_raw(move |this| {
                let this = this.get_unchecked_mut().as_mut_ptr();
                self::ffi::cxx_default_new(this);
            })
        }
    }
}
unsafe impl ::cxx_memory::CopyNew for HashCode {
    #[inline]
    unsafe fn copy_new(that: &Self, this: ::core::pin::Pin<&mut ::core::mem::MaybeUninit<Self>>) {
        let this = this.get_unchecked_mut().as_mut_ptr();
        self::ffi::cxx_copy_new(this, that)
    }
}
unsafe impl ::cxx_memory::MoveNew for HashCode {
    #[inline]
    unsafe fn move_new(
        that: ::core::pin::Pin<::cxx_memory::MoveRef<'_, Self>>,
        this: ::core::pin::Pin<&mut ::core::mem::MaybeUninit<Self>>,
    ) {
        let this = this.get_unchecked_mut().as_mut_ptr();
        let that = &mut *::core::pin::Pin::into_inner_unchecked(that);
        self::ffi::cxx_move_new(this, that)
    }
}
#[cxx::bridge]
pub(crate) mod ffi {
    #[namespace = "cxx_llvm::llvm::adt::hash_code"]
    unsafe extern "C++" {
        include!("cxx-llvm-abi/cxx/include/llvm/ADT/HashCode.hxx");
        #[cxx_name = "HashCode"]
        #[allow(unused)]
        type HashCode = super::HashCode;
        unsafe fn cxx_copy_new(This: *mut HashCode, that: &HashCode);
        unsafe fn cxx_move_new(This: *mut HashCode, that: *mut HashCode);
        unsafe fn cxx_default_new(This: *mut HashCode);
    }
}
#[cfg(test)]
mod info {
    use super::*;
    mod test {
        use super::*;
        #[test]
        fn cxx_abi_align() {
            ::core::assert_eq!(::core::mem::align_of::<HashCode>(), 8)
        }
        #[test]
        fn cxx_abi_size() {
            ::core::assert_eq!(::core::mem::size_of::<HashCode>(), 8)
        }
        :: static_assertions :: assert_impl_all ! (HashCode : :: core :: marker :: Copy);
        :: static_assertions :: assert_impl_all ! (HashCode : :: core :: marker :: Unpin);
    }
}
