use windows::{Win32::System::WindowsProgramming::GetUserNameW, core::{HRESULT, PWSTR}};
use std::{slice::from_raw_parts, error::Error, usize};


fn get_current_user() -> Result<String, String> {
    let mut cb_buffer = 257_u32;
    let mut buffer = Vec::<u16>::with_capacity(cb_buffer as usize);
    let lp_buffer = PWSTR(buffer.as_mut_ptr());

    unsafe {
        let success = GetUserNameW(lp_buffer, &mut cb_buffer);

        if success.is_err() {return Err("Error".into())}

        let user_name = String::from_utf16_lossy(from_raw_parts(lp_buffer.0, cb_buffer as usize - 1));

        return Ok(user_name)
    }
}

fn main() {
    dbg!(get_current_user().unwrap());
}