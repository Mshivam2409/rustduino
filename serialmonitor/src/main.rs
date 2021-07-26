//     RustDuino : A generic HAL implementation for Arduino Boards in Rust
//     Copyright (C) 2021  Nikhil Gupta,Indian Institute of Technology Kanpur
//
//     This program is free software: you can redistribute it and/or modify
//     it under the terms of the GNU Affero General Public License as published
//     by the Free Software Foundation, either version 3 of the License, or
//     (at your option) any later version.
//
//     This program is distributed in the hope that it will be useful,
//     but WITHOUT ANY WARRANTY; without even the implied warranty of
//     MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
//     GNU Affero General Public License for more details.
//
//     You should have received a copy of the GNU Affero General Public License
//     along with this program.  If not, see <https://www.gnu.org/licenses/>
use serialport;
use chrono::{Datelike, Timelike,Local};
use std::fs::File;
use std::fs::OpenOptions;
use std::io;
use std::io::ErrorKind;
use std::io::Write;
use std::time::Duration;
fn main() {
    println!("Welcome to Arduino Serial Monitor\nWrite help to get list of functions available:");

    let _help1: Vec<u8> = vec![104, 101, 108, 112, 13, 10];
    let _ser: Vec<u8> = vec![
        83, 101, 114, 105, 97, 108, 95, 80, 111, 114, 116, 115, 95, 97, 118, 97, 105, 108, 97, 98,
        108, 101, 13, 10,
    ];
    let _port1: Vec<u8> = vec![110, 101, 119, 95, 112, 111, 114, 116, 13, 10];
    let _read1: Vec<u8> = vec![114, 101, 97, 100, 13, 10];
    let _write1: Vec<u8> = vec![119, 114, 105, 116, 101, 13, 10];
    let _exit1: Vec<u8> = vec![101, 120, 105, 116, 13, 10];
    let  f = OpenOptions::new().write(true).open("recieved.txt");
                let mut f = match f {
                    Ok(file) => file,
                    Err(error) => match error.kind() {
                        ErrorKind::NotFound => match File::create("recieved.txt") {
                            Ok(fc) => fc,
                            Err(e) => panic!("Problem creating the file: {:?}", e),
                        },
                        other_error => {
                            panic!("Problem opening the file: {:?}", other_error)
                        }
                    },
                };
    let flag = 0;
    loop {
        println!("Please type the function:");
        let mut arg: String = String::new();
        io::stdin().read_line(&mut arg).expect("input failed");
        let arg1 = arg.into_bytes();

        if flag == 0 {
            if arg1.eq(&_help1) {
                help();
            } else if arg1.eq(&_ser) {
                ports();
            } else if arg1.eq(&_port1) {
                new_port();
            } else if arg1.eq(&_read1) {
                read(&mut f);
            } else if arg1.eq(&_write1) {
            } else if arg1.eq(&_exit1) {
                break;
            } else {
            }
        } /*
                match arg1{
                      help1=>{help();},
                      ser=>{ports_avaialable();},
                      port1=>{println!("port is already chosen")},
                      read=>{
                            let mut str=String::new();
                            read(&mut str);
                      },
                      exit1=>{//std::mem::drop
                            break;
                      },
                      _=>(),
                };
          */
    }
}

fn help() {
    println!("1.Serial_Ports_available\n2.new_port\n3.read\n4.exit");
}
fn ports() {
    let ports = serialport::available_ports().expect("No ports found!");
    for p in ports {
        println!("{}", p.port_name);
    }
}

fn new_port() {
    println!("Please give the name of port:");

    let mut name: String = String::new();
    io::stdin().read_line(&mut name).expect("input failed");
    let name = name.trim();

    println!("Please give the baud rate:");

    let mut baud1: String = String::new();
    io::stdin().read_line(&mut baud1).expect("input failed");
    let baud = baud1.trim().parse::<u32>().unwrap();

    let _serial_port = serialport::new(name, baud)
        .timeout(Duration::from_millis(100))
        .open()
        .expect("Failed to open port");
}
fn read(f:&mut File) {
    println!("Please give the name of port:");
    let mut name: String = String::new();
    io::stdin().read_line(&mut name).expect("input failed");
    let name = name.trim();

    println!("Please give the baud rate:");

    let mut baud1: String = String::new();
    io::stdin().read_line(&mut baud1).expect("input failed");
    let baud = baud1.trim().parse::<u32>().unwrap();

    let mut port = serialport::new(name, baud)
        .timeout(Duration::from_millis(100))
        .open()
        .expect("Failed to open port");


    loop {
      println!("Send Data:");
      f.write_all(b"Send Data:").expect("Unable to write data");
      let mut arg: String = String::new();
      io::stdin().read_line(&mut arg).expect("input failed");
      let arg1 = arg.trim().as_bytes(); 
      let _exit2: Vec<u8> = vec![101, 120, 105, 116, 13, 10];
      if arg1.eq(&_exit2){
            break;
      }   
      port.write(arg1).expect("Write failed!");
      time();
      f.write_all(&arg1).expect("Unable to write data");

        let mut serial_buf: Vec<u8> = vec![0; 32];
        let _k = port.read(serial_buf.as_mut_slice());
        let _k = match _k {
            Ok(t) => {
                println!("Data Recieved:{:?}", serial_buf);
                time();
                f.write_all(b"Send Data:").expect("Unable to write data");
                f.write_all(&serial_buf).expect("Unable to write data");
                t
            }
            Err(_e) => (0),
        };
    }
}


fn time(){
    let now = Local::now();
    let (_is_pm, _hour) = now.hour12();
    let (_is_common_era, _year) = now.year_ce();
    println!(
        "{:02}:{:02}:{:02} {} {}-{:02}-{:02} {:?}",
        _hour,
        now.minute(),
        now.second(),
        if _is_pm { "PM" } else { "AM" },
        _year,
        now.month(),
        now.day(),
        now.weekday()
    );
       
}