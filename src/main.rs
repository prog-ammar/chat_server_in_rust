
mod server;
mod client;
use std::io;
use std::io::{ Write};
fn main() 
{
    let mut user_input=String::new();
    print!("1 => Start Server\n2 => Start Client\nEnter Choice : ");
    io::stdout().flush().unwrap();

    io::stdin().read_line(&mut user_input).expect("Failed To Read Line");

    let conv_input : i8= user_input.trim().parse::<i8>().expect("Please Enter A Valid Number");

    if conv_input == 1
    {
        let mut server_port= String::new();
        print!("\nEnter Port Number : ");
        io::stdout().flush().unwrap();
        io::stdin().read_line(&mut server_port).expect("Failed to read line");
        let port_number :i32 = server_port.trim().parse::<i32>().expect("Please Enter An Valid Port Number");
        let server_addr=format!("0.0.0.0:{}",port_number);
        server::listens(&server_addr);
    }
    else if conv_input == 2
    {
        let mut ip_of_server=String::new();
        print!("\nEnter Server IP with Port i.e x.x.x:xx : ");
        io::stdout().flush().unwrap();
        
        io::stdin().read_line(&mut ip_of_server).expect("Failed to read line");
        client::connect(&ip_of_server.trim());
    }
    else 
    {
        println!("You Entered Wrong Choice!");   
    }
}
