//     RustDuino : A generic HAL implementation for Arduino Boards in Rust
//     Copyright (C) 2021  Nikhil Gupta, Indian Institute of Technology Kanpur
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
use clap::{Arg, App};
use std::thread;
fn main() {
    println!("Welcome to Arduino Serial Monitor");
     let args = App::new("Serialmonitor")
	.version("0.1.0")
	.about("nothing much")
	.author("Teamrustduino(nikhil2121)")
	.arg(
		Arg::with_name("portname")
			.short("l")
			.long("portname")
			.takes_value(true))
    .arg(
		Arg::with_name("baud")
			.short("b")
			.long("baud")
            .takes_value(true),
	).get_matches();
    let name: &str = args.value_of("portname").unwrap_or("COM4");
    let baud: &str = args.value_of("baud").unwrap_or("9600");
    
    let baud1 = baud.trim().parse::<u32>().unwrap();
    let name1=name.clone();
     let mut port = serialport::new(name, baud1)
     .timeout(Duration::from_millis(100))
     .open()
     .expect("Failed to open port");
     let mut port1 = serialport::new(name1, baud1)
        .timeout(Duration::from_millis(100))
        .open()
        .expect("Failed to open port");
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

                let  f1 = OpenOptions::new().write(true).open("send.txt");
    let mut f1 = match f1 {
        Ok(file) => file,
        Err(error) => match error.kind() {
            ErrorKind::NotFound => match File::create("send.txt") {
                Ok(fc) => fc,
                Err(e) => panic!("Problem creating the file: {:?}", e),
            },
            other_error => {
                panic!("Problem opening the file: {:?}", other_error)
            }
        },
    };
 
    thread::spawn(move || {
        loop{
      let mut serial_buf: Vec<u8> = vec![0; 32];
        let _k = port1.read(serial_buf.as_mut_slice());
        let _k = match _k {
            Ok(t) => {
                println!("\nData Recieved:{:?}", serial_buf);
                let _s1:Vec<u8>=time();
                f.write_all(b"Send Data:").expect("Unable to write data");
                f.write_all(&serial_buf).expect("Unable to write data");
                f.write_all(&_s1).expect("Unable to write data");
                t
            }
            Err(_e) => (0),
        };
    }
    });

    loop {
      println!("\nSend Data:");
      f1.write_all(b"Send Data:").expect("Unable to write data");
      let mut arg: String = String::new();
      io::stdin().read_line(&mut arg).expect("input failed");
      let arg1 = arg.trim().as_bytes(); 
      let _exit2: Vec<u8> = vec![101, 120, 105, 116, 13, 10];
      if arg1.eq(&_exit2){
            break;
      }   
      port.write(arg1).expect("Write failed!");
      let _s2:Vec<u8>=time();
      f1.write_all(&arg1).expect("Unable to write data");
      f1.write_all(&_s2).expect("Unable to write data");   
    }
                  
    
}


fn time()->Vec<u8>{
    let now = Local::now();
    let  _hour = now.hour();
    let (_is_common_era, _year) = now.year_ce();
    println!(
        "{:02}:{:02}:{:02} {}-{:02}-{:02} {:?}",
        _hour,
        now.minute(),
        now.second(),
        _year,
        now.month(),
        now.day(),
        now.weekday()
    );
    let hour1:String=_hour.to_string();
    let min:String=now.minute().to_string();
    let sec:String=now.second().to_string();
    let year1:String=_hour.to_string();
    let month:String=now.minute().to_string();
    let day:String=now.second().to_string();
    let week:String=now.weekday().to_string();
let s=hour1 + ":"+&min+":" + &sec + " " + &year1 + "-" +&month + "-" +&day + " " + &week;
 let _s1:Vec<u8>=s.into_bytes();
 _s1
       
}

 
