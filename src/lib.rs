use headless_chrome::{Browser, LaunchOptionsBuilder};
use std::ffi::CStr;
use std::os::raw::c_char;

fn generate_pdf_internal(url: *const c_char, output_path: *const c_char) -> Option<()> {
    let (url, output_path) = unsafe { (CStr::from_ptr(url), CStr::from_ptr(output_path)) };
    let url = url.to_str().ok()?;
    let output_path = output_path.to_str().ok()?;

    let browser = Browser::new(
        LaunchOptionsBuilder::default()
            .headless(true)
            .build()
            .ok()?,
    )
    .ok()?;
    let tab = browser.new_tab().ok()?;
    tab.navigate_to(url).ok()?;
    tab.wait_until_navigated().ok()?;
    std::fs::write(output_path, tab.print_to_pdf(None).ok()?).ok()?;
    Some(())
}

#[no_mangle]
pub extern "C" fn generate_pdf(url: *const c_char, output_path: *const c_char) -> bool {
    generate_pdf_internal(url, output_path).is_some()
}
