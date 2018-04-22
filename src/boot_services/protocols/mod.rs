//
//
/// Various object protocols

pub use self::loaded_image::LoadedImage;
pub use self::loaded_image_device_path::LoadedImageDevicePath;
pub use self::device_path::DevicePath;
pub use self::simple_file_system::SimpleFileSystem;
pub use self::graphics_output::{GraphicsOutput, PixelFormat};

pub use self::file::*;

pub trait Protocol
{
	fn guid() -> ::Guid;
	unsafe fn from_ptr(*const ::Void) -> *const Self;
}

mod loaded_image;
mod loaded_image_device_path;
mod device_path;
mod simple_file_system;

mod graphics_output;
mod file;

