//! UEFI Interface Crate
//!
//! Provides FFI access to a UEFI environment for UEFI Applications and bootloaders
//!
//! ```no_run
//! #[no_mangle]
//! pub extern "win64" fn efi_main(_image_handle: ::uefi::Handle, system_table: &::uefi::SystemTable) -> ::uefi::Status
//! {
//!     system_table.con_out.output_string_utf8("Hello, world.");
//!     ::uefi::status::SUCCESS
//! }
//! ```
#![no_std]
#![crate_name="uefi"]
#![crate_type="lib"]
#![feature(unique)]
#![feature(try_trait)]	// Makes Status a little easier to use
#![feature(ptr_internals)]	// rawptr as_ref

pub use self::str16::Str16;
pub use self::str16::{CStr16Ptr, CStr16};

pub use self::con::{EfiLogger};
pub use self::con::{InputKey, SimpleInputInterface, SimpleTextOutputInterface};

pub use self::status::Status;

macro_rules! efi_fcn {
	(fn $name:ident ( $($n:ident: $t:ty),* ) -> $rv:ty) => {
		extern "win64" fn $name( $($n: $t),* ) -> $rv
	};
	(fn ( $($n:ident: $t:ty),* ) -> $rv:ty) => {
		unsafe extern "win64" fn( $($n: $t),* ) -> $rv
	};
	(fn ( $($t:ty),* ) -> $rv:ty) => {
		unsafe extern "win64" fn( $($t),* ) -> $rv
	};
}

mod con;
mod str16;
pub mod status;
pub mod runtime_services;
pub mod boot_services;

// libstd miniature clones
pub mod borrow;

pub enum Void {}
pub type Handle = *mut Void;
pub type PhysicalAddress = u64;
pub type VirtualAddress = u64;

/// GUID
pub const GRAPHICS_OUTPUT_PROTOCOL_GUID: Guid = Guid(0x9042a9de, 0x23dc, 0x4a38, [0x96,0xfb,0x7a,0xde,0xd0,0x80,0x51,0x6a]);
pub const FILE_SYSTEM_GUID: Guid = Guid(0x964e5b22, 0x6459, 0x11d2, [0x8e, 0x39, 0x00, 0xa0, 0xc9, 0x69, 0x72, 0x3b]);
pub const FILE_INFO_ID: Guid = Guid(0x09576e92, 0x6d3f, 0x11d2, [0x8e, 0x39, 0x00, 0xa0, 0xc9, 0x69, 0x72, 0x3b]);
pub const ACPI_TABLE_GUID: Guid = Guid(0xeb9d2d30, 0x2d88, 0x11d3, [0x9a, 0x16, 0x00, 0x90, 0x27, 0x3f, 0xc1, 0x4d]);
pub const ACPI_20_TABLE_GUID: Guid = Guid(0x8868e871, 0xe4f1, 0x11d3, [0xbc, 0x22, 0x00, 0x80, 0xc7, 0x3c, 0x88, 0x81]);
pub const SMBIOS_TABLE_GUID: Guid = Guid(0xeb9d2d31, 0x2d88, 0x11d3, [0x9a, 0x16, 0x00, 0x90, 0x27, 0x3f, 0xc1, 0x4d]);
pub const SMBIOS3_TABLE_GUID: Guid = Guid(0xf2fd1544, 0x9794, 0x4a2c, [0x99, 0x2e, 0xe5, 0xbb, 0xcf, 0x20, 0xe3, 0x94]);

#[derive(Copy, Clone, Debug)]
pub struct Guid( pub u32, pub u16, pub u16, pub [u8; 8] );

#[macro_export]
/// Log to the provided UEFI SimpleTextOutputInterface sink
macro_rules! loge {
	($l:expr, $($t:tt)*) => {{
		use ::core::fmt::Write;
		let mut logger = $crate::EfiLogger::new($l);
		let _ = write!(&mut logger, "[{}] ", module_path!());
		let _ = write!(&mut logger, $($t)*); 
	}};
}

#[repr(C)]
/// Header for a UEFI table
pub struct TableHeader
{
	pub signature: u64,
	pub revision: u32,
	pub header_size: u32,
	pub crc32: u32,
	_reserved: u32,
}

#[repr(C)]
/// Size+Pointer array pointer
pub struct SizePtr<T>
{
	count: usize,
	data: *const T,
}
impl<T> ::core::ops::Deref for SizePtr<T>
{
	type Target = [T];
	fn deref(&self) -> &[T] {
		// SAFE: (assumed) from FFI and defined to be correct
		unsafe {
			::core::slice::from_raw_parts(self.data, self.count)
		}
	}
}

#[repr(C)]
/// System Table (top-level EFI structure)
///
/// A pointer to this is passed by the environment to the application as the second parameter to `efi_main`
pub struct SystemTable<'a>
{
	pub hdr: TableHeader,

	pub firmware_vendor: CStr16Ptr,
	pub firmware_revision: u32,

	pub console_in_handle: Handle,
	pub con_in: &'a SimpleInputInterface,

	pub console_out_handle: Handle,
	pub con_out: &'a SimpleTextOutputInterface,

	pub standard_error_handle: Handle,
	pub std_err: &'a SimpleTextOutputInterface,

	/// Runtime-acessible UEFI services (avaliable after `boot_services.exit_boot_services` has been called)
	pub runtime_services: *const runtime_services::RuntimeServices,
	pub boot_services: &'a boot_services::BootServices,

	pub configuraton_table: SizePtr<ConfigurationTable>
}
impl<'a> SystemTable<'a>
{
	#[inline]
	pub fn firmware_vendor(&self) -> &Str16 {
		unsafe {
			Str16::from_nul_terminated(self.firmware_vendor)
		}
	}
	#[inline]
	pub fn con_in(&self) -> &SimpleInputInterface {
		self.con_in
	}
	#[inline]
	pub fn con_out(&self) -> &SimpleTextOutputInterface {
		self.con_out
	}
	#[inline]
	pub fn std_err(&self) -> &SimpleTextOutputInterface {
		self.std_err
	}

	#[inline]
	pub fn runtime_services(&self) -> &runtime_services::RuntimeServices {
		unsafe { &*self.runtime_services }
	}
	#[inline]
	pub fn boot_services(&self) -> &boot_services::BootServices {
		self.boot_services
	}
	#[inline]
	pub fn configuraton_table(&self) -> &[ConfigurationTable] {
		&self.configuraton_table[..]
	}
}

#[derive(Copy, Clone, Debug)]
pub struct ConfigurationTable
{
	pub vendor_guid: Guid,
	pub vendor_table: *const Void,
}



