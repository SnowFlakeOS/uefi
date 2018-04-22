use {Status, FILE_SYSTEM_GUID, Guid};

#[repr(C)]
pub struct SimpleFileSystem
{
	revision: u64,
	pub open_volume: efi_fcn!{ fn(&SimpleFileSystem, &mut *mut super::File) -> Status },
}

impl super::Protocol for SimpleFileSystem
{
	fn guid() -> Guid {
		FILE_SYSTEM_GUID
	}
	unsafe fn from_ptr(v: *const ::Void) -> *const Self {
		v as *const _
	}
}
