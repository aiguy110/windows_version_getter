use windows::Win32::System::SystemInformation::{
    GetVersionExA,
    GetNativeSystemInfo,
    OSVERSIONINFOA,
    SYSTEM_INFO
};

fn main() {
    // Get OS Version Info
    let mut ver_info: OSVERSIONINFOA = Default::default();
    let ver_info_ptr = (&mut ver_info) as *mut OSVERSIONINFOA;
    
    let success = unsafe { GetVersionExA(ver_info_ptr).as_bool() };
    if !success {
        println!("Error calling GetVersionExA. Aborting.");
        return;
    }
    println!("Got OS Version Info: {:?}", ver_info);

    // Get "NativeSystemInfo" (only care about dwProcessorType field)
    let mut sys_info: SYSTEM_INFO = Default::default();
    let sys_info_ptr = (&mut sys_info) as *mut SYSTEM_INFO; 

    unsafe { GetNativeSystemInfo(sys_info_ptr) };
    if !success {
        println!("Error calling GetNativeSystemInfo. Aborting.");
        return;
    }
    println!("Got Processor Arch: {:?}", sys_info.dwProcessorType);
}
