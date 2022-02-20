use core::ops::Deref;

use super::{Endpoint, RemoteAddr, ConnectOpts};

// Safety:
// pointer is not null once inited(comes from an immutable ref)
// pointee memory is always valid during the eventloop

macro_rules! ptr_wrap {
    ($old: ident,$new: ident) => {
        #[derive(Clone, Copy)]
        pub struct $new {
            ptr: *const $old,
        }

        unsafe impl Send for $new {}
        unsafe impl Sync for $new {}

        impl AsRef<$old> for $new {
            #[inline]
            fn as_ref(&self) -> &$old {
                unsafe { &*self.ptr }
            }
        }

        impl Deref for $new {
            type Target = $old;

            #[inline]
            fn deref(&self) -> &Self::Target {
                self.as_ref()
            }
        }

        impl From<&$old> for $new {
            fn from(ptr: &$old) -> Self {
                $new { ptr }
            }
        }
    };
}

ptr_wrap!(Endpoint, EndpointRef);
ptr_wrap!(RemoteAddr, RemoteAddrRef);
ptr_wrap!(ConnectOpts, ConnectOptsRef);
