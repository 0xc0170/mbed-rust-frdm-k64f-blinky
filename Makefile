# Copyright (c) 2015, Martin Kojtal (0xc0170)
#
# Licensed under the Apache License, Version 2.0 (the "License");
# you may not use this file except in compliance with the License.
# You may obtain a copy of the License at
#
#     http://www.apache.org/licenses/LICENSE-2.0
#
# Unless required by applicable law or agreed to in writing, software
# distributed under the License is distributed on an "AS IS" BASIS,
# WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
# See the License for the specific language governing permissions and
# limitations under the License.

CC      := arm-none-eabi-gcc
AR      := arm-none-eabi-ar
LD      := arm-none-eabi-ld
RUSTC   := rustc
RUSTPKG := rustpkg

OPT  := 1 #There's a problem with optimization = 0
ARCH := thumbv7m
CPU  := cortex-m4

OBJCPY = arm-none-eabi-objcopy

RUSTFLAGS = -L . --target $(ARCH)-none-eabi -C target-cpu=$(CPU) 
RUSTFLAGS += -C relocation-model=static
RUSTFLAGS += -C opt-level=$(OPT) -g -Z no-landing-pads 
# RUSTFLAGS += -A dead_code -A unused_variables 

LDFLAGS  = -T K64FN1M0xxx12.ld
LDFLAGS += -Map=frdm-k64f-mbed-blinky.map
LDFLAGS += --gc-sections
LDFLAGS += #-print-gc-sections

.SUFFIXES: .o .rs .c

all: frdm-k64f-mbed-blinky.elf frdm-k64f-mbed-blinky.bin print_info

.rs.o:
	$(RUSTC) $(RUSTFLAGS) --emit obj -o $@ $<

frdm-k64f-mbed-blinky.elf: src/mbed-rust-frdm-k64-blinky.o 
	$(LD) $(LDFLAGS) -L . src/mbed-rust-frdm-k64-blinky.o board.o cmsis_nvic.o mbed_overrides.o retarget.o startup_MK64F12.o system_MK64F12.o -lmbed -o $@

frdm-k64f-mbed-blinky.bin: frdm-k64f-mbed-blinky.elf
	$(OBJCPY) -O binary $< $@

libcore: libcore.rlib

libcore.rlib:
	$(RUSTC) $(RUSTFLAGS) ../rust/src/libcore/lib.rs

print_info:
	arm-none-eabi-size --totals frdm-k64f-mbed-blinky.elf

.PHONY: clean

clean:
	rm -f *.o frdm-k64f-mbed-blinky.bin frdm-k64f-mbed-blinky.elf
