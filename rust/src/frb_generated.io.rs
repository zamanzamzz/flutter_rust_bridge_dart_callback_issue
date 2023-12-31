// This file is automatically generated, so please do not edit it.
// Generated by `flutter_rust_bridge`@ 2.0.0-dev.10.

// Section: imports

use super::*;
use flutter_rust_bridge::for_generated::byteorder::{NativeEndian, ReadBytesExt, WriteBytesExt};
use flutter_rust_bridge::for_generated::transform_result_dco;
use flutter_rust_bridge::{Handler, IntoIntoDart};

// Section: dart2rust

impl CstDecode<anyhow::Error> for *mut wire_cst_list_prim_u_8 {
    fn cst_decode(self) -> anyhow::Error {
        unimplemented!()
    }
}
impl CstDecode<flutter_rust_bridge::DartOpaque> for *const std::ffi::c_void {
    fn cst_decode(self) -> flutter_rust_bridge::DartOpaque {
        unsafe { flutter_rust_bridge::for_generated::cst_decode_dart_opaque(self as _) }
    }
}
impl CstDecode<String> for *mut wire_cst_list_prim_u_8 {
    fn cst_decode(self) -> String {
        let vec: Vec<u8> = self.cst_decode();
        String::from_utf8(vec).unwrap()
    }
}
impl CstDecode<Vec<u8>> for *mut wire_cst_list_prim_u_8 {
    fn cst_decode(self) -> Vec<u8> {
        unsafe {
            let wrap = flutter_rust_bridge::for_generated::box_from_leak_ptr(self);
            flutter_rust_bridge::for_generated::vec_from_leak_ptr(wrap.ptr, wrap.len)
        }
    }
}
pub trait NewWithNullPtr {
    fn new_with_null_ptr() -> Self;
}

impl<T> NewWithNullPtr for *mut T {
    fn new_with_null_ptr() -> Self {
        std::ptr::null_mut()
    }
}

#[no_mangle]
pub extern "C" fn frbgen_flutter_rust_bridge_dart_callback_issue_dart_fn_deliver_output(
    call_id: i32,
    ptr_: *mut u8,
    rust_vec_len_: i32,
    data_len_: i32,
) {
    let message = unsafe {
        flutter_rust_bridge::for_generated::Dart2RustMessageSse::from_wire(
            ptr_,
            rust_vec_len_,
            data_len_,
        )
    };
    FLUTTER_RUST_BRIDGE_HANDLER.dart_fn_handle_output(call_id, message)
}

#[no_mangle]
pub extern "C" fn frbgen_flutter_rust_bridge_dart_callback_issue_wire_async_greet(
    port_: i64,
    name: *mut wire_cst_list_prim_u_8,
) {
    wire_async_greet_impl(port_, name)
}

#[no_mangle]
pub extern "C" fn frbgen_flutter_rust_bridge_dart_callback_issue_wire_async_greet_with_callback(
    port_: i64,
    name: *mut wire_cst_list_prim_u_8,
    logger: *const std::ffi::c_void,
) {
    wire_async_greet_with_callback_impl(port_, name, logger)
}

#[no_mangle]
pub extern "C" fn frbgen_flutter_rust_bridge_dart_callback_issue_wire_async_greet_with_callback_no_await(
    port_: i64,
    name: *mut wire_cst_list_prim_u_8,
    logger: *const std::ffi::c_void,
) {
    wire_async_greet_with_callback_no_await_impl(port_, name, logger)
}

#[no_mangle]
pub extern "C" fn frbgen_flutter_rust_bridge_dart_callback_issue_wire_init_app(port_: i64) {
    wire_init_app_impl(port_)
}

#[no_mangle]
pub extern "C" fn frbgen_flutter_rust_bridge_dart_callback_issue_wire_sync_greet(
    name: *mut wire_cst_list_prim_u_8,
) -> flutter_rust_bridge::for_generated::WireSyncRust2DartDco {
    wire_sync_greet_impl(name)
}

#[no_mangle]
pub extern "C" fn frbgen_flutter_rust_bridge_dart_callback_issue_cst_new_list_prim_u_8(
    len: i32,
) -> *mut wire_cst_list_prim_u_8 {
    let ans = wire_cst_list_prim_u_8 {
        ptr: flutter_rust_bridge::for_generated::new_leak_vec_ptr(Default::default(), len),
        len,
    };
    flutter_rust_bridge::for_generated::new_leak_box_ptr(ans)
}

#[repr(C)]
#[derive(Clone, Copy)]
pub struct wire_cst_list_prim_u_8 {
    ptr: *mut u8,
    len: i32,
}
