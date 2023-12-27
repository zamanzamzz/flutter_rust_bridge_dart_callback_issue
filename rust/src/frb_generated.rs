// This file is automatically generated, so please do not edit it.
// Generated by `flutter_rust_bridge`@ 2.0.0-dev.4.

#![allow(
    non_camel_case_types,
    unused,
    non_snake_case,
    clippy::needless_return,
    clippy::redundant_closure_call,
    clippy::redundant_closure,
    clippy::useless_conversion,
    clippy::unit_arg,
    clippy::unused_unit,
    clippy::double_parens,
    clippy::let_and_return,
    clippy::too_many_arguments
)]

// Section: imports

use flutter_rust_bridge::for_generated::byteorder::{NativeEndian, ReadBytesExt, WriteBytesExt};
use flutter_rust_bridge::for_generated::transform_result_dco;
use flutter_rust_bridge::{Handler, IntoIntoDart};

// Section: boilerplate

flutter_rust_bridge::frb_generated_boilerplate!();

// Section: executor

flutter_rust_bridge::frb_generated_default_handler!();

// Section: wire_funcs

fn wire_async_greet_impl(
    port_: flutter_rust_bridge::for_generated::MessagePort,
    name: impl CstDecode<String> + core::panic::UnwindSafe,
) {
    FLUTTER_RUST_BRIDGE_HANDLER.wrap_async::<flutter_rust_bridge::for_generated::DcoCodec, _, _, _>(
        flutter_rust_bridge::for_generated::TaskInfo {
            debug_name: "async_greet",
            port: Some(port_),
            mode: flutter_rust_bridge::for_generated::FfiCallMode::Normal,
        },
        move || {
            let api_name = name.cst_decode();
            move |context| async move {
                transform_result_dco(
                    (move || async move { crate::api::simple::async_greet(api_name).await })()
                        .await,
                )
            }
        },
    )
}
fn wire_async_greet_with_callback_impl(
    port_: flutter_rust_bridge::for_generated::MessagePort,
    name: impl CstDecode<String> + core::panic::UnwindSafe,
    logger: impl CstDecode<flutter_rust_bridge::DartOpaque> + core::panic::UnwindSafe,
) {
    FLUTTER_RUST_BRIDGE_HANDLER.wrap_async::<flutter_rust_bridge::for_generated::DcoCodec, _, _, _>(
        flutter_rust_bridge::for_generated::TaskInfo {
            debug_name: "async_greet_with_callback",
            port: Some(port_),
            mode: flutter_rust_bridge::for_generated::FfiCallMode::Normal,
        },
        move || {
            let api_name = name.cst_decode();
            let api_logger = decode_DartFn_Inputs_String_Output_unit(logger.cst_decode());
            move |context| async move {
                transform_result_dco(
                    (move || async move {
                        crate::api::simple::async_greet_with_callback(api_name, api_logger).await
                    })()
                    .await,
                )
            }
        },
    )
}
fn wire_async_no_await_greet_impl(
    port_: flutter_rust_bridge::for_generated::MessagePort,
    name: impl CstDecode<String> + core::panic::UnwindSafe,
    logger: impl CstDecode<flutter_rust_bridge::DartOpaque> + core::panic::UnwindSafe,
) {
    FLUTTER_RUST_BRIDGE_HANDLER.wrap_async::<flutter_rust_bridge::for_generated::DcoCodec, _, _, _>(
        flutter_rust_bridge::for_generated::TaskInfo {
            debug_name: "async_no_await_greet",
            port: Some(port_),
            mode: flutter_rust_bridge::for_generated::FfiCallMode::Normal,
        },
        move || {
            let api_name = name.cst_decode();
            let api_logger = decode_DartFn_Inputs_String_Output_unit(logger.cst_decode());
            move |context| async move {
                transform_result_dco(
                    (move || async move {
                        crate::api::simple::async_no_await_greet(api_name, api_logger).await
                    })()
                    .await,
                )
            }
        },
    )
}
fn wire_greet_impl(
    name: impl CstDecode<String> + core::panic::UnwindSafe,
) -> flutter_rust_bridge::for_generated::WireSyncRust2DartDco {
    FLUTTER_RUST_BRIDGE_HANDLER.wrap_sync::<flutter_rust_bridge::for_generated::DcoCodec, _>(
        flutter_rust_bridge::for_generated::TaskInfo {
            debug_name: "greet",
            port: None,
            mode: flutter_rust_bridge::for_generated::FfiCallMode::Sync,
        },
        move || {
            let api_name = name.cst_decode();
            transform_result_dco((move || {
                Result::<_, ()>::Ok(crate::api::simple::greet(api_name))
            })())
        },
    )
}

// Section: related_funcs

fn decode_DartFn_Inputs_String_Output_unit(
    dart_opaque: flutter_rust_bridge::DartOpaque,
) -> impl Fn(String) -> flutter_rust_bridge::DartFnFuture<()> {
    use flutter_rust_bridge::IntoDart;

    async fn body(dart_opaque: flutter_rust_bridge::DartOpaque, arg0: String) -> () {
        let args = vec![arg0.into_into_dart().into_dart()];
        let message = FLUTTER_RUST_BRIDGE_HANDLER
            .dart_fn_invoke(dart_opaque, args)
            .await;
        <()>::sse_decode_single(message)
    }

    move |arg0: String| {
        flutter_rust_bridge::for_generated::convert_into_dart_fn_future(body(
            dart_opaque.clone(),
            arg0,
        ))
    }
}

// Section: dart2rust

impl CstDecode<u8> for u8 {
    fn cst_decode(self) -> u8 {
        self
    }
}
impl CstDecode<usize> for usize {
    fn cst_decode(self) -> usize {
        self
    }
}
impl SseDecode for anyhow::Error {
    fn sse_decode(deserializer: &mut flutter_rust_bridge::for_generated::SseDeserializer) -> Self {
        unimplemented!("not yet supported in serialized mode, feel free to create an issue");
    }
}

impl SseDecode for flutter_rust_bridge::DartOpaque {
    fn sse_decode(deserializer: &mut flutter_rust_bridge::for_generated::SseDeserializer) -> Self {
        let mut inner = <usize>::sse_decode(deserializer);
        return unsafe { flutter_rust_bridge::for_generated::sse_decode_dart_opaque(inner) };
    }
}

impl SseDecode for String {
    fn sse_decode(deserializer: &mut flutter_rust_bridge::for_generated::SseDeserializer) -> Self {
        let mut inner = <Vec<u8>>::sse_decode(deserializer);
        return String::from_utf8(inner).unwrap();
    }
}

impl SseDecode for Vec<u8> {
    fn sse_decode(deserializer: &mut flutter_rust_bridge::for_generated::SseDeserializer) -> Self {
        let mut len_ = <i32>::sse_decode(deserializer);
        let mut ans_ = vec![];
        for idx_ in 0..len_ {
            ans_.push(<u8>::sse_decode(deserializer));
        }
        return ans_;
    }
}

impl SseDecode for u8 {
    fn sse_decode(deserializer: &mut flutter_rust_bridge::for_generated::SseDeserializer) -> Self {
        deserializer.cursor.read_u8().unwrap()
    }
}

impl SseDecode for () {
    fn sse_decode(deserializer: &mut flutter_rust_bridge::for_generated::SseDeserializer) -> Self {}
}

impl SseDecode for usize {
    fn sse_decode(deserializer: &mut flutter_rust_bridge::for_generated::SseDeserializer) -> Self {
        deserializer.cursor.read_u64::<NativeEndian>().unwrap() as _
    }
}

impl SseDecode for i32 {
    fn sse_decode(deserializer: &mut flutter_rust_bridge::for_generated::SseDeserializer) -> Self {
        deserializer.cursor.read_i32::<NativeEndian>().unwrap()
    }
}

impl SseDecode for bool {
    fn sse_decode(deserializer: &mut flutter_rust_bridge::for_generated::SseDeserializer) -> Self {
        deserializer.cursor.read_u8().unwrap() != 0
    }
}

// Section: rust2dart

impl SseEncode for anyhow::Error {
    fn sse_encode(self, serializer: &mut flutter_rust_bridge::for_generated::SseSerializer) {
        <String>::sse_encode(format!("{:?}", self), serializer);
    }
}

impl SseEncode for flutter_rust_bridge::DartOpaque {
    fn sse_encode(self, serializer: &mut flutter_rust_bridge::for_generated::SseSerializer) {
        <usize>::sse_encode(self.encode(), serializer);
    }
}

impl SseEncode for String {
    fn sse_encode(self, serializer: &mut flutter_rust_bridge::for_generated::SseSerializer) {
        <Vec<u8>>::sse_encode(self.into_bytes(), serializer);
    }
}

impl SseEncode for Vec<u8> {
    fn sse_encode(self, serializer: &mut flutter_rust_bridge::for_generated::SseSerializer) {
        <i32>::sse_encode(self.len() as _, serializer);
        for item in self {
            <u8>::sse_encode(item, serializer);
        }
    }
}

impl SseEncode for u8 {
    fn sse_encode(self, serializer: &mut flutter_rust_bridge::for_generated::SseSerializer) {
        serializer.cursor.write_u8(self).unwrap();
    }
}

impl SseEncode for () {
    fn sse_encode(self, serializer: &mut flutter_rust_bridge::for_generated::SseSerializer) {}
}

impl SseEncode for usize {
    fn sse_encode(self, serializer: &mut flutter_rust_bridge::for_generated::SseSerializer) {
        serializer
            .cursor
            .write_u64::<NativeEndian>(self as _)
            .unwrap();
    }
}

impl SseEncode for i32 {
    fn sse_encode(self, serializer: &mut flutter_rust_bridge::for_generated::SseSerializer) {
        serializer.cursor.write_i32::<NativeEndian>(self).unwrap();
    }
}

impl SseEncode for bool {
    fn sse_encode(self, serializer: &mut flutter_rust_bridge::for_generated::SseSerializer) {
        serializer.cursor.write_u8(self as _).unwrap();
    }
}

#[cfg(not(target_family = "wasm"))]
#[path = "frb_generated.io.rs"]
mod io;
#[cfg(not(target_family = "wasm"))]
pub use io::*;

/// cbindgen:ignore
#[cfg(target_family = "wasm")]
#[path = "frb_generated.web.rs"]
mod web;
#[cfg(target_family = "wasm")]
pub use web::*;
