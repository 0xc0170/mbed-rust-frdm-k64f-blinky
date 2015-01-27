// Copyright 2015 Martin Kojtal (0xc0170)
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

#![no_std]
#![crate_type = "rlib"]
#![feature(lang_items, asm)]

#![allow(dead_code)]
#![allow(non_snake_case)]

extern crate core;

#[no_mangle]
pub fn _start() {
    main();
}

#[no_mangle]
pub fn __assert_func() {

}

#[no_mangle]
pub fn fprintf() {

}

#[no_mangle]
pub fn _impure_ptr() {

}

pub mod DigitalOut;

#[link(name = "mbed", kind = "static")]
extern {
    pub fn wait(sec: f32);
    pub fn mbed_die();
}

pub fn delay(mut cycles: u32) 
{
    while cycles > 0 {
        unsafe {
            asm!("nop" :::: "volatile");
        }
        cycles -= 1;
    }
}

#[no_mangle]
pub fn main() {
    let mut led = DigitalOut::DigitalOut::new((1 << 12) | 22);
    loop {
        led.write(1);
        delay(5000000);
        // wait(1.0);
        led.write(0);
        delay(5000000);
        // wait(1.0);
    }
}

#[doc(hidden)]
#[no_mangle]
pub extern fn __aeabi_unwind_cpp_pr0() {
    abort();
}

#[doc(hidden)]
#[no_mangle]
pub extern fn __aeabi_unwind_cpp_pr1() {
    abort();
}

#[no_mangle]
pub extern fn abort() -> ! {
    loop {} //TODO call mbed_die here
}

#[lang = "stack_exhausted"] extern fn stack_exhausted() {}
#[lang = "eh_personality"] extern fn eh_personality() {}

#[lang = "panic_fmt"]
extern fn panic_fmt(_args: &core::fmt::Arguments,
                   _file: &str,
                   _line: u32) -> ! {
    abort();
}
