use crate::{ffi::llvm::adt::twine::Twine, gen::llvm::adt::triple};
use core::{mem::MaybeUninit, pin::Pin};

pub use crate::{
    auto::llvm::adt::triple::Triple,
    gen::llvm::adt::triple::{ArchType, EnvironmentType, OSType, ObjectFormatType, SubArchType, VendorType},
};

impl Triple {
    #[must_use]
    #[inline]
    pub fn new() -> impl moveref::New<Output = Triple> {
        Self::default_new()
    }

    #[inline]
    pub fn from<Data>(data: Data) -> impl moveref::New<Output = Triple>
    where
        Data: Into<crate::Initializer<Self, Data>>,
    {
        data.into()
    }
}

impl<'a, Arch, Vendor, Os> From<(Arch, Vendor, Os)> for crate::Initializer<Triple, (Arch, Vendor, Os)>
where
    Arch: 'a + Into<Twine<'a>>,
    Vendor: 'a + Into<Twine<'a>>,
    Os: 'a + Into<Twine<'a>>,
{
    #[inline]
    fn from(data: (Arch, Vendor, Os)) -> Self {
        #[inline]
        unsafe fn closure<'a, Arch, Vendor, Os>(this: Pin<&mut MaybeUninit<Triple>>, data: (Arch, Vendor, Os))
        where
            Arch: 'a + Into<Twine<'a>>,
            Vendor: 'a + Into<Twine<'a>>,
            Os: 'a + Into<Twine<'a>>,
        {
            let this = this.get_unchecked_mut().as_mut_ptr();
            let arch = data.0.into();
            let vendor = data.1.into();
            let os = data.2.into();
            unsafe { triple::placement_new_from_arch_vendor_os(this, &arch, &vendor, &os) }
        }
        crate::Initializer::new(closure, data)
    }
}

impl<'a, Arch, Vendor, Os, Environment> From<(Arch, Vendor, Os, Environment)>
    for crate::Initializer<Triple, (Arch, Vendor, Os, Environment)>
where
    Arch: 'a + Into<Twine<'a>>,
    Vendor: 'a + Into<Twine<'a>>,
    Os: 'a + Into<Twine<'a>>,
    Environment: 'a + Into<Twine<'a>>,
{
    #[inline]
    fn from(data: (Arch, Vendor, Os, Environment)) -> Self {
        #[inline]
        unsafe fn closure<'a, Arch, Vendor, Os, Environment>(
            this: Pin<&mut MaybeUninit<Triple>>,
            data: (Arch, Vendor, Os, Environment),
        ) where
            Arch: 'a + Into<Twine<'a>>,
            Vendor: 'a + Into<Twine<'a>>,
            Os: 'a + Into<Twine<'a>>,
            Environment: 'a + Into<Twine<'a>>,
        {
            let this = this.get_unchecked_mut().as_mut_ptr();
            let arch = data.0.into();
            let vendor = data.1.into();
            let os = data.2.into();
            let environment = data.3.into();
            unsafe { triple::placement_new_from_arch_vendor_os_environment(this, &arch, &vendor, &os, &environment) }
        }
        crate::Initializer::new(closure, data)
    }
}
