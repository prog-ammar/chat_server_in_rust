use std::io::{Read};
use std::io;
use std::net::{ TcpListener, TcpStream};
use std::io::{Write};
use std::thread;
use std::sync::{Arc, Mutex};
use local_ip_address::{local_ip};


pub fn handle_connection(mut data: TcpStream, clients:Arc<Mutex<Vec<TcpStream>>>)
{
    let mut name=String::new();
    println!("\nConnection Successfull : {}\n",data.peer_addr().unwrap());
    
    let mut client_name=[0;1024];
    match data.read(&mut client_name)
    {
        Ok(b) => if b > 0
        {
            name=String::from_utf8_lossy(&client_name[..b]).to_string();
            let msg=format!("\n{} Joined!\n",name);
            println!(" {} ",msg);
            send_to_clients(/*&data,*/ &clients, &msg);
        }
        Err(_e) =>
        {
            println!("Disconnected Error : 3");
        }
    }

    loop
    {
        let mut data_string=[0;1024];
        match data.read(&mut data_string)
        {
            Ok(0) =>
            {
                break;
            }
            Ok(bytes) => if bytes > 0
            {
                let recv_data=String::from_utf8_lossy(&data_string[0..bytes]).to_string();
                let msg=format!("{} : {}",name.trim(),recv_data.trim());
                println!("{}",msg);
                send_to_clients(/*&data,*/ &clients, &msg);
            }
            Err(_e) =>
            {
                let msg=format!("\n{} Disconnected, IP : {}\n ",name,data.peer_addr().unwrap());
                print!("{}",msg);
                io::stdout().flush().unwrap();
                clients.lock().unwrap().retain(|x| x.peer_addr().unwrap() != data.peer_addr().unwrap());
                send_to_clients(&clients, &msg );
                return;
            }
        }
    }
}   


pub fn send_to_clients(/*current_client:&TcpStream,*/clients:&Arc<Mutex<Vec<TcpStream>>>,data : &str)
{
    let mut clients_idk=clients.lock().unwrap();

    for  client in clients_idk.iter_mut() 
    {
        // if client.peer_addr().unwrap().ip() != current_client.peer_addr().unwrap().ip()
        // {
            client.write_all(data.as_bytes()).unwrap();
        // }
    }
}

pub fn listens(ip_address: &str)
{
   
  let listener= match TcpListener::bind(ip_address)
  {
    Ok(l) =>
    {
        l
    }
    Err(_e) =>
    {
        println!("Disconnted Error : 2");
        return;
    }
  };

  let local_ip=local_ip().unwrap().to_string();
  println!("\nStarted Chat Server on {}:{}",local_ip,ip_address.split(':').last().unwrap());


 let  clients=Arc::new(Mutex::new(Vec::new()));  

  for stream in listener.incoming()
  {
    match stream
    {
        Ok(data) =>
        {
            let clients_copy=Arc::clone(&clients);
           clients_copy.lock().unwrap().push(data.try_clone().unwrap());
           thread::spawn(move||
                {handle_connection(data,clients_copy)});
        }
        Err(_e) =>
        {
            println!("Disconnected Error : 1");
        }
    }
  }
}

