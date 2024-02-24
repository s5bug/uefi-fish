#![no_main]
#![no_std]

mod fish;

use crate::fish::{FishEvent, FishHandle, FishSystemTable, U16String};
use core::mem::MaybeUninit;
use uefi_raw::protocol::console::InputKey;
use uefi_raw::Status;

#[export_name = "efi_main"]
unsafe extern "efiapi" fn main(image_handle: FishHandle, system_table: &FishSystemTable) -> Status {
    let mut rc: Status = Status::SUCCESS;

    let stdout = system_table.stdout();
    let stdin = system_table.stdin();

    let Some(hello_world) = U16String::<16>::from_str("Hello, world!\r\n") else {
        unreachable!()
    };
    rc = stdout.output_string(&hello_world);
    if rc.is_error() {
        return rc;
    }

    // consume all input
    rc = stdin.reset(false);
    if rc.is_error() {
        return rc;
    }

    // wait for a keystroke
    let boot_services = system_table.boot_services();
    let mut desired_events: [FishEvent; 1] = [stdin.wait_for_key()];
    let mut received_event: usize = usize::MAX;
    rc = boot_services.wait_for_event(&mut desired_events, &mut received_event);
    if rc.is_error() {
        return rc;
    }

    // consume it and exit
    let mut key: MaybeUninit<InputKey> = MaybeUninit::uninit();
    rc = stdin.read_key_stroke(&mut key);
    if rc.is_error() {
        return rc;
    }

    return Status::SUCCESS;
}

#[panic_handler]
fn panic_handler(info: &core::panic::PanicInfo) -> ! {
    loop {}
}
