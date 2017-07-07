use lib::error::{Error, Result};
use lib::util::{calc_distance, list_residue_and_name};
use chemfiles::Frame;

pub fn print_bond_distances(frame: &Frame, args: &[String]) -> Result<()> {
    if args.len() < 2 {
        return Err(Error::InvalidArgument);
    }
    let (name1, name2) = (&args[0], &args[1]);

    let natoms = frame.natoms()?;
    let positions = frame.positions()?;
    let (residue_ids, atom_names) = list_residue_and_name(&frame.topology()?)?;

    println!("id1, id2, distance");
    for i in 1..(natoms as usize) {
        if (atom_names[i] != *name1 && atom_names[i] != *name2)
            || residue_ids[i].is_none() {
            continue;
        }

        for j in 0..i {
            if residue_ids[j].is_none() || residue_ids[i] != residue_ids[j] {
                continue;
            }

            if atom_names[i] == *name1 && atom_names[j] == *name2 {
                println!("{}, {}, {}",
                         i, j,
                         calc_distance(&positions[i], &positions[j]));
            } else if atom_names[i] == *name2 && atom_names[j] == *name1 {
                println!("{}, {}, {}",
                         j, i,
                         calc_distance(&positions[i], &positions[j]));
            }
        }
    }

    Ok(())
}
