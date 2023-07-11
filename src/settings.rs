pub struct Settings{
    pub ip:String,
    
    pub port:String,
    
    pub is_debug:bool,
    
    pub templates:String,
}

impl Default for Settings {

    fn default() -> Self {
        Settings {
            ip:"0.0.0.0".to_string(),
            port:"8080".to_string(),
            is_debug:true,
            templates:"templates".to_string()
        }
    }
}

pub fn def_settings() -> Settings {
    Settings{..Default::default()}
}
pub fn new(ip:String, port:String, is_debug:bool, templates:String) -> Settings {
    Settings{ip:ip ,port:port, is_debug:is_debug, templates:templates}
}

