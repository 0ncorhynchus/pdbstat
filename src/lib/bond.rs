use lib::error::{Error, Result};
use lib::util::calc_distance;
use chemfiles::Frame;

pub fn print_bond_distances(frame: &Frame, args: &[String]) -> Result<()> {
    if args.len() < 2 {
        return Err(Error::InvalidArgument);
    }
    let (name1, name2) = (&args[0], &args[1]);

    let natoms = frame.natoms()?;
    let (residue_ids, atom_names) = {
        let topology = frame.topology()?;
        let max_id = topology.residues_count()?;
        let mut ids = vec![0; natoms as usize];
        let mut names = vec![String::new(); natoms as usize];

        for i in 0..natoms {
            ids[i as usize] = match topology.residue_for_atom(i)? {
                Some(residue) => residue.id()?,
                None          => max_id,
            };
            names[i as usize] = topology.atom(i)?.name()?;
        }

        (ids, names)
    };

    let positions = frame.positions()?;

    println!("id1, id2, distance");
    for i in 1..(natoms as usize) {
        if atom_names[i] != *name1 && atom_names[i] != *name2 {
            continue;
        }

        for j in 0..i {
            if residue_ids[i] != residue_ids[j] {
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
