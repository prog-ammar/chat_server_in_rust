use std::collections::HashMap;
use std::io::{Read};
use std::net::{IpAddr, TcpListener, TcpStream};
use std::io::{Write};
use std::thread;
use std::sync::{Arc, Mutex};
use local_ip_address::{local_ip};


pub fn handle_connection(mut data: TcpStream, clients:Arc<Mutex<Vec<TcpStream>>>)
{
    let mut ips_map:HashMap<IpAddr,String>=HashMap::new();

    let mut name=String::new();
    if ips_map.is_empty() == true || !ips_map.contains_key(&data.peer_addr().unwrap().ip())
    {
        println!("\nConnection Successfull : {}",data.peer_addr().unwrap().ip());
        let client_ip = data.peer_addr().unwrap().ip();
        data.write_all(b"Enter Your Name : ").unwrap();
        let mut buffer=[0;1];
        let mut data_present:bool=false;
        match data.peek(&mut buffer)
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
            let mut client_name=[0;1024];
            match data.read(&mut client_name)
            {
                Ok(b) => if b > 0
                {
                    name=String::from_utf8_lossy(&client_name[..b]).to_string();
                    ips_map.insert(client_ip,name.trim().to_string());
                    let msg=format!("\n{} Joined!\n",name);
                    send_to_clients(/*&data,*/ &clients, &msg);
                    data.write_all(msg.as_bytes()).unwrap();
                }
                Err(e) =>
                {
                    println!("Error : {}",e);
                }
            }
        }
    }

    loop
    {
        let mut data_string=[0;1024];
        match data.read(&mut data_string)
        {
            Ok(bytes) => if bytes > 0
            {
                let recv_data=String::from_utf8_lossy(&data_string[0..bytes]).to_string();
                let msg=format!("{} : {}",name.trim(),recv_data.trim());
                println!("{}",msg);
                send_to_clients(/*&data,*/ &clients, &msg);
            }
            Err(e) =>
            {
                println!("Error : {}",e);
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
    Err(e) =>
    {
        panic!("Error : {}",e);
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
        Err(e) =>
        {
            panic!("Error : {}",e);
        }
    }
  }
}

