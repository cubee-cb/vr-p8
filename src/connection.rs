/// with reference to pinput code

/// File name of the regular PICO-8 executable (not the one from a standalone cartridge).
#[cfg(windows)]
static PICO8_EXECUTABLE_NAME: &str = "pico8.exe";

/// File name of the regular PICO-8 executable (not the one from a standalone cartridge).
#[cfg(not(windows))]
static PICO8_EXECUTABLE_NAME: &str = "pico8";


pub fn hello()
{
    println!("hello there");
}
