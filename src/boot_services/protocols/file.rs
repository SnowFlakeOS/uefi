use runtime_services::Time;
use {Status, Guid};

pub const FILE_MODE_READ: u64 = 1;
pub const FILE_MODE_WRITE: u64 = 2;
pub const FILE_MODE_CREATE: u64 = 1 << 63;

pub const FILE_READ_ONLY: u64 = 0x01;
pub const FILE_HIDDEN: u64 = 0x02;
pub const FILE_SYSTEM: u64 = 0x04;
pub const FILE_RESERVED: u64 = 0x08;
pub const FILE_DIRECTORY: u64 = 0x10;
pub const FILE_ARCHIVE: u64 = 0x20;

#[repr(C)]
pub struct FileInfo {
    pub Size: u64,
    pub FileSize: u64,
    pub PhysicalSize: u64,
    pub CreateTime: Time,
    pub LastAccessTime: Time,
    pub ModificationTime: Time,
    pub Attribute: u64,
    pub FileName: [u16; 256],
}

impl Default for FileInfo {
    fn default() -> Self {
        FileInfo {
            Size: Default::default(),
            FileSize: Default::default(),
            PhysicalSize: Default::default(),
            CreateTime: Default::default(),
            LastAccessTime: Default::default(),
            ModificationTime: Default::default(),
            Attribute: Default::default(),
            FileName: [0; 256],
        }
    }
}

#[repr(C)]
pub struct File
{
	revision: u64,
	pub open: efi_fcn!{ fn(&File, &mut *mut File, *const u16, u64, u64) -> Status },
	pub close: efi_fcn!{ fn(&mut File) -> Status },
	pub delete: efi_fcn!{ fn(&mut File) -> Status },
	pub read : efi_fcn!{ fn(&mut File, &mut usize, *mut ::Void) -> Status },
	pub write: efi_fcn!{ fn(&mut File, &mut usize, *const ::Void) -> Status },
	pub get_position: efi_fcn!{ fn(&File, &mut u64) -> Status },
	pub set_position: efi_fcn!{ fn(&mut File, u64) -> Status },
    pub get_info: efi_fcn!{ fn(&mut File, &Guid, &mut usize, *mut u8) -> Status },
    pub set_info: efi_fcn!{ fn(&mut File, &Guid, &mut usize, *const u8) -> Status },
    pub flush: efi_fcn!{ fn(&mut File) -> Status }
}
