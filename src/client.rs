use std::net::{TcpStream};
use std::io;
use std::io::{Read};
use std::io::{Write};
use std::io::stdout;
use std::thread;
use crossterm::{execute, terminal::{Clear, ClearType}, cursor::MoveUp};


pub fn connect(server_ip: &str)
{
    let mut server=TcpStream::connect(server_ip).expect("Enter An Valid Server IP");

    let reader_stream = server.try_clone().expect("Failed to clone stream");
    let writer_stream = server.try_clone().expect("Failed to clone stream");
    
        
    let  mut name=String::new();
    println!("Enter Your Username : ");
    io::stdin().read_line(&mut name).expect("Cant Read Name");
    execute!(stdout(),MoveUp(1), Clear(ClearType::CurrentLine)).unwrap();
    server.write_all(name.trim().as_bytes()).unwrap();
    
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
                panic!("Error : {}",e);
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
            io::stdin().read_line(&mut message).expect("Error has occured");
            execute!(stdout(),MoveUp(1), Clear(ClearType::CurrentLine)).unwrap();
            server.write_all(message.trim().as_bytes()).unwrap();  
            message.clear();    
        } 
}
