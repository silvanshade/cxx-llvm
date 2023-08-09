use crate::ffi::llvm::adt::small_vector::SmallVectorElement;
use core::pin::Pin;
use cxx_auto::ctypes::c_void;

#[repr(C)]
pub struct SmallVectorImpl<T>
where
    T: SmallVectorImplElement,
{
    _header: (*const c_void, T::SizeType, T::SizeType),
    _non_send_sync: core::marker::PhantomData<[*const u8; 0]>,
    _pinned: core::marker::PhantomPinned,
    _covariant: core::marker::PhantomData<Box<T>>,
}

impl<T> Drop for SmallVectorImpl<T>
where
    T: SmallVectorImplElement,
{
    fn drop(&mut self) {
        unsafe {
            <T as SmallVectorImplElement>::cxx_destruct(self);
        }
    }
}

impl<T> SmallVectorImpl<T>
where
    T: SmallVectorImplElement,
{
    #[allow(clippy::new_ret_no_self)]
    #[inline]
    pub fn new() -> impl moveref::New<Output = T::DefaultType> {
        unsafe {
            moveref::new::by_raw(move |this| {
                let this = this.get_unchecked_mut().as_mut_ptr();
                T::cxx_default_new(this)
            })
        }
    }

    #[inline]
    pub fn iter(&self) -> core::slice::Iter<'_, T> {
        T::as_slice(self).iter()
    }

    #[inline]
    pub fn iter_pin(self: Pin<&mut Self>) -> IterPin<'_, T> {
        let slice = unsafe { Pin::into_inner_unchecked(T::as_pin_slice(self)) };
        let inner = slice.iter_mut();
        IterPin { inner }
    }
}

pub trait SmallVectorImplElement: SmallVectorElement {
    type ReprType;

    #[inline]
    fn into_repr_mut_ptr(this: *mut SmallVectorImpl<Self>) -> *mut <Self as SmallVectorImplElement>::ReprType {
        this.cast()
    }

    #[inline]
    fn into_repr_ref(this: &SmallVectorImpl<Self>) -> &<Self as SmallVectorImplElement>::ReprType {
        unsafe { core::mem::transmute(this) }
    }

    #[inline]
    fn from_repr_ref(repr: &<Self as SmallVectorImplElement>::ReprType) -> &SmallVectorImpl<Self> {
        unsafe { core::mem::transmute(repr) }
    }

    #[inline]
    fn into_repr_pin(this: Pin<&mut SmallVectorImpl<Self>>) -> Pin<&mut <Self as SmallVectorImplElement>::ReprType> {
        unsafe { core::mem::transmute(this) }
    }

    #[inline]
    fn from_repr_pin(repr: Pin<&mut <Self as SmallVectorImplElement>::ReprType>) -> Pin<&mut SmallVectorImpl<Self>> {
        unsafe { core::mem::transmute(repr) }
    }

    /// # Safety
    ///
    /// - `this` must be previously allocated memory
    /// - `this` must be in a valid, initialized state for its type
    /// - after invocation, `this` must be destroyed and the pointer must not be dereferenced again
    unsafe fn cxx_destruct(this: *mut SmallVectorImpl<Self>);

    fn as_slice(this: &SmallVectorImpl<Self>) -> &[Self];

    fn as_pin_slice(this: Pin<&mut SmallVectorImpl<Self>>) -> Pin<&mut [Self]>;
}

#[repr(transparent)]
pub struct IterPin<'a, T>
where
    T: 'a,
{
    pub(crate) inner: core::slice::IterMut<'a, T>,
}

impl<T> core::fmt::Debug for IterPin<'_, T>
where
    T: core::fmt::Debug,
{
    #[inline]
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.inner, f)
    }
}

impl<T> DoubleEndedIterator for IterPin<'_, T> {
    #[inline]
    fn next_back(&mut self) -> Option<Self::Item> {
        self.inner.next_back().map(|val| unsafe { Pin::new_unchecked(val) })
    }
}

impl<T> ExactSizeIterator for IterPin<'_, T> {
    #[inline]
    fn len(&self) -> usize {
        self.inner.len()
    }
}

impl<T> core::iter::FusedIterator for IterPin<'_, T> {
}

impl<'a, T> Iterator for IterPin<'a, T> {
    type Item = Pin<<core::slice::IterMut<'a, T> as Iterator>::Item>;

    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        self.inner.next().map(|val| unsafe { Pin::new_unchecked(val) })
    }

    #[inline]
    fn size_hint(&self) -> (usize, Option<usize>) {
        self.inner.size_hint()
    }
}

unsafe impl<T> Send for IterPin<'_, T> where T: Send
{
}

unsafe impl<T> Sync for IterPin<'_, T> where T: Sync
{
}

impl<'a, T> IntoIterator for &'a SmallVectorImpl<T>
where
    T: SmallVectorImplElement,
{
    type Item = <&'a [T] as IntoIterator>::Item;
    type IntoIter = <&'a [T] as IntoIterator>::IntoIter;

    #[inline]
    fn into_iter(self) -> Self::IntoIter {
        T::as_slice(self).iter()
    }
}

impl<'a, T> IntoIterator for Pin<&'a mut SmallVectorImpl<T>>
where
    T: SmallVectorImplElement,
{
    type Item = <IterPin<'a, T> as Iterator>::Item;
    type IntoIter = IterPin<'a, T>;

    #[inline]
    fn into_iter(self) -> Self::IntoIter {
        self.iter_pin()
    }
}
