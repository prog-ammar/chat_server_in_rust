

use std::net::{TcpStream};
use std::io;
use std::io::{Read};
use std::io::{Write};
use std::thread;
use rpassword::read_password;

extern crate rpassword;

pub fn connect(server_ip: &str)
{
    let mut server=TcpStream::connect(server_ip).expect("Enter An Valid Server IP");

    let reader_stream = server.try_clone().expect("Failed to clone stream");
    let writer_stream = server.try_clone().expect("Failed to clone stream");
    

    let mut recv_data=[0;1024];

    let mut first_time_connect:bool = false;

    let mut buf=[0;1];

    match server.peek(&mut buf)
    {
        Ok(0) => 
        {
            println!("Server Disconnected!");
        }
        Ok(_) => 
        {
            first_time_connect=true;
        }
        Err(e) =>
        {
            panic!("Error : {}",e);
        }
    }
    
    if first_time_connect==true
    {
        match server.read(&mut recv_data)
        {
            Ok(bytes) if bytes > 0 =>
            {
                let received = String::from_utf8_lossy(&recv_data[..bytes]);
                print!("{}",received);
                io::stdout().flush().unwrap();
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
        let  mut name=String::new();
        io::stdin().read_line(&mut name).expect("Cant Read Name");
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
    }
    
    let receive_handle=thread::spawn(move||{
        receive_data(reader_stream);
    });

    let send_handle=thread::spawn(move||{
        send_data(writer_stream);
    });

    receive_handle.join().unwrap();
    send_handle.join().unwrap();
}



pub fn receive_data(mut server: TcpStream)
{
    let mut rec_data=[0;1024];
    let mut buffer=[0;1];
    let mut data_present:bool=false;
    loop
    {
        match server.peek(&mut buffer)
        {
            Ok(0) => 
            {
                data_present=false;
            }
            Ok(_) => 
            {
                data_present=true;
            }
            Err(e) =>
            {
                println!("Error : {}",e);
            }
        }
        if data_present
        {
            match server.read(&mut rec_data)
            {
                Ok(bytes) if bytes > 0 =>
                {
                    let received = String::from_utf8_lossy(&rec_data[..bytes]).to_string();
                    println!("{}",received);
                }
                Ok(_) =>
                {
                    println!("Disconnected");
                }
                Err(e) =>
                {
                    println!("Error : {}",e);
                }
            }  
        }
    }
}


pub fn send_data(mut server: TcpStream)
{
        let mut message=String::new();
        loop
        {
            print!("You : ");
            io::stdout().flush().unwrap();
            io::stdin().read_line(&mut message).expect("Error has occured");
            match server.write_all(message.trim().as_bytes())
            {
                Ok(_) =>
                {
                    
                }
                Err(e) =>
                {
                    println!("Error : {}",e);
                }
            }  
            message.clear();    
        } 
}
