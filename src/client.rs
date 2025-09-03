use std::net::{TcpStream};
use std::io;
use std::io::{Read};
use std::io::{Write};
use std::thread;

fn connect(server_ip: &str)
{
    let mut server=TcpStream::connect(server_ip).unwrap();
    let mut name=String::new();

    let reader_stream = server.try_clone().expect("Failed to clone stream");
    let writer_stream = server.try_clone().expect("Failed to clone stream");

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

    let _handle=thread::spawn(move||
    {
        let mut server=reader_stream;
        loop
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
                println!("idk");
            }
            Err(e) =>
            {
                println!("Error : {}",e);
            }
        }  
        }
    });
    

    let _new_handle=thread::spawn(move||
    {
        let mut server=writer_stream;
        let mut message=String::new();
        loop
        {
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
                
        }  
    });

}

fn main()
{
    connect("192.168.1.70:4443");
}