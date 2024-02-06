// FOK's GRILLIN'
// Feel free to edit how server works.
// However edited servers may not be accepted as official, without being monitored.

mod threading;
use threading::ThreadPool;
use std::{
  net::{TcpListener, TcpStream},
  io,
  fs,
  path::{Path, PathBuf},
  io::{prelude::*, BufReader},
  process::Command,
  thread,
  sync::{mpsc, Arc, Mutex},

  time::Duration,
};

fn main() {
  let listener = TcpListener::bind("0.0.0.0:2137").unwrap();
  let pool = ThreadPool::new(4);

  for stream in listener.incoming() {
    let stream = stream.unwrap();

    pool.execute(|| {handle_connection(stream);});
  }

}

fn handle_connection(mut stream: TcpStream) -> io::Result<()> {
  let buf_reader = BufReader::new(&mut stream);
  let request_line = buf_reader.lines().next().unwrap().unwrap();

  println!("{:#?}", request_line);

  let htmlreqs: Vec<&str> = ["GET / HTTP/1.1", "GET /sleep HTTP/1.1"].to_vec();
  //let mut all_reqs: Vec<(&str, (&str, &str))> = [("GET / HTTP/1.1", ("HTTP/1.1 200 OK", "templates/index.html"))].to_vec();
  let mut length: usize = 0;
  let mut contents: String = String::new();
  let mut status_line: &str = "";
  let mut filename: &str = "";

  println!("Syncing files...");
  let mut path = Path::new(".");
  let mut files: Vec<PathBuf> = list_dir(PathBuf::from(path))?;
  println!("{:#?}", files);
  let mut attachment=String::new();

  let (status_line, filename) = match &request_line[..] {
    
    "GET / HTTP/1.1" => ("HTTP/1.1 200 OK", "templates/index.html"),
    "GET /sleep HTTP/1.1" => {
      thread::sleep(Duration::from_secs(5));
      ("HTTP/1.1 200 OK", "templates/index.html")
    },
    _ => {
      //verify if request_line.replace("GET", "").replace("HTTP/1.1", "") exists as a file. If it does, copy it into stream.
      let mut returned: (&str, &str) = ("HTTP/1.1 404 NOT FOUND", "templates/404.html");
      for x in files {
        let strx = x.into_os_string().into_string().unwrap().replace("./", "/");
        // println!("{}:{}", strx, "");
        // println!("{strx}, {request_line} {}", request_line.contains(&strx));
        if request_line.replace("GET ", "").replace(" HTTP/1.1", "") == strx && &strx != "." {
          //let mut fx = fs::File::open(".".to_owned() + &strx)?;
          //io::copy(&mut fx, &mut stream);
          let mut count = 0;
          for i in strx.chars() {
            if count>0 {
              attachment+=i.to_string().as_str();
            }
            count+=1;
          }

          println!("Handling a file {}", attachment);

          returned = ("HTTP/1.1 200 OK", "templates/downloading.html");
        }
      }

//   ("HTTP/1.1 404 NOT FOUND", "templates/404.html");
     returned
    },
  };
  println!("{}", filename);

  contents = fs::read_to_string(filename).unwrap();
  length = contents.len();
  let mut response = String::new();
  if attachment!="" {
    let buf_content = fs::read_to_string(attachment.clone()).unwrap();
    length = buf_content.len();
    response = format!("{status_line}\r\n\
    Content-Disposition: attachment; filename=\"{attachment}\"\r\n\
    Content-Type: text/plain\r\n\
    Content-Length: {length}\r\n\r\n");

    stream.write_all(response.as_bytes()).unwrap();
    stream.write_all(&buf_content.as_bytes()).unwrap();
    stream.flush().unwrap();
  } else {
    response=format!("{status_line}\r\nContent-Length: {length}\r\n\r\n{contents}");
    stream.write_all(response.as_bytes()).unwrap();

  }

  //stream.write_all(response.as_bytes()).unwrap();
  Ok(())
}



fn list_dir(path: PathBuf) -> io::Result<Vec<PathBuf>> {
  let mut result: Vec<PathBuf> = [].to_vec();

  if path.is_dir() {
    for entry in fs::read_dir(path.clone())? {
      let mut npath = entry?.path();
      if npath.is_dir() {
        result.append(&mut list_dir(npath.clone())?);
      } else {
        result.push(npath.clone());
      }
    }
  }
  result.push(path.clone());


  return Ok(result)
}
