use simple_server::{Server, StatusCode};

fn main() {
    let server_1 = Server::new(|req, mut res|{
        println!("request received {} {} {:?}",req.method(), req.uri(), req.body());

        match(req.method().as_str(), req.uri().path()){
            
            ("GET" , "/" )=> Ok(res.body(String::from("response from get / request").into_bytes())?),
            ("GET" , "/piaic" )=> Ok(res.body(String::from("response from get / piaic").into_bytes())?), 
            ("POST" , "/hello" )=> Ok(res.body(String::from("response from post /hello req").into_bytes())?), 
            (_,_) =>  Ok(res.body(String::from("request not found").into_bytes())?),
        }
    });

    server_1.listen("127.0.0.1", "5000");
}
