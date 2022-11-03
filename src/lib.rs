#![allow(non_camel_case_types)]

// mod attrs;
pub mod builder;
pub mod element;
pub mod value;
pub mod attrs;

pub use attribute::*;
pub use builder::*;
pub use element::*;

use wasm_bindgen::prelude::*;
use web_sys::Node;

use web_sys::Element;

#[used]
static mut MSG_PTR: usize = 0;
#[used]
static mut MSG_PTR_PTR: *const usize = unsafe { &MSG_PTR } as *const usize;
#[used]
static mut MSG_POS_UPDATED: u8 = 255;
#[used]
static mut MSG_METADATA_PTR: *const u8 = unsafe { &MSG_POS_UPDATED } as *const u8;
#[used]
static mut STR_PTR: usize = 0;
#[used]
static mut STR_PTR_PTR: *const usize = unsafe { &STR_PTR } as *const usize;
#[used]
static mut STR_LEN: usize = 0;
#[used]
static mut STR_LEN_PTR: *const usize = unsafe { &STR_LEN } as *const usize;

#[wasm_bindgen(module = "/interpreter_opt.js")]
// #[wasm_bindgen(module = "/interpreter.js")]
extern "C" {
    fn work_last_created();

    fn update_last_memory(mem: JsValue);

    fn last_needs_memory() -> bool;

    pub type JsInterpreter;

    #[wasm_bindgen(constructor)]
    pub(crate) fn new(
        mem: JsValue,
        msg_pos_updated_ptr: usize,
        msg_ptr: usize,
        str_ptr: usize,
        str_len_ptr: usize,
    ) -> JsInterpreter;

    #[wasm_bindgen(method)]
    pub(crate) fn UpdateMemory(this: &JsInterpreter, mem: JsValue);

    #[allow(unused)]
    #[wasm_bindgen(method)]
    pub(crate) fn NeedsMemory(this: &JsInterpreter) -> bool;

    #[wasm_bindgen(method)]
    pub(crate) fn SetNode(this: &JsInterpreter, id: u32, node: Node);

    #[allow(unused)]
    #[wasm_bindgen(method)]
    pub(crate) fn GetNode(this: &JsInterpreter, id: u32) -> Node;
}
