use std::fs;
use std::collections::HashMap;
use regex::*;

pub enum Method {
    GET,
    POST
}


pub fn route(buf:[u8; 1024], url:String, method:Method) -> bool {
    match method {
        Method::GET => {
            if buf.starts_with(format!("GET {} HTTP/1.1\r\n",url).as_bytes()){
                true
            }else{
                false
            }
        },
        Method::POST => {
            if buf.starts_with(format!("POST {} HTTP/1.1\r\n",url).as_bytes()){
                true
            }else{
                false
            }
        }
        
    }
    
}

pub struct Response {
    
    pub status: String, // return status code

    pub template: Option<String>, // path of html file ,in post request can be None
                        
    pub context: Option<HashMap<String,String>>

}

impl Default for Response {
    fn default() -> Response {
        Response {
            status: "".to_string(),
            
            template: None,

            context: None
        }
    }
}

impl Response {
    
    pub fn create(mut self) -> String{
        
        let mut template:String = String::new();

        match self.template {
            Some(data) => {
                template = fs::read_to_string(data).unwrap()
            },
            None => {}
        };

        
        match self.context {
            Some(data) => {
               template = back_to_front(template.clone(),data)
            },
            None => {}
        }
        
        if self.status == "200".to_string() {self.status = String::from("200 OK")
        }else if self.status == "201".to_string() {self.status = String::from("201 Created")
        }else if self.status == "302".to_string() {self.status = String::from("302 Found")
        }else if self.status == "400".to_string() {self.status = String::from("400 Bad Request")
        }else if self.status == "201".to_string() {self.status = String::from("401 Unauthorized")
        }else if self.status == "403".to_string() {self.status = String::from("403 Forbiden")
        }else if self.status == "404".to_string() {self.status = String::from("404 NOT FOUND")
        }else if self.status == "500".to_string() {self.status = String::from("500 Internal Server Error")
        }else {
            panic!("using not supported status")
        }
        
        let res = format!("HTTP/1.1 {}\r\nContent-Length: {}\r\n\r\n{}",
            self.status,
            template.len(),
            template
        );
        res
    }

}

pub struct POST {
    pub request:[u8; 1024]
}


//parsing post request

impl POST {
    pub fn all(&self) -> HashMap<String, String>{
        
        let str_buf = String::from_utf8_lossy(& self.request);

        let ind = str_buf.find("\r\n\r\n").unwrap();

        let mut raw_str = str_buf.to_string();

        raw_str.replace_range(..ind, "");

        let data:Vec<&str> = raw_str.split('&').collect();
        
        let mut dict = HashMap::new();

        for mut string in data {
            
            string = string.trim();
            
            let key_value:Vec<&str> = string.split('=').collect();

            dict.insert(key_value[0].to_string(),key_value[1].to_string());
        }
        dict
    }
    
    pub fn get(&self, key:String) -> (String, String) {
        
        let dict = self.all();
        
        
        for (k,v) in dict {
            if key == k {
                if k.to_lowercase() == "none" || v.to_lowercase() == "none".to_string() {
                    panic!("you cant pass none values, instead use Null")
                }
                return (k,v)
            }else{
                continue;
            }
        };
        ("None".to_string(),"None".to_string())
    }
}


fn back_to_front(template:String,context:HashMap<String, String>) -> String{
   
    let template:&str = template.as_str(); 
    
     
    
    let re = Regex::new(r"%%(\w+)%%").unwrap();

    let datas: Vec<&str> = re.find_iter(template).map(|m| m.as_str()).collect();

    let mut keys:Vec<&str> = Vec::new();
    
    let mut template_s = template.to_string();

    for i in datas{
        keys.push(i.split("%%").collect::<Vec<&str>>()[1])
    };
    
        
    for key in keys {

        match context.get(key) {
            
            Some(data) => {
                
                let ind = template_s.find(format!("%%{}%%",key).as_str()).unwrap();

                template_s.replace_range(ind..ind+key.len()+4, data);
 
            },
            None => {}
        }
    }

    template_s
    //println!("{}",template)

}
