#![allow(clippy::missing_safety_doc)]
#![allow(clippy::type_complexity)]
#![feature(const_fn, const_fn_union, const_raw_ptr_deref, const_size_of_val, core_intrinsics)]

pub mod const_config;
pub mod const_http;
mod const_sys;
pub mod epoll;
pub mod extern_http_date;
pub mod http_content_length;
mod listener;
pub mod util;

pub use faf_pico_sys::phr_header;

#[macro_export]
macro_rules! const_concat_bytes {
    () => {
        ""
    };
    ($a:expr) => {
        $a
    };
    ($a:expr, $b:expr) => {{
        let bytes: &'static [u8] = unsafe {
            &$crate::util::concat::<
                [u8; $a.len()],
                [u8; $b.len()],
                [u8; $a.len() + $b.len()],
            >($a, $b)
        };

        unsafe { $crate::util::transmute(bytes) }
    }};
    ($a:expr, $($rest:expr),*) => {{
        pub const TAIL: &[u8] = const_concat_bytes!($($rest),*);
        const_concat_bytes!($a, TAIL)
    }};
    ($a:expr, $($rest:expr),*,) => {
        const_concat_bytes!($a, $($rest),*)
    };
}

#[macro_export]
macro_rules! likely {
   ($expr: expr) => {
      std::intrinsics::likely($expr)
   };
}

#[macro_export]
macro_rules! unlikely {
   ($expr: expr) => {
      std::intrinsics::unlikely($expr)
   };
}
