#![doc = r" NOTE: This module is auto-generated and should not be edited."]
#[derive(Clone, Copy)]
#[repr(C, align(8))]
pub struct Twine<'a> {
    _layout: [u8; 40],
    _neither_send_nor_sync: ::core::marker::PhantomData<[*const u8; 0]>,
    _lifetimes: ::core::marker::PhantomData<(&'a (),)>,
}
unsafe impl<'a> ::cxx::ExternType for Twine<'a> {
    type Id = ::cxx::type_id!("cxx_llvm::llvm::adt::twine::Twine");
    type Kind = ::cxx::kind::Trivial;
}
#[cfg(feature = "debug")]
impl<'a> ::core::fmt::Debug for Twine<'a> {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("Twine").finish()
    }
}
impl<'a> Twine<'a> {
    #[inline]
    pub fn default() -> Self {
        unsafe {
            let initializer = ::cxx_memory::new::by_raw(move |this| {
                let this = this.get_unchecked_mut().as_mut_ptr();
                self::ffi::cxx_default_new(this);
            });
            let this = ::core::mem::MaybeUninit::uninit();
            ::cxx_memory::New::new(initializer, ::core::pin::pin!(this));
            this.assume_init()
        }
    }
}
unsafe impl<'a> ::cxx_memory::CopyNew for Twine<'a> {
    #[inline]
    unsafe fn copy_new(that: &Self, this: ::core::pin::Pin<&mut ::core::mem::MaybeUninit<Self>>) {
        let this = this.get_unchecked_mut().as_mut_ptr();
        self::ffi::cxx_copy_new(this, that)
    }
}
unsafe impl<'a> ::cxx_memory::MoveNew for Twine<'a> {
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
    #[namespace = "cxx_llvm::llvm::adt::twine"]
    unsafe extern "C++" {
        include!("cxx-llvm-abi/cxx/include/llvm/ADT/Twine.hxx");
        #[cxx_name = "Twine"]
        #[allow(unused)]
        type Twine<'a> = super::Twine<'a>;
        unsafe fn cxx_copy_new<'a>(This: *mut Twine<'a>, that: &Twine<'a>);
        unsafe fn cxx_move_new<'a>(This: *mut Twine<'a>, that: *mut Twine<'a>);
        unsafe fn cxx_default_new<'a>(This: *mut Twine<'a>);
    }
}
#[cfg(test)]
mod info {
    use super::*;
    mod test {
        use super::*;
        #[test]
        fn cxx_abi_align() {
            ::core::assert_eq!(::core::mem::align_of::<Twine<'static>>(), 8)
        }
        #[test]
        fn cxx_abi_size() {
            ::core::assert_eq!(::core::mem::size_of::<Twine<'static>>(), 40)
        }
        :: static_assertions :: assert_impl_all ! (Twine < 'static > : :: core :: marker :: Copy);
        :: static_assertions :: assert_impl_all ! (Twine < 'static > : :: core :: marker :: Unpin);
    }
}
