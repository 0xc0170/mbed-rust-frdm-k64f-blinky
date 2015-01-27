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

use core::marker::Copy;
use core::intrinsics::transmute;
use core::intrinsics::init;
use core::mem;
use core::slice::SliceExt;

#[link(name = "mbed", kind = "static")]
extern {
    pub fn gpio_write(obj: *mut u8, value: i32);
    pub fn gpio_init_out(obj: *mut u8, pin: u32);
    pub fn gpio_read(obj: *mut u8) -> i32;
    pub fn gpio_mode(obj: *mut u8, mode: i32);
    pub fn gpio_set(pin: i32) -> u32;
}

pub struct DigitalOut {
    gpio : [u8; 4]
}

impl DigitalOut {
    pub fn new(pin: u32) -> DigitalOut {
        unsafe {
            // let mut gpio_ptr : &'static [u8; 4] = mem::zeroed();
            // let gpio_out = DigitalOut { gpio: transmute(gpio_ptr)};
            // let array : [u8; 4] = [0; 4];
            let mut gpio_out : DigitalOut = init();
            // let mut gpio_out = DigitalOut { gpio: array}
            gpio_init_out(gpio_out.gpio.as_mut_ptr(), pin);
            gpio_out
        }
    }

    fn as_mut_ptr(&mut self) -> *mut u8 {
        self.gpio.as_mut_ptr()
    }

    pub fn write(&mut self, value: i32) {
        unsafe {
            gpio_write(self.gpio.as_mut_ptr(), value);
        }
    }

    pub fn read(&mut self, value: i32) -> i32 {
        unsafe {
            gpio_read(self.gpio.as_mut_ptr())
        }
    }
}
