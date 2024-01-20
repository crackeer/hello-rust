use std::ffi::CStr;
use libc::{dev_t, ioctl};
use udev::Context;
 
fn main() {
    let context = Context::new().unwrap();
    
    // 获取所有设备节点
    for device in context.enumerate_devices(Some("block")).expect("Failed to enumerate devices.") {
        if let Some(device) = device.into_result() {
            // 判断设备类型为块设备并且是否存在子系统属性
            if device.property_value("ID_BUS").is_some() && device.property_value("SUBSYSTEM").is_some() {
                // 获取设备名称
                let name = CStr::from_ptr((*device.syspath()).as_ref());
                
                println!("Device inserted: {}", name);
                
                // 如果需要进行更多操作，比如读写文件等，可以根据设备路径打开相应的文件或者调用其他函数
            }
        }
    }
}