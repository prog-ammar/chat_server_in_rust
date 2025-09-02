use std::collections::HashMap;
use std::io::{Read};
use std::net::{TcpListener,TcpStream};
use std::io::{Write};


pub fn handle_connection(mut data: TcpStream)
{
    let mut data_string=String::new();
    match data.read_to_string(&mut data_string)
    {
        Ok(e) =>
        {
            println!("Recieved Data : {} {}",data_string,e);
        }
        Err(e) =>
        {
            println!("Error : {}",e);
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

  let mut ips_map:HashMap<String,String>=HashMap::new();

  for stream in listener.incoming()
  {
    match stream
    {
        Ok(data) =>
        {
            // println!("Connection Successfull : {}",stream.peer_addr()?);
            let client_ip = ToString::to_string(&data.peer_addr().unwrap());
            let mut client= TcpStream::connect(&client_ip).unwrap();
            client.write_all(b"Enter Your Name : ").unwrap();
            let mut client_name=String::new();
            client.read_to_string(&mut client_name).unwrap();
            ips_map.insert(client_ip,client_name.clone());
            let msg=client_name+"Joined !";
            client.write_all(msg.as_bytes()).unwrap();

            handle_connection(data);
        }
        Err(e) =>
        {
            panic!("Error : {}",e);
        }
    }
  }
}

