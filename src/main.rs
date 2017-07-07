extern crate chemfiles;
mod lib;

use std::env;
use std::path::Path;
use lib::error::{Error, Result};
use lib::distance::print_distances;
use lib::bond::print_bond_distances;

use chemfiles::{Trajectory, Frame};

fn print_usage(called_name: &str) {
    let progname = Path::new(called_name)
                        .file_name()
                        .map(|name| name.to_str().unwrap_or(called_name))
                        .unwrap_or(called_name);
    println!("Usage: {} FILE COMMAND [args ...]", progname);
}

fn call_command(frame: &Frame, command: &str, args: &[String]) -> Result<()> {
    match command {
        "bond"     => print_bond_distances(frame, args),
        "distance" => print_distances(frame, args),
        _          => Err(Error::InvalidArgument)
    }
}

fn main() {
    let args : Vec<String> = env::args().collect();
    if args.len() < 3 {
        print_usage(&args[0]);
        return;
    }

    let filename = &args[1];
    let command = &args[2];

    let mut trajectory = Trajectory::open(filename, 'r').unwrap();
    let mut frame = Frame::new().unwrap();

    trajectory.read(&mut frame).unwrap();

    match call_command(&frame, command, &args[3..]) {
        Ok(_) => {},
        Err(e) => match e {
            Error::InvalidArgument => print_usage(&args[0]),
            _ => println!("{:?}", e)
        }
    }
}
