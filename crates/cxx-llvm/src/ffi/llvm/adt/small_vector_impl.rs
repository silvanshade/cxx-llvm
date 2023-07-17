use crate::ffi::llvm::adt::small_vector::SmallVectorElement;
use core::pin::Pin;
use cxx_memory_abi::ctypes::c_void;

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
    #[inline]
    pub fn default() -> impl cxx_memory::New<Output = T::DefaultType> {
        unsafe {
            cxx_memory::new::by_raw(move |this| {
                let this = this.get_unchecked_mut().as_mut_ptr();
                T::cxx_default_new(this)
            })
        }
    }

    #[inline]
    pub fn iter(&self) -> Iter<'_, T> {
        let inner = T::as_slice(self).iter();
        Iter { inner }
    }

    #[inline]
    pub fn iter_pin(self: Pin<&mut Self>) -> IterPin<'_, T> {
        let inner = unsafe { T::as_mut_slice(self) }.iter_mut();
        IterPin { inner }
    }
}

pub trait SmallVectorImplElement: SmallVectorElement {
    unsafe fn cxx_destruct(this: *mut SmallVectorImpl<Self>);

    fn as_slice(this: &SmallVectorImpl<Self>) -> &[Self];

    unsafe fn as_mut_slice(this: Pin<&mut SmallVectorImpl<Self>>) -> &mut [Self];
}

#[repr(transparent)]
pub struct Iter<'a, T>
where
    T: 'a,
{
    pub(crate) inner: core::slice::Iter<'a, T>,
}

impl<'a, T> Iter<'a, T> {
    #[inline]
    pub fn as_slice(&self) -> &'a [T] {
        self.inner.as_slice()
    }
}

impl<T> AsRef<[T]> for Iter<'_, T> {
    #[inline]
    fn as_ref(&self) -> &[T] {
        self.inner.as_ref()
    }
}

impl<T> Clone for Iter<'_, T> {
    #[inline]
    fn clone(&self) -> Self {
        Self {
            inner: self.inner.clone(),
        }
    }
}

impl<T> core::fmt::Debug for Iter<'_, T>
where
    T: core::fmt::Debug,
{
    #[inline]
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.inner, f)
    }
}

impl<T> DoubleEndedIterator for Iter<'_, T> {
    #[inline]
    fn next_back(&mut self) -> Option<Self::Item> {
        self.inner.next_back()
    }
}

impl<T> ExactSizeIterator for Iter<'_, T> {
    #[inline]
    fn len(&self) -> usize {
        self.inner.len()
    }
}

impl<T> core::iter::FusedIterator for Iter<'_, T> {
}

impl<'a, T> Iterator for Iter<'a, T> {
    type Item = <core::slice::Iter<'a, T> as Iterator>::Item;

    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        self.inner.next()
    }

    #[inline]
    fn size_hint(&self) -> (usize, Option<usize>) {
        self.inner.size_hint()
    }
}

unsafe impl<T> Send for Iter<'_, T> where T: Send
{
}

unsafe impl<T> Sync for Iter<'_, T> where T: Sync
{
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
