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
                        template:Some(format!("./{}/index.html",settings.templates).to_string()),
                        context: None // context should be Some(HashMap<String,String>)
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
                        template:Some(format!("./{}/info.html",settings.templates).to_string()),, can be None if you dont want to send a respond after a post request
                        status:"200".to_string()
                        context:None //should be Non
                    };

                    let post = POST{  //getting data from post requst
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

# sending context to template

`src/lib.rs`

```

if route(buf, "/".to_string() ,Method::GET){

                    let response = Response{
                        status:"200".to_string(),
                        template:Some(format!("./{}/index.html",settings.templates).to_string()),

                        /// key("ip".to_string()) and value(settings.ip) you should add key in your html code
                        with %%key%% format and then BackToFront function can read it and replace it with the value associated with key///

                        context: Some(HashMap::from([("ip".to_string(),settings.ip) 
                                                    ,("port".to_string(),settings.port)
                        ]))
                        // context should be Some(HashMap<String,String>)
                        
                    };


                                       
                    match stream.write(response.create().as_bytes()) {
                        Ok(_) => stream.flush().unwrap(),
                        Err(_) => println!("Connection reseted (maybe it's an attack)")

                    };
                }
}
```

`templates/index.html` 

note : replace `index.html` with template that you defined a view for it

```
<h1>Your <span style="color: red;">krabTTp</span>🦀 website is running on %%ip%%:%%port%%</h1> 

```
in this code im replacing ip and port key with their defined values in context and using *%%key%%* format


# last thing
this project is not even near to be completed but i'm working On it 
and i can use your help for making this better💚🤍❤
