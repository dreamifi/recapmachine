
use pyo3::prelude::*;
use winapi::um::winuser;
use winapi::um::wingdi;
use winapi::ctypes::{c_int, c_void};
use winapi::shared::minwindef::{UINT, DWORD};

#[pymodule]
fn drimage(_py: Python, module: &PyModule) -> PyResult<()> {
	
    #[pyfn(module, "capture")]
    fn capture(_py: Python) -> PyResult<String> {	
		let width : c_int = 50;
		let height : c_int = 50;
		let offset_x : c_int = 0;
		let offset_y : c_int = 0;
		let no_bits_ptr = std::ptr::null_mut::<Vec::<c_int>>();
		let mut bitmapinfo = wingdi::BITMAPINFO {
			bmiHeader: wingdi::BITMAPINFOHEADER {
				biSize: std::mem::size_of::<wingdi::BITMAPINFOHEADER>() as DWORD,
				biWidth: width,
				biHeight: height,
				biPlanes: 1,
				biBitCount: 0, // we want GetDIBits to change these values
				biCompression: wingdi::BI_RGB,
				biSizeImage: 0,
				biXPelsPerMeter: 0,
				biYPelsPerMeter: 0,
				biClrUsed: 0,
				biClrImportant: 0
			},
			bmiColors: [wingdi::RGBQUAD{
				rgbBlue: 0,
				rgbGreen: 0,
				rgbRed: 0,
				rgbReserved: 0}]
		};
		let bitmapinfo_ptr : wingdi::LPBITMAPINFO = &mut bitmapinfo;
		unsafe {
			let desktop_window = winuser::GetDesktopWindow();
			let screen_dc = winuser::GetDC(desktop_window);
			let memory_dc = wingdi::CreateCompatibleDC(screen_dc);
			let win_bitmap 
				= wingdi::CreateCompatibleBitmap(screen_dc, width, height);
			let original_bitmap 
				= wingdi::SelectObject(memory_dc, win_bitmap as *mut c_void);
			// copy image from screen to windows bitmap
			if 	wingdi::BitBlt(memory_dc, 0 as c_int, 0 as c_int, width, 
				height, screen_dc, offset_x, offset_y, wingdi::SRCCOPY) != 0 
			{
				// deselect windows bitmap
				wingdi::SelectObject(memory_dc, original_bitmap);
				// initialize bitmapinfo values
				if	wingdi::GetDIBits(memory_dc, win_bitmap, 0 as UINT, 
					height as UINT, no_bits_ptr as *mut c_void, bitmapinfo_ptr,
					wingdi::DIB_RGB_COLORS) != 0 
				{
					// copy bits of windows bitmap to rust memory
					wingdi::GetDIBits(memory_dc, win_bitmap, 0 as UINT, 
						height as UINT, , bitmapinfo_ptr, 
						wingdi::DIB_RGB_COLORS) != 0 
				}
			} 
			else {
				wingdi::SelectObject(memory_dc, original_bitmap);
			}
			wingdi::DeleteObject(win_bitmap as *mut c_void);
			wingdi::DeleteDC(memory_dc);
			winuser::ReleaseDC(desktop_window, screen_dc);
		}
		
        Ok(String::from("hello python!"))
    }
    Ok(())
}