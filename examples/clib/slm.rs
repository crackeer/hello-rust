pub mod binding;
use binding::{SS_OK, ST_INIT_PARAM, SLM_CALLBACK_VERSION02, slm_init, slm_login};

fn main() {
    println!("{}", SS_OK);
    let mut init_param : ST_INIT_PARAM = ST_INIT_PARAM{
        version :SLM_CALLBACK_VERSION02,
        pfn: None,
        password: convert_passwd(&"sim"),
        flag: 0,
        timeout:86400,
    };
    unsafe {
        let result = slm_init(&mut init_param);
        println!("{}",result);
    }
    

}

fn convert_passwd(passwd : &str) -> [u8;16] {
    let data : [u8;16] = [0;16];
    return data
}


