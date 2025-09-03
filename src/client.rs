use std::net::{TcpStream};
use std::io;
use std::io::{Read};
use std::io::{Write};
use std::thread;

fn connect(server_ip: &str)
{
    let mut server=TcpStream::connect(server_ip).unwrap();
    let mut name=String::new();

    let mut recv_data=[0;1024];

    match server.read(&mut recv_data)
    {
        Ok(bytes) if bytes > 0 =>
        {
            let received = String::from_utf8_lossy(&recv_data[..bytes]);
            println!("{}",received);
        }
        Ok(_) =>
        {
            println!("Connection Closed");
        }
        Err(e) =>
        {
            panic!("Error : {}",e)
        }
    }
    
    io::stdin().read_line(&mut name).expect("Error has occured");
    match server.write_all(name.trim().as_bytes())
    {
        Ok(_) =>
        {
            println!("");
        }
        Err(e) =>
        {
            panic!("Error : {}",e)
        }
    }

    let handle=thread::spawn(||
    {

        let mut rec_data=[0;1024];

        match server.read(&mut rec_data)
        {
            Ok(bytes) if bytes > 0 =>
            {
                let received = String::from_utf8_lossy(&rec_data[..bytes]);
                println!("{}",received);
            }
            Ok(_) =>
            {
                println!("Connection Closed");
            }
            Err(e) =>
            {
                panic!("Error : {}",e)
            }
        }   
    });
}

fn main()
{
    connect("192.168.1.70:4443");
}