use std::{net::{TcpListener,TcpStream}, io::{Write, Read}, fs, thread, time::Duration, os, path, env, io};

mod treadpool;
use treadpool::*;
mod reqres;
use reqres::*;



/* connection/request handling*/

pub fn listen(ip:String, port:String) -> io::Result<()>{

    let mut dir = String::new();

    if cfg!(target_os = "windows"){
        dir = env::var("PATH").unwrap();
    }else {
        dir = env::var("PWD").unwrap();
    }
    ///switch to BaseDIR
    if dir.ends_with("/bin") {
        
        env::set_current_dir("../../");
        
    }else if dir.ends_with("/src"){
        
        env::set_current_dir("../");
    }
    else{}


    let listener = TcpListener::bind(format!("{}:{}",ip,port)).map_err(|_| format!(
        "cant bind address {} on {} port",ip,port)).unwrap();
    
    let pool = ThreadPool::new(4);

    for stream in listener.incoming(){
        
        pool.execute(|| {handle_connection(stream)});
        
    }
    Ok(())

}

fn handle_connection(stream:io::Result<TcpStream>) {

    match stream {

        Ok(mut stream) => {

            
            let mut buf: [u8; 1024] = [0;1024]; //fill array with 1024 zeros

            stream.read(&mut buf).unwrap();
            

            //GET request handling
            if buf.starts_with(b"GET") {
                       
                if route(buf, "/".to_string() ,Method::GET){

                    let response = Response{
                        status:"200".to_string(),
                        content:"./assets/index.html".to_string(),
                        
                    };

                                       
                    match stream.write(response.create().as_bytes()) {
                        Ok(_) => stream.flush().unwrap(),
                        Err(_) => println!("Connection reseted (maybe it's an attack)")

                    };
                }

                else if route(buf, "/info".to_string(), Method::GET){
                    let response = Response{
                        content:"./assets/info.html".to_string(),
                        status:"200".to_string()
                    };

                    match stream.write(response.create().as_bytes()) {
                        Ok(_) => stream.flush().unwrap(),
                        Err(_) => println!("Connection reseted (maybe it's an attack)")
                    }
                }
                else {

                    let response = Response{
                        content:"./assets/404.html".to_string(),
                        status:"200".to_string()
                    };


                    match stream.write(response.create().as_bytes()) {
                        Ok(_) => stream.flush().unwrap(),
                        Err(_) => println!("Connection reseted (maybe it's an attack)")

                    };

                }
            

            //POST
            }else if buf.starts_with(b"POST"){
                if route(buf, "/info".to_string(), Method::POST) {
                    
                    let response = Response{
                        content:"./assets/info.html".to_string(),
                        status:"200".to_string()
                    };

                    let post = POST{
                            request:buf
                        };
                    
                    for (k,v) in post.all(){
                        print!("{}:{} \n",k,v)
                    }
                    

                    match stream.write(response.create().as_bytes()) {
                        Ok(_) => stream.flush().unwrap(),
                        Err(_) => println!("Connection reseted (maybe it's an attack)")

                    };


                }
            }

            
        },

        Err(_) => println!("cant connect")
    }
}
