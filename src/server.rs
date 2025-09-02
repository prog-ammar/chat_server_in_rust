use std::io::{Read};
use std::net::{TcpListener,TcpStream};


pub fn handle_connection(mut data: TcpStream)
{
    let mut data_string=String::new();
    match data.read(&mut data_string)
    {
        Ok(data) =>
        {
            println!("Recieved Data : {}",data_string);
        }
        Err(e) =>
        {
            println!("Error : {}",e);
        }
    }
}


pub fn listens()
{
  let listener= match TcpListener::bind("0.0.0.0::4443")
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
            // println!("Connection Successfull : {}",stream.peer_addr()?);
            handle_connection(data);
        }
        Err(e) =>
        {
            panic!("Error : {}",e);
        }
    }
  }
}

