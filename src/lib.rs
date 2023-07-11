use std::{net::{TcpListener,TcpStream}, io::{Write, Read}, fs, thread, time::Duration, os, path, env, io, collections::HashMap};

mod treadpool;
use treadpool::*;
mod reqres;
use reqres::*;
mod settings;
use settings::*;


pub fn handle_connection(stream:io::Result<TcpStream>){

    match stream {

        Ok(mut stream) => {

            
            let mut buf: [u8; 1024] = [0;1024]; //fill array with 1024 zeros

            stream.read(&mut buf).unwrap();
            
            let settings = def_settings();

            //GET request view
            if buf.starts_with(b"GET") {
                       
                if route(buf, "/".to_string() ,Method::GET){

                    let response = Response{
                        status:"200".to_string(),
                        template:format!("./{}/index.html",settings.templates).to_string(),
                        context: None // context should be Some(HashMap<String,String>)

                        
                    };

                                       
                    match stream.write(response.create().as_bytes()) {
                        Ok(_) => stream.flush().unwrap(),
                        Err(_) => println!("Connection reseted (maybe it's an attack)")

                    };
                }
                else {

                    let response = Response{
                        template:format!("./{}/404.html",settings.templates).to_string(),
                        status:"200".to_string(),
                        context:Some(HashMap::new())
                    };


                    match stream.write(response.create().as_bytes()) {
                        Ok(_) => stream.flush().unwrap(),
                        Err(_) => println!("Connection reseted (maybe it's an attack)")

                    };

                }
            

            //POST requests view
            }else if buf.starts_with(b"POST"){
                
            }

            
        },

        Err(_) => println!("cant connect")
    }
}





/* connection/request handling*/
pub fn listen() -> io::Result<()>{
    
    let settings = def_settings();

    let mut dir = String::new();

    if cfg!(target_os = "windows"){
        dir = env::var("PATH").unwrap();
    }else {
        dir = env::var("PWD").unwrap();
    }
    //switch to BaseDIR
    if dir.ends_with("/bin") {
        
        env::set_current_dir("../../");
        
    }else if dir.ends_with("/src"){
        
        env::set_current_dir("../");
    }
    else{}


    let listener = TcpListener::bind(format!("{}:{}",settings.ip,settings.port)).map_err(|_| format!(
        "cant bind address {} on {} port",settings.ip
        ,settings.port)).unwrap();
    
    let pool = ThreadPool::new(4);

    for stream in listener.incoming(){
        
        pool.execute(|| {handle_connection(stream)});
        
    }
    Ok(())

}

