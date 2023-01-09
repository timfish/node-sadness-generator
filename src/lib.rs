use napi_derive::napi;

#[napi]
pub fn raise_abort() {
    unsafe {
        sadness_generator::raise_abort();
    }
}

#[napi]
pub fn raise_bus() {
    unsafe {
        sadness_generator::raise_bus();
    }
}

#[napi]
pub fn raise_floating_point_exception() {
    unsafe {
        sadness_generator::raise_floating_point_exception();
    }
}

#[napi]
pub fn raise_illegal_instruction() {
    unsafe {
        sadness_generator::raise_illegal_instruction();
    }
}

#[napi]
pub fn raise_segfault() {
    unsafe {
        sadness_generator::raise_segfault();
    }
}

#[napi]
pub fn raise_stack_overflow() {
    unsafe {
        sadness_generator::raise_stack_overflow();
    }
}

#[napi]
pub fn raise_stack_overflow_in_non_rust_thread(uses_longjmp: bool) {
    unsafe {
        sadness_generator::raise_stack_overflow_in_non_rust_thread(uses_longjmp);
    }
}

#[napi]
pub fn raise_stack_overflow_in_non_rust_thread_normal() {
    unsafe {
        sadness_generator::raise_stack_overflow_in_non_rust_thread_normal();
    }
}

#[napi]
pub fn raise_trap() {
    unsafe {
        sadness_generator::raise_trap();
    }
}
