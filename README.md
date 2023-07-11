# krabTTp
Back-end framework/server with rust🦀


# Introduction
krabTTp🦀 is a (simple)rust framework for develooping back-end of your website

and it'll be more reliable, secure, advanced and productive in future


# Idea
i'm a Back-end Developer since 2021(noob) and i'm still learning about django and python so i came up with this idea to make a Back-end Development framework for Rust 
because i found rust so fast⚡, secure🔐 and intersint🙄 lang

# using


## views and urls
`lib.rs` after `handle_connection` function you can handle requests with views and defining urls like:
```
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
```
or
```
if route(buf, "/info".to_string(), Method::POST) {
                    
                    let response = Response{
                        content:"./assets/info.html".to_string(),
                        status:"200".to_string()
                    };

                    let post = POST{  //get data from post requst
                            request:buf
                        };
                    
                    for (k,v) in post.all(){
                        print!("{}:{} \n",k,v)
                    }
                    

                    match stream.write(response.create().as_bytes()) {
                        Ok(_) => stream.flush().unwrap(),
                        Err(_) => println!("Connection reseted (maybe it's an attack)")

                    };
```
for Post requests

# last thing
this project is not even near to be completed but i'm working On it 
and i can use your help for making this better💚🤍❤
