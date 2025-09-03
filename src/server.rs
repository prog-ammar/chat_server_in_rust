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
        Ok(mut data) =>
        {
            // println!("Connection Successfull : {}",stream.peer_addr()?);
            // let client_ip = ToString::to_string(&data.peer_addr().unwrap().ip());
            data.write_all(b"Enter Your Name : ").unwrap();
            let mut client_name=[0;1024];
            match data.read(&mut client_name)
            {
                Ok(b) => if b > 0
                {
                    let name=String::from_utf8_lossy(&client_name[..b]);
                    println!("{} Joined!",name);
                    let msg=format!("{} Joined\n",name);
                    data.write_all(msg.as_bytes()).unwrap();
                }
                Err(e) =>
                {
                    println!("Error has Occured!");
                }
            };
            // ips_map.insert(client_ip,client_name.clone());
            handle_connection(data);
        }
        Err(e) =>
        {
            panic!("Error : {}",e);
        }
    }
  }
}

