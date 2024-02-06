mod core;
use std::{
  io,
  env,
  io::{Read, Write},
  fs,
};


fn main() -> io::Result<()> {

  let mut hosts: Vec<String> = Vec::new();
  let mut packages: Vec<String> = Vec::new();


  let args = env::args().collect::<Vec<String>>();
  if args.len()<2 {
    println!("Invalid argument len");
    return Ok(());
  }

  //so the whole thing doesn't explode :D
  let _ = fs::create_dir_all("core/packages/bin");

  //MAKE IT FROM THE CONFIG FILE THAT WAS CHOSEN
  if args[1]=="install" {
    let mut file = fs::File::open("configs/package-servers.conf")?;
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();

    for i in contents.split("\n") {
      if !i.contains("#") {
        hosts.push(i.to_string());
      }

    }

    let mut count=0;
    for i in args {
      if count>1 && !i.contains("--") {
        packages.push(i);
      }
      count = count + 1;
    }

    core::sources::hi::hi();
    println!("{:#?}, {:#?}", hosts, packages);
    core::sources::install::install(hosts, packages);
  }
  Ok(())
}
