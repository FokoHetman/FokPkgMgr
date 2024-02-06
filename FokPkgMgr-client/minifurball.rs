use std::{
  env,
  fs,
  io::{Read, Write},
  path::Path,
  io,
  io::prelude::*,
  io::BufReader,
  fs::File,
  convert::TryInto,
  path::PathBuf,
};


pub fn decode(path: &Path) -> io::Result<()> {


    let mut file = fs::File::open(path)?;
    let mut content = String::new();

    file.read_to_string(&mut content);

    let mut buffer:String = String::new();
    for i in content.chars() {
      buffer = buffer + &i.to_string();


      if buffer.contains("[FURDIR-HEADER]") && buffer.contains("[/FURDIR-HEADER]") {

          let mut fork = buffer.replace("[FURDIR-HEADER]","")
              .replace("[/FURDIR-HEADER]","");

          let locs: Vec<&str> = fork.split(&format!("fur-{}=", "location")).collect::<Vec<&str>>();

          let loc: &str = &locs[1].replace("``\"", "").replace("\"``","");

 
          fs::create_dir(loc);
      

        buffer = "".to_string();
      }

      if buffer.contains("[FURFILE-HEADER]") && buffer.contains("[/FURFILE-HEADER]") {
        let mut fork= buffer.replace("[FURFILE-HEADER]","")
            .replace("[/FURFILE-HEADER]","");

        let locs: Vec<&str> = fork.split(&format!(";{};", "FUR-BONE")).collect::<Vec<&str>>();


        let loc: &str = &locs[0].split(&format!("fur-{}=", "location")).collect::<Vec<&str>>()[1].replace("\"``", "").replace("``\"", "");

        let src: &str = &locs[1].split(&format!("data{}", "eq")).collect::<Vec<&str>>()[1];

        println!("{}", loc);
        let mut nwrite = fs::File::create(loc)?;

        nwrite.write_all(src.as_bytes());


        buffer = "".to_string();
      }
    }
    Ok(())
}
