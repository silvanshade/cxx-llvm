#![doc = r" NOTE: This module is auto-generated and should not be edited."]
#[repr(C, align(8))]
pub struct Triple {
    _layout: [u8; 56],
    _neither_send_nor_sync: ::core::marker::PhantomData<[*const u8; 0]>,
    _pinned: ::core::marker::PhantomPinned,
}
unsafe impl ::cxx::ExternType for Triple {
    type Id = ::cxx::type_id!("cxx_llvm::llvm::adt::triple::Triple");
    type Kind = ::cxx::kind::Opaque;
}
impl ::core::ops::Drop for Triple {
    #[inline]
    fn drop(&mut self) {
        unsafe {
            self::ffi::cxx_destruct(self);
        }
    }
}
#[cfg(feature = "debug")]
impl ::core::fmt::Debug for Triple {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("Triple").finish()
    }
}
impl Triple {
    #[inline]
    pub fn default() -> impl ::cxx_memory::New<Output = Triple> {
        unsafe {
            ::cxx_memory::new::by_raw(move |this| {
                let this = this.get_unchecked_mut().as_mut_ptr();
                self::ffi::cxx_default_new(this);
            })
        }
    }
}
unsafe impl ::cxx_memory::CopyNew for Triple {
    #[inline]
    unsafe fn copy_new(that: &Self, this: ::core::pin::Pin<&mut ::core::mem::MaybeUninit<Self>>) {
        let this = this.get_unchecked_mut().as_mut_ptr();
        self::ffi::cxx_copy_new(this, that)
    }
}
unsafe impl ::cxx_memory::MoveNew for Triple {
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
    #[namespace = "cxx_llvm::llvm::adt::triple"]
    unsafe extern "C++" {
        include!("cxx-llvm-abi/cxx/include/llvm/ADT/Triple.hxx");
        #[cxx_name = "Triple"]
        #[allow(unused)]
        type Triple = super::Triple;
        unsafe fn cxx_copy_new(This: *mut Triple, that: &Triple);
        unsafe fn cxx_move_new(This: *mut Triple, that: *mut Triple);
        unsafe fn cxx_default_new(This: *mut Triple);
        unsafe fn cxx_destruct(This: *mut Triple);
    }
}
#[cfg(test)]
mod info {
    use super::*;
    mod test {
        use super::*;
        #[test]
        fn cxx_abi_align() {
            ::core::assert_eq!(::core::mem::align_of::<Triple>(), 8)
        }
        #[test]
        fn cxx_abi_size() {
            ::core::assert_eq!(::core::mem::size_of::<Triple>(), 56)
        }
    }
}
