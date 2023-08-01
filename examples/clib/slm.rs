pub mod binding;
use binding::{slm_init, slm_login, SLM_CALLBACK_VERSION02, SS_OK, ST_INIT_PARAM};

fn main() {
    println!("{}", SS_OK);
    let mut init_param: ST_INIT_PARAM = ST_INIT_PARAM {
        version: SLM_CALLBACK_VERSION02,
        pfn: None,
        password: convert_passwd(&"simple"),
        flag: 0,
        timeout: 86400,
    };
    unsafe {
        let result = slm_init(&mut init_param);
        println!("{}", result);
    }
}

fn get_u8_value(c: char) -> u8 {
    let char_list: [char; 16] = [
        '0', '1', '2', '3', '4', '5', '6', '7', '8', '9', 'A', 'B', 'C', 'D', 'E', 'F',
    ];
    for (index, tmp) in char_list.into_iter().enumerate() {
        if tmp == c {
            return index as u8;
        }
    }
    return 0;
}

fn convert_passwd(passwd: &str) -> [u8; 16] {
    let mut data: [u8; 16] = [0; 16];
    if passwd.len() != 32 {
        return data;
    }
    let bytes = passwd.as_bytes();
    for i in 0..32 {
        if i % 2 == 0 {
            data[i / 2] =  get_u8_value(bytes[i] as char) * 16 +  get_u8_value(bytes[i+1] as char)
        }
    }
    
    return data;
}
