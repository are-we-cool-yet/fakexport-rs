#![no_std]

#[macro_export]
macro_rules! fake {
    ( $( $i:ident )+ ) => {
        $(
            ::paste::paste! {
                #[export_name = ::core::stringify!($i)]
                #[allow(non_snake_case)]
                pub extern fn [<$i _FAKE>]() -> *mut ::core::ffi::c_void {
                    unsafe { ::core::mem::transmute::<extern "C" fn() -> *mut ::core::ffi::c_void, *mut ::core::ffi::c_void>([<$i _FAKE>]) }
                }
            }
        )+
    };
}
