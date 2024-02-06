mod minifurball;
mod httpHandler;
use std::{
  io,
  io::{Read, Write},
  fs,
  path::Path,
  collections::HashMap,
  process::Command,
};

pub fn install(hosts: Vec<String>, packages: Vec<String>) -> std::io::Result<()> {

  for host in hosts {
    let host = host + ":2137";
    if host.contains(".") {

      for path in &packages {
        let path = "/packages/".to_owned() + path + ".furball";
      
        let response = httpHandler::Response::new(httpHandler::make_request(&host, &path).unwrap());
        println!("Got a response: \n{:#?}", response);


        let binding = response.attachment.clone();

        let mut path = binding.split("/").collect::<Vec<&str>>();

        let mut buffer=String::from("core/");


        for i in path {
          if !i.contains(".") {
            buffer = buffer.to_owned() + i + "/";
            fs::create_dir_all(buffer.to_string());
        } else {
            let mut f = fs::File::create("core/".to_owned() + &binding.clone())?;
            f.write_all(response.content.as_bytes())?;
            minifurball::decode(Path::new(&("core/".to_owned() + &binding.clone())), "core/packages/".to_string());

            fs::remove_file(&("core/".to_owned() + &binding.clone()));

            println!("Parsing config file: {}", "core/".to_owned() + &binding.clone().replace(".furball", "") + "/package.conf");
            let mut confile = fs::File::open("core/".to_owned() + &binding.clone().replace(".furball", "") + "/package.conf")?;
            let mut confstr = String::new();
            let _ = confile.read_to_string(&mut confstr).unwrap();

            let mut conf = parseConf(confstr.clone());

            println!("Compiling...");
            if conf.get("compiler").unwrap()=="cargo" {
              println!("{}", "core/".to_owned() + &binding.clone().replace(".furball", "") + "/Cargo.toml");
              let output = Command::new("cargo")
//                .current_dir("/home/foko/PackageManager/FokPkgMgr-client")
                .arg("build")
                .arg("--manifest-path") 
                .arg("core/".to_owned()+ &binding.clone().replace(".furball", "") + "/Cargo.toml")
                .output()
                .expect("Failed to execute cargo build");

              println!("{:#?}", output);
            }

            let outputmv = Command::new("mv")
              .arg("core/".to_owned() + &binding.clone().replace(".furball", "/") + conf.get("output").unwrap())
              .arg("core/packages/bin")
              .output()
              .expect("Failed to execute mv");
            println!("{:#?}", outputmv);

            //conf.output;

          }

        }

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
