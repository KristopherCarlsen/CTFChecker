use std::net::{TcpListener, TcpStream};
use std::io::Write;
use std::process::{Command,Stdio};

pub mod config;

use crate::config::Config;

fn run_command(cmd: &str) -> bool{
    match Command::new("/bin/sh")
    .arg("-c")
    .arg(cmd)
    .stdout(Stdio::null())
    .stderr(Stdio::null())
    .status(){
        Ok(r) => r.success(),
        Err(_) => { 
            println!("ERROR: Failed to run shell command \"{}\".", cmd); 
            false
        },
    }
}

fn write_str(stream: &mut TcpStream, str: &str){
    match stream.write(str.as_bytes()){
        Ok(_) => {},
        Err(_) => { println!("WARNING: Failed to send value back to client.") },
    };
}

fn handle_client(stream: &mut TcpStream, config: &Config){
    let cmd_status = run_command(&config.cmd);

    // The flag is only given to the user if the shell script exits with success.
    if cmd_status{
        write_str(stream, &config.flag);
    }
    else{
        write_str(stream, &config.fail_msg);
    }

    // To properly end the message, send a newline and return to properly format the user's terminal.
    write_str(stream, "\n\r");
}

fn main() {
    let config = Config::load("ctfchecker.conf");
    config.print();

    let listener = TcpListener::bind(("0.0.0.0", config.port)).expect("ERROR: Failed to bind server to port.");
    println!("Server listening on port {}.", config.port);

    // Wait for a client to knock on the port.
    for stream in listener.incoming(){
        let mut stream = stream.unwrap();
        handle_client(&mut stream, &config);
    }
}
