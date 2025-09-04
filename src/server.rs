use std::collections::HashMap;
use std::io::{Read};
use std::net::{IpAddr, TcpListener, TcpStream};
use std::io::{Write};


pub fn handle_connection(mut data: TcpStream)
{
    let mut data_string=[0;1024];
    match data.read(&mut data_string)
    {
        Ok(bytes) => if bytes > 0
        {
            let recv_data=String::from_utf8_lossy(&data_string[0..bytes]);
            println!("Recieved Data : {}",recv_data);
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

  let mut ips_map:HashMap<IpAddr,String>=HashMap::new();

  for stream in listener.incoming()
  {
    match stream
    {
        Ok(mut data) =>
        {
            if ips_map.is_empty() == true || !ips_map.contains_key(&data.peer_addr().unwrap().ip())
            {
                println!("Connection Successfull : {}",data.peer_addr().unwrap().ip());
                let client_ip = data.peer_addr().unwrap().ip();
                data.write_all(b"Enter Your Name : ").unwrap();
                let mut client_name=[0;1024];
                match data.read(&mut client_name)
                {
                    Ok(b) => if b > 0
                    {
                        let name=String::from_utf8_lossy(&client_name[..b]);
                        println!("{} Joined!",name);
                        ips_map.insert(client_ip,name.to_string());
                        let msg=format!("{} Joined\n",name);
                        data.write_all(msg.as_bytes()).unwrap();
                    }
                    Err(e) =>
                    {
                        println!("Error : {}",e);
                    }
                };
            }
            else 
            {
                handle_connection(data);
            }
           
        }
        Err(e) =>
        {
            panic!("Error : {}",e);
        }
    }
  }
}

