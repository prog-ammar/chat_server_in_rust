use std::collections::HashMap;
use std::io::{Read};
use std::net::{IpAddr, TcpListener, TcpStream};
use std::io::{Write};
use std::thread;
use std::sync::mpsc;

pub fn handle_connection(mut data: TcpStream)
{
    let mut ips_map:HashMap<IpAddr,String>=HashMap::new();

    let mut name=String::new();
    if ips_map.is_empty() == true || !ips_map.contains_key(&data.peer_addr().unwrap().ip())
    {
        println!("Connection Successfull : {}",data.peer_addr().unwrap().ip());
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
                    println!("{} Joined!",name);
                    ips_map.insert(client_ip,name.to_string());
                    let msg=format!("{} Joined\n",name);
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
    let mut buffer=[0;1];
    let mut data_present:bool =false;

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
            return;
        }
    }

    if data_present
    {
        let mut data_string=[0;1024];
        match data.read(&mut data_string)
        {
            Ok(bytes) => if bytes > 0
            {
                let recv_data=String::from_utf8_lossy(&data_string[0..bytes]).to_string();
                let msg=format!("{} : {}",name,recv_data);
                println!("{}",msg);
                data.write_all(msg.as_bytes()).unwrap();
            }
            Err(e) =>
            {
                println!("Error : {}",e);
                return;
            }
        }
    }
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

  

  for stream in listener.incoming()
  {
    match stream
    {
        Ok(data) =>
        {
            thread::spawn(move||
                {handle_connection(data)});
        }
        Err(e) =>
        {
            panic!("Error : {}",e);
        }
    }
  }
}

