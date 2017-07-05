extern crate chemfiles;
mod lib;

use std::env;
use std::path::Path;
use lib::error::{Error, Result};

use chemfiles::{Trajectory, Frame, Selection};

fn list_pairs(frame: &Frame, name1: &str, name2: &str) -> Result<Vec<[u64; 2]>> {
    let query = format!("pairs: name(#1) {} and name(#2) {}", name1, name2);
    let mut selection = Selection::new(query.as_ref())?;

    Ok(selection.evaluate(frame)?
                .iter()
                .map(|pair| [pair[0], pair[1]])
                .collect())
}

fn calc_distance(pos1: &[f64; 3], pos2: &[f64; 3]) -> f64 {
    ((pos2[0] - pos1[0]).powi(2)
     + (pos2[1] - pos1[1]).powi(2)
     + (pos2[2] - pos1[2]).powi(2)).sqrt()
}

fn print_distances(frame: &Frame, args: &[String]) -> Result<()> {
    if args.len() < 2 {
        return Err(Error::InvalidArgument);
    }
    let name1 = &args[0];
    let name2 = &args[1];

    let positions = frame.positions()?;

    println!("id1, id2, distance");
    for pair in list_pairs(frame, name1, name2)? {
        let id1 = pair[0] as usize;
        let id2 = pair[1] as usize;
        println!("{}, {}, {}", id1, id2, calc_distance(&positions[id1], &positions[id2]));
    }

    Ok(())
}

fn print_usage(called_name: &str) {
    let progname = Path::new(called_name)
                        .file_name()
                        .map(|name| name.to_str().unwrap_or(called_name))
                        .unwrap_or(called_name);
    println!("Usage: {} FILE name1 name2", progname);
}

fn main() {
    let args : Vec<String> = env::args().collect();
    if args.len() < 4 {
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
