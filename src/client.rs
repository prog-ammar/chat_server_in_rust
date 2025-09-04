use std::net::{TcpStream};
use std::io;
use std::io::{Read};
use std::io::{Write};
// use std::thread;

fn connect(server_ip: &str)
{
    let mut server=TcpStream::connect(server_ip).expect("Failed To Connect");

    // let reader_stream = server.try_clone().expect("Failed to clone stream");
    // let writer_stream = server.try_clone().expect("Failed to clone stream");
    
    let mut name=String::new();

    // server.set_nonblocking(true).unwrap();
    // reader_stream.set_nonblocking(true).unwrap();
    // writer_stream.set_nonblocking(true).unwrap();

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
            println!("Some Data is Avaliable");
            first_time_connect=true;
        }
        Err(e) =>
        {
            println!("Error : {}",e);
        }
    }
    
    if first_time_connect==true
    {
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
    }

    // let _handle=thread::spawn(move||
    // {
    //     let mut server=reader_stream;
    //     let mut rec_data=[0;1024];
    //     let mut buffer=[0;1];
    //     let mut data_present:bool=false;
    //     loop
    //     {

    //         match server.peek(&mut buffer)
    //         {
    //             Ok(0) => 
    //             {
    //                 data_present=false;
    //             }
    //             Ok(_) => 
    //             {
    //                 data_present=true;
    //             }
    //             Err(e) =>
    //             {
    //                 println!("Error : {}",e);
    //             }
    //         }

    //         if data_present
    //         {
    //             match server.read(&mut rec_data)
    //             {
    //                 Ok(bytes) if bytes > 0 =>
    //                 {
    //                     let received = String::from_utf8_lossy(&rec_data[..bytes]);
    //                     println!("{}",received);
    //                 }
    //                 Ok(_) =>
    //                 {
    //                     println!("idk");
    //                 }
    //                 Err(e) =>
    //                 {
    //                     println!("Error : {}",e);
    //                 }
    //             }  
    //         }
        
    //     }
    // });
    

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
            message.clear();    
        }  
}

fn main()
{
    connect("192.168.1.70:4443");
}