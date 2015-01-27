# Blinky for FRDM K64F in rust with mbed

That's right mbed with rust! The mbed library consist of 2 parts - mbed API (C++ world) and mbed HAL - C layer (target + cmsis). The mbed library was built by GCC ARM, for K64F target. It's linked along with this application.

### Current status

It's still work in progress, as for example interrupts are not properly working, thus wait() function is commented out.

- mbed rust API in the separate module (DigitalOut will be there, not part of this app)
- use cargo for everything (+ build script if required)
- target definitions - to have a complete definition for K64F target, the PinNames or other enums/macros.
- DigitalOut should be generic - currently gpio array is set to 4 (sizeof gpio_t for K64F), either to use dynamic allocation by mbed, or rust
- to have multiple mbed targets supported 
