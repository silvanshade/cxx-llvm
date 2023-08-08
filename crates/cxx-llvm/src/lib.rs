mod auto;
mod ffi {
    pub(crate) mod llvm {
        pub(crate) mod adt {
            pub(crate) mod hash_code;
            pub(crate) mod small_vector;
            pub(crate) mod small_vector_impl;
            pub(crate) mod string_ref;
            pub(crate) mod triple;
            pub(crate) mod twine;
        }
    }
}
mod gen {
    pub(crate) mod llvm {
        pub(crate) mod adt {
            pub(crate) mod hash_code;
            pub(crate) mod string_ref;
            pub(crate) mod triple;
            pub(crate) mod twine;
        }
    }
}
pub mod llvm {
    pub mod adt {
        pub mod hash_code {
            pub use crate::ffi::llvm::adt::hash_code::HashCode;
        }
        pub mod small_vector {
            pub use crate::ffi::llvm::adt::small_vector::{small_vector_element, SmallVector, SmallVectorElement};
        }
        pub mod small_vector_impl {
            pub use crate::ffi::llvm::adt::small_vector_impl::{IterPin, SmallVectorImpl, SmallVectorImplElement};
        }
        pub mod string_ref {
            pub use crate::ffi::llvm::adt::string_ref::StringRef;
        }
        pub mod triple {
            pub use crate::ffi::llvm::adt::triple::{
                ArchType,
                EnvironmentType,
                OSType,
                ObjectFormatType,
                SubArchType,
                Triple,
                VendorType,
            };
        }
        pub mod twine {
            pub use crate::ffi::llvm::adt::twine::Twine;
        }
    }
    pub use crate::ffi::llvm::adt::{
        hash_code::HashCode,
        small_vector::SmallVector,
        small_vector_impl::SmallVectorImpl,
        string_ref::StringRef,
        triple::Triple,
        twine::Twine,
    };
}

use core::{mem::MaybeUninit, pin::Pin};

pub struct Initializer<This, Data> {
    pub(crate) call: unsafe fn(Pin<&mut MaybeUninit<This>>, Data),
    pub(crate) data: Data,
}

impl<This, Data> Initializer<This, Data> {
    #[inline]
    pub(crate) fn new(call: unsafe fn(Pin<&mut MaybeUninit<This>>, Data), data: Data) -> Self {
        Self { call, data }
    }
}

unsafe impl<This, Data> moveref::New for Initializer<This, Data> {
    type Output = This;

    #[inline]
    unsafe fn new(self, this: Pin<&mut MaybeUninit<Self::Output>>) {
        (self.call)(this, self.data)
    }
}

pub mod casting {
    pub trait DynCast<T> {
        fn dyn_cast(&self) -> Option<&T>;
    }
}
