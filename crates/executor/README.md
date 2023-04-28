# ISU CprE288 CyBot Project Template
This is a template project for running rust code on the ISU CyBot platform.
This template is a derivative of the template from the rust embedded folks [here](https://github.com/rust-embedded/cortex-m-quickstart).

## Getting Started
Follow the steps to install rust on your machine [here](https://www.rust-lang.org/tools/install);

### Installation
1. Run `rustup target add thumbv7em-none-eabihf` to add the Cortex-M4 with FPU target to your rust toolchain
2. Install [cargo-binutils](https://github.com/rust-embedded/cargo-binutils) with `cargo install cargo-binutils`
3. Follow OS specific instructions from the embedded rust book. You need openocd and the gdb/gcc for arm but qemu is optional.
    - [Linux](https://docs.rust-embedded.org/book/intro/install/linux.html)
    - [MacOS](https://docs.rust-embedded.org/book/intro/install/macos.html)
    - [Windows](https://docs.rust-embedded.org/book/intro/install/windows.html)
4. Optional but recommended install [gdbgui](https://www.gdbgui.com/installation/) to get a more visual debugging environment
5. Clone this repo to your machine or use [cargo-generate](https://github.com/cargo-generate/cargo-generate).
    1. `cargo generate --git https://github.com/Atrociously/cybot-template`
    2. `git clone https://github.com/Atrociously/cybot-template`

### The Starter Code
In the project you just downloaded in the `src/` directory is your main.rs which should look like this
```rust
#![no_std]
#![no_main]

use cybot::entry;

#[entry]
fn main() -> ! {
    // Your Code Goes Here!
    loop {}
}
```

### Running on the CyBot
1. Run `openocd` while connected to the cybot via USB.
2. In another CMD window run `gdb-multiarch target/thumbv7em-none-eabihf/OUTFILE` or `gdbgui -g arm-none-eabi-gdb`
3. Follow information [here](https://docs.rust-embedded.org/book/start/hardware.html) for debugging instructions.

### Looking at the docs
Most of the complex stuff is hidden within the [cybot](https://github.com/Atrociously/cybot) crate.
You can look at the docs by running `cargo doc --open` and looking for the cybot crate on the left.
