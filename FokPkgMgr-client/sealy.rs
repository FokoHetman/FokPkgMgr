mod minifurball;
mod httpHandler;
use std::{
  path::Path,
  io,
  io::{Read, Write},
  fs,
  env,
  collections::HashMap,
  process::Command,
};



fn main() -> std::io::Result<()> {

  let host = "127.0.0.1:2137";


  let mut args: Vec<_> = env::args().collect();
  let mut confile = fs::File::open("configs/main.conf")?;
  let mut config = String::new();

  confile.read_to_string(&mut config).unwrap();


  if args.len()<2 {
    println!("No args");
    return Ok(());
  }


  
  let mut conf = parseConf(config);
  println!("{:#?}", conf);


  if !conf.contains_key("prefix") {
    println!("Config file doesn't have `prefix`. Defaulting to apt");
    conf.insert("prefix".to_string(), "apt".to_string());
  }


  let mut command = String::new();
  let mut prefixconf = String::new();
  let mut prefixconfile = fs::File::open("configs/".to_owned()+&conf.get("prefix").unwrap()+".conf")?;

  let _ = prefixconfile.read_to_string(&mut prefixconf).unwrap();

  let mut prefixconf = parseConf(prefixconf);
  println!("{:#?}", prefixconf);

  if prefixconf.get("upgrade").unwrap().contains(&args[1]) {
    command="upgrade".to_string();    
  } else {
    println!("Invalid command: {}", args[1]);
    return Ok(());
  }



  if command.contains("upgrade") {
    let mut path = String::from("/core.list");


    let response = httpHandler::Response::new(httpHandler::make_request(host, &path).unwrap());
    println!("Got a response: \n{:#?}", response);


    let binding = response.attachment.clone();

    let mut path = binding.split("/").collect::<Vec<&str>>();

    let mut buffer=String::new();

    for i in response.content.split("\n") {
      if i.contains("/") {
        let mut path = String::from(i);
        println!("Fetching {}", path);
        let response = httpHandler::Response::new(httpHandler::make_request(host, &path).unwrap());
        println!("Got a response: \n{:#?}", response);


        let mut binding = response.attachment.clone();
        let mut buffer = String::new();
        let mut path = binding.split("/").collect::<Vec<&str>>();


        for i in path {
          if i.contains(".") {
            buffer = buffer+i;
            fs::write(buffer.clone(), response.content.clone())?;
          } else {
            buffer = buffer+i+"/";
            fs::create_dir_all(buffer.clone())?;
          }
        }
        minifurball::decode(Path::new(&buffer));

        fs::remove_file(buffer)?;
      }
    }
  }




  Ok(())
}

fn parseConf(conf: String) -> HashMap<String, String> {
  let mut result: HashMap<String, String> = HashMap::new();
  for i in conf.split("\n") {
    if i.contains("=") {
      result.insert(i.to_string().split("=").collect::<Vec<&str>>()[0].to_string(),
      i.to_string().split("=").collect::<Vec<&str>>()[1].split("\"").collect::<Vec<&str>>()[1].to_string());
    }
  }

  return result;  
}
