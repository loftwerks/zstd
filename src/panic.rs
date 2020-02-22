/*!
Panic handling for Zesty
*/

use core::panic::PanicInfo;

use crate::Console;

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
  Console::write(_info.into());

  loop {}
}

#[repr(u32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ExitCode {
  Success = 0x10,
  Failure = 0x11,
}

pub fn exit(exit_code: ExitCode) {
  use x86_64::instructions::port::Port;

  unsafe {
    let mut port = Port::new(0xf4);
    port.write(exit_code as u32);
  }
}
