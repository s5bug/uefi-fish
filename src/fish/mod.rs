mod u16str;

use core::mem;
use core::mem::MaybeUninit;
use uefi_raw::protocol::console::{InputKey, SimpleTextInputProtocol, SimpleTextOutputProtocol};
use uefi_raw::table::boot::BootServices;
use uefi_raw::table::system::SystemTable;
use uefi_raw::{Event, Handle, Status};

pub use u16str::U16String;

#[repr(transparent)]
pub struct FishEvent(Event);

#[repr(transparent)]
pub struct FishHandle(Handle);

#[repr(transparent)]
pub struct FishSystemTable(SystemTable);

impl FishSystemTable {
    pub fn stdout(&self) -> &mut FishSimpleTextOutputProtocol {
        unsafe {
            let inner = self.0.stdout;
            let wrapper: *mut FishSimpleTextOutputProtocol = mem::transmute_copy(&inner);
            &mut *wrapper
        }
    }
    pub fn stdin(&self) -> &mut FishSimpleTextInputProtocol {
        unsafe {
            let inner = self.0.stdin;
            let wrapper: *mut FishSimpleTextInputProtocol = mem::transmute_copy(&inner);
            &mut *wrapper
        }
    }
    pub fn boot_services(&self) -> &mut FishBootServices {
        unsafe {
            let inner = self.0.boot_services;
            let wrapper: *mut FishBootServices = mem::transmute_copy(&inner);
            &mut *wrapper
        }
    }
}

#[repr(transparent)]
pub struct FishSimpleTextOutputProtocol(SimpleTextOutputProtocol);

impl FishSimpleTextOutputProtocol {
    pub fn output_string<const LEN: usize>(&mut self, string: &U16String<LEN>) -> Status {
        unsafe {
            let unwrapper: *mut SimpleTextOutputProtocol = mem::transmute_copy(&self);

            let str_as_ptr = string.as_ptr();

            let f = self.0.output_string;

            f(unwrapper, str_as_ptr)
        }
    }
}

#[repr(transparent)]
pub struct FishSimpleTextInputProtocol(SimpleTextInputProtocol);

impl FishSimpleTextInputProtocol {
    pub fn reset(&mut self, extended_verification: bool) -> Status {
        unsafe {
            let unwrapper: *mut SimpleTextInputProtocol = mem::transmute_copy(&self);

            let f = self.0.reset;

            f(unwrapper, extended_verification)
        }
    }

    pub fn read_key_stroke(&mut self, input_key: &mut MaybeUninit<InputKey>) -> Status {
        unsafe {
            let unwrapper: *mut SimpleTextInputProtocol = mem::transmute_copy(&self);

            let f = self.0.read_key_stroke;

            f(unwrapper, input_key.as_mut_ptr())
        }
    }

    pub fn wait_for_key(&self) -> FishEvent {
        FishEvent(self.0.wait_for_key)
    }
}

#[repr(transparent)]
pub struct FishBootServices(BootServices);

impl FishBootServices {
    pub fn wait_for_event<const COUNT: usize>(
        &self,
        events: &mut [FishEvent; COUNT],
        out_index: &mut usize,
    ) -> Status {
        unsafe {
            let f = self.0.wait_for_event;

            let wrapped = events.as_mut_ptr();

            f(COUNT, mem::transmute_copy(&wrapped), out_index)
        }
    }
}
