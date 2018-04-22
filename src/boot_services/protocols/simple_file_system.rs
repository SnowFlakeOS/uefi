use {Status, FILE_SYSTEM_GUID, Guid};

#[repr(C)]
pub struct SimpleFileSystem
{
	revision: u64,
	open_volume: extern "win64" fn(&SimpleFileSystem, &mut *mut super::File) -> Status,
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

impl SimpleFileSystem
{
	pub fn open_volume(&self) -> Result< super::Owned<super::File>, Status > {
		let mut ptr = ::core::ptr::null_mut();
		(self.open_volume)(self, &mut ptr)
			// SAFE: Pointer passed to us for ownership
			.err_or_else(|| unsafe {super::Owned::from_ptr(ptr) } )
	}
}

