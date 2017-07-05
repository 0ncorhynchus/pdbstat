extern crate chemfiles;
mod lib;

use std::env;
use std::path::Path;
use lib::error::Error;
use lib::distance::print_distances;

use chemfiles::{Trajectory, Frame};

fn print_usage(called_name: &str) {
    let progname = Path::new(called_name)
                        .file_name()
                        .map(|name| name.to_str().unwrap_or(called_name))
                        .unwrap_or(called_name);
    println!("Usage: {} FILE name1 name2", progname);
}

fn main() {
    let args : Vec<String> = env::args().collect();
    if args.len() < 2 {
        print_usage(&args[0]);
        return;
    }

    let filename = &args[1];

    let mut trajectory = Trajectory::open(filename, 'r').unwrap();
    let mut frame = Frame::new().unwrap();

    trajectory.read(&mut frame).unwrap();

    match print_distances(&frame, &args[2..]) {
        Ok(_) => {},
        Err(e) => match e {
            Error::InvalidArgument => print_usage(&args[0]),
            _ => println!("{:?}", e)
        }
    }
}
