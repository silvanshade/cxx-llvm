use crate::ffi::llvm::adt::small_vector_impl::{SmallVectorImpl, SmallVectorImplElement};
use core::pin::Pin;

#[repr(C)]
pub struct SmallVector<T, const CAPACITY: usize>
where
    T: SmallVectorImplElement,
{
    _impl: SmallVectorImpl<T>,
    _storage: [T; CAPACITY],
}

// NOTE: Most methods below are forced to take a `CAPACITY`, since we cannot specify the type
// `SmallVector<T, { T::DEFAULT_CAPACITY }>` without unstable features.
//
// In practice, `CAPACITY` should always be equal to `T::DEFAULT_CAPACITY` because the only way to
// construct a `SmallVector<T>` is through `SmallVectorImpl::<T>::default`, which always returns a
// `SmallVector<T>` with `CAPACITY == T::DEFAULT_CAPACITY`.
//
// However, as a reminder of this situation, `debug_assert_eq` is used as a guard in these methods.
// And in fact it would undefined-behavior if these methods were actually called with a `CAPACITY`
// different from `T::DEFAULT_CAPACITY` since the ffi-bindings are only defined for the default.

impl<T, const CAPACITY: usize> Drop for SmallVector<T, CAPACITY>
where
    T: SmallVectorImplElement,
{
    #[inline]
    fn drop(&mut self) {
        debug_assert_eq!(CAPACITY, T::DEFAULT_CAPACITY);
        let this = (self as *mut Self).cast();
        unsafe {
            <T as SmallVectorElement>::cxx_destruct(this);
        }
    }
}

impl<T, const CAPACITY: usize> AsRef<SmallVectorImpl<T>> for SmallVector<T, CAPACITY>
where
    T: SmallVectorImplElement,
{
    fn as_ref(&self) -> &SmallVectorImpl<T> {
        debug_assert_eq!(CAPACITY, T::DEFAULT_CAPACITY);
        let this = unsafe { core::mem::transmute(self) };
        T::as_ref_small_vector_impl(this)
    }
}

impl<T, const CAPACITY: usize> SmallVector<T, CAPACITY>
where
    T: SmallVectorImplElement,
{
    pub fn as_pin(self: Pin<&mut Self>) -> Pin<&mut SmallVectorImpl<T>> {
        debug_assert_eq!(CAPACITY, T::DEFAULT_CAPACITY);
        let this = unsafe { core::mem::transmute(self) };
        T::as_pin_small_vector_impl(this)
    }
}

pub trait SmallVectorElement: Sized {
    type DefaultType;
    type ReprType;
    type SizeType;

    const DEFAULT_CAPACITY: usize;

    #[inline]
    fn into_repr_mut_ptr(this: *mut Self::DefaultType) -> *mut Self::ReprType {
        unsafe { core::mem::transmute(this) }
    }

    #[inline]
    fn into_repr_ref(this: &Self::DefaultType) -> &Self::ReprType {
        unsafe { core::mem::transmute(this) }
    }

    #[inline]
    fn from_repr_ref(repr: &Self::ReprType) -> &Self::DefaultType {
        unsafe { core::mem::transmute(repr) }
    }

    #[inline]
    fn into_repr_pin(this: Pin<&mut Self::DefaultType>) -> Pin<&mut Self::ReprType> {
        unsafe { core::mem::transmute(this) }
    }

    #[inline]
    fn from_repr_pin(repr: Pin<&mut Self::ReprType>) -> Pin<&mut Self::DefaultType> {
        unsafe { core::mem::transmute(repr) }
    }

    unsafe fn cxx_default_new(this: *mut Self::DefaultType);

    unsafe fn cxx_destruct(this: *mut Self::DefaultType);

    fn as_ref_small_vector_impl(this: &Self::DefaultType) -> &SmallVectorImpl<Self>
    where
        Self: SmallVectorImplElement;

    fn as_pin_small_vector_impl(this: Pin<&mut Self::DefaultType>) -> Pin<&mut SmallVectorImpl<Self>>
    where
        Self: SmallVectorImplElement;
}

pub mod small_vector_element {
    // NOTE: used in const asserts
    #[inline]
    pub const fn size_type_size<T>() -> usize {
        if core::mem::size_of::<T>() < 4 && core::mem::size_of::<*const core::ffi::c_void>() >= 8 {
            8
        } else {
            4
        }
    }

    #[inline]
    pub const fn header<T>() -> usize
    where
        T: super::SmallVectorElement,
    {
        let void_type_size = core::mem::size_of::<*const core::ffi::c_void>();
        let size_type_size = core::mem::size_of::<T::SizeType>();
        void_type_size + 2 * size_type_size
    }

    #[inline]
    pub const fn capacity<T>() -> usize
    where
        T: super::SmallVectorElement,
    {
        let k_preferred_small_vector_size_of = 64;
        let preferred_inline_bytes = k_preferred_small_vector_size_of - header::<T>();
        let num_elements_that_fit = preferred_inline_bytes / core::mem::size_of::<T>();
        if num_elements_that_fit == 0 {
            1
        } else {
            num_elements_that_fit
        }
    }
}

impl<'a, T, const CAPACITY: usize> IntoIterator for &'a SmallVector<T, CAPACITY>
where
    T: SmallVectorImplElement,
{
    type Item = <&'a SmallVectorImpl<T> as IntoIterator>::Item;
    type IntoIter = <&'a SmallVectorImpl<T> as IntoIterator>::IntoIter;

    #[inline]
    fn into_iter(self) -> <&'a SmallVectorImpl<T> as IntoIterator>::IntoIter {
        self.as_ref().into_iter()
    }
}

impl<'a, T, const CAPACITY: usize> IntoIterator for Pin<&'a mut SmallVector<T, CAPACITY>>
where
    T: SmallVectorImplElement,
{
    type Item = <Pin<&'a mut SmallVectorImpl<T>> as IntoIterator>::Item;
    type IntoIter = <Pin<&'a mut SmallVectorImpl<T>> as IntoIterator>::IntoIter;

    #[inline]
    fn into_iter(self) -> <Pin<&'a mut SmallVectorImpl<T>> as IntoIterator>::IntoIter {
        self.as_pin().into_iter()
    }
}
