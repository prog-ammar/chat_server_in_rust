
mod server;
fn main() 
{
    println!("Started Listening on 4443 Port =>");
    server::listens("0.0.0.0:4443");

}
